#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod koopa;

#[skyline::main(name = "sound_koopa_smashline")]
pub fn main() {
	koopa::install();
}
