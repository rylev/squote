/// Formatting macro for constructing `Ident`s.
///
/// <br>
///
/// # Syntax
///
/// Syntax is copied from the [`format!`] macro, supporting both positional and
/// named arguments.
///
/// Only a limited set of formatting traits are supported. The current mapping
/// of format types to traits is:
///
/// * `{}` ⇒ [`IdentFragment`]
/// * `{:o}` ⇒ [`Octal`](`std::fmt::Octal`)
/// * `{:x}` ⇒ [`LowerHex`](`std::fmt::LowerHex`)
/// * `{:X}` ⇒ [`UpperHex`](`std::fmt::UpperHex`)
/// * `{:b}` ⇒ [`Binary`](`std::fmt::Binary`)
///
/// See [`std::fmt`] for more information.
#[macro_export]
macro_rules! format_ident {
    ($($fmt:tt)*) => {
        $crate::Ident::new(format!($($fmt)*))
    };
}
