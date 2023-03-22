#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod ike;

#[skyline::main(name = "ike_smashline")]
pub fn main() {
	ike::install();
}
