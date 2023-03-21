#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod diddy;

#[skyline::main(name = "sound_diddy_smashline")]
pub fn main() {
	diddy::install();
}
