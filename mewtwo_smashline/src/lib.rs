#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod mewtwo;

#[skyline::main(name = "mewtwo_smashline")]
pub fn main() {
	mewtwo::install();
}
