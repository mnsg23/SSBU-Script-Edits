#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod rosetta;

#[skyline::main(name = "sound_rosetta_smashline")]
pub fn main() {
	rosetta::install();
}
