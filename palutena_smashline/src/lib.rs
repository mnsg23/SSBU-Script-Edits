#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod palutena;

#[skyline::main(name = "palutena_smashline")]
pub fn main() {
	palutena::install();
}
