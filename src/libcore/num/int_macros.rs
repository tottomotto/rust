#![doc(hidden)]

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! int_module {
    ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
    ($T:ident, #[$attr:meta]) => (
        doc_comment! {
            concat!("The smallest value that can be represented by this integer type.
Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead."),
            #[$attr]
            pub const MIN: $T = $T::MIN;
        }

        doc_comment! {
            concat!("The largest value that can be represented by this integer type.
Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead."),
            #[$attr]
            pub const MAX: $T = $T::MAX;
        }
    )
}
