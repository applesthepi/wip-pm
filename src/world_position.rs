use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

const IMPL_TRAIT: &[&str] = &[
	"std::cmp::PartialEq",
	"std::cmp::PartialOrd",
	"std::ops::Add",
	"std::ops::Sub",
	"std::ops::Mul",
	"std::ops::Div",
];

const IMPL_FN: &[&str] = &[
/* PARTIAL_EQ */"
fn eq(&self, rhs: &Self) -> bool {
	self.x == rhs.x &&
	self.y == rhs.y
} fn ne(&self, rhs: &Self) -> bool {
	!self.eq(rhs)
}", /* PARTIAL_ORD */"
fn ge(&self, rhs: &Self) -> bool {
	self.x >= rhs.x &&
	self.y >= rhs.y
} fn le(&self, rhs: &Self) -> bool {
	self.x <= rhs.x &&
	self.y <= rhs.y
} fn gt(&self, rhs: &Self) -> bool {
	self.x > rhs.x &&
	self.y > rhs.y
} fn lt(&self, rhs: &Self) -> bool {
	self.x < rhs.x &&
	self.y < rhs.y
} fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
	if self.lt(rhs) { Some(std::cmp::Ordering::Less) } else
	if self.gt(rhs) { Some(std::cmp::Ordering::Greater) } else
	{ Some(std::cmp::Ordering::Equal) }
}
",/* ADD */"
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