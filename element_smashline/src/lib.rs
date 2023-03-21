#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod eflame;
mod elight;

#[skyline::main(name = "element_smashline")]
pub fn main() {
	eflame::install();
	elight::install();
}
