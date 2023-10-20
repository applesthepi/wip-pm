use proc_macro::TokenStream;
use syn::parse::Parse;

mod world_position;

// fn parse_str<T: Parse>(
// 	string: &str,
// ) -> T {
// 	syn::parse(string.parse::<TokenStream>().unwrap()).unwrap()
// }

#[proc_macro_derive(WorldPosition)]
pub fn derive_world_position(
	input: TokenStream,
) -> TokenStream {
	world_position::derive(input)
}