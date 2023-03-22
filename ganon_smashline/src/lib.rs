#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod ganon;

#[skyline::main(name = "ganon_smashline")]
pub fn main() {
	ganon::install();
}
