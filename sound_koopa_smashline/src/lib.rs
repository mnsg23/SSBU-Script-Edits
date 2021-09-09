#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod koopa;

#[skyline::main(name = "sound_koopa_smashline")]
pub fn main() {
	koopa::install();
}
