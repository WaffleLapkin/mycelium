use crate::{
    bitfield,
    example::{AnotherTestEnum, TestEnum},
};

bitfield! {
    /// An example bitfield type.
    ///
    /// This type was generated by the following [`bitfield!`]
    /// macro invocation:
    /// ```
    #[doc = include_str!("example_bitfield.rs")]
    /// ```
    #[derive(PartialEq, Eq, Hash)]
    pub struct ExampleBitfield<u32> {
        /// Six bits of arbitrary meaning.
        pub const SOME_BITS = 6;

        /// A bit flag.
        ///
        /// This is `true` if foo is enabled. What that means is left
        /// as an exercise to the reader.
        pub const FOO_ENABLED: bool;

        /// Another bit flag.
        ///
        /// This is `true` if bar is enabled. What that means is left
        /// as an exercise to the reader.
        pub const BAR_ENABLED: bool;

        /// These bits are reserved and should always be 0.
        const _RESERVED_1 = 2;

        /// An enum value
        pub const TEST_ENUM: TestEnum;

        const _RESERVED_BITS = 4;

        /// Another enum.
        pub const ANOTHER_ENUM: AnotherTestEnum;

        /// An 8-bit signed integer value.
        ///
        /// Who knows what this means.
        pub const A_BYTE: i8;
    }
}