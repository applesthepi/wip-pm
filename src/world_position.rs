use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

const IMPL_TRAIT: &[&str] = &[
	"std::ops::Add",
	"std::ops::Sub",
	"std::ops::Mul",
	"std::ops::Div",
];

const IMPL_FN: &[&str] = &[
/* ADD */"
type Output = Self;
fn add(self, rhs: Self) -> Self::Output {
	Self::new(
		self.x + rhs.x,
		self.y + rhs.y,
	)
}
",/* SUB */"
type Output = Self;
fn sub(self, rhs: Self) -> Self::Output {
	Self::new(
		self.x - rhs.x,
		self.y - rhs.y,
	)
}
",/* MUL */"
type Output = Self;
fn mul(self, rhs: Self) -> Self::Output {
	Self::new(
		self.x * rhs.x,
		self.y * rhs.y,
	)
}
",/* DIV */"
type Output = Self;
fn div(self, rhs: Self) -> Self::Output {
	Self::new(
		self.x / rhs.x,
		self.y / rhs.y,
	)
}
"];

pub fn derive(
	input: TokenStream,
) -> TokenStream {
	assert!(IMPL_TRAIT.len() == IMPL_FN.len());
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;

	let mut stream = String::new();
	for i in 0..IMPL_TRAIT.len() {
		let impl_trait = &IMPL_TRAIT[i];
		let impl_fn = &IMPL_FN[i];
		let impl_block = format!("
			impl {} for {} {{
			{}
			}}
		", impl_trait, struct_name.to_string(), impl_fn);
		stream.push_str(&impl_block);
	}
	stream.parse().unwrap()
}