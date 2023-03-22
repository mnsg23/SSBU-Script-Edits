#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod krool;

#[skyline::main(name = "krool_smashline")]
pub fn main() {
	krool::install();
}
