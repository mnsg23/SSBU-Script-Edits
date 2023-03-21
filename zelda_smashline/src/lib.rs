#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod zelda;

#[skyline::main(name = "zelda_smashline")]
pub fn main() {
	zelda::install();
}
