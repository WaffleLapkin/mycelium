use super::atomic::{spin_loop_hint, AtomicBool, Ordering};
use crate::cell::CausalCell;
use core::{fmt, ops};

/// A simple spinlock ensuring mutual exclusion.
#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicBool,
    data: CausalCell<T>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: CausalCell::new(data),
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.locked.compare_and_swap(false, true, Ordering::Acquire) == false {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self.locked.compare_and_swap(false, true, Ordering::Acquire) != false {
            while self.locked.load(Ordering::Relaxed) {
                spin_loop_hint();
            }
        }
        MutexGuard { mutex: self }
    }
}

// === impl MutexGuard ===

impl<'a, T> ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.mutex.data.with(|ptr| unsafe { &*ptr })
    }
}

impl<'a, T> ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutex.data.with_mut(|ptr| unsafe { &mut *ptr })
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use loom::thread;
    use std::prelude::v1::*;
    use std::sync::Arc;

    use super::*;

    #[test]
    fn multithreaded() {
        loom::model(|| {
            let mutex = Arc::new(Mutex::new(String::new()));
            let mutex2 = mutex.clone();

            let t1 = thread::spawn(move || {
                println!("t1: locking...");
                let mut lock = mutex2.lock();
                println!("t1: locked");
                lock.push_str("bbbbb");
                println!("t1: dropping...");
            });

            {
                println!("t2: locking...");
                let mut lock = mutex.lock();
                println!("t2: locked");
                lock.push_str("bbbbb");
                println!("t2: dropping...");
            }
            t1.join().unwrap();
        });
    }

    #[test]
    fn try_lock() {
        loom::model(|| {
            let mutex = Mutex::new(42);
            // First lock succeeds
            let a = mutex.try_lock();
            assert_eq!(a.as_ref().map(|r| **r), Some(42));

            // Additional lock failes
            let b = mutex.try_lock();
            assert!(b.is_none());

            // After dropping lock, it succeeds again
            ::core::mem::drop(a);
            let c = mutex.try_lock();
            assert_eq!(c.as_ref().map(|r| **r), Some(42));
        });
    }
}