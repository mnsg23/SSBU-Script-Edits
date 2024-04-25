#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod lucario;

#[skyline::main(name = "lucario_smashline")]
pub fn main() {
	lucario::install();
}
