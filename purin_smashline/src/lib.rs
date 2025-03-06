#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod purin;

#[skyline::main(name = "purin_smashline")]
pub fn main() {
	purin::install();
}
