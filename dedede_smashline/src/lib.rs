#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod dedede;

#[skyline::main(name = "dedede_smashline")]
pub fn main() {
	dedede::install();
}
