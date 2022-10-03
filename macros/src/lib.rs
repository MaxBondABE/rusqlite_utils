use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod util;
use util::impl_try_from_row;

#[proc_macro_derive(TryFromRow)]
pub fn try_from_row(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let impl_block = impl_try_from_row(ident, data);

    impl_block.into()
}
