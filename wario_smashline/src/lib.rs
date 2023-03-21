#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]

mod wario;

#[skyline::main(name = "wario_smashline")]
pub fn main() {
	wario::install();
}
