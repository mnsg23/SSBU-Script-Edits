#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod diddy;

#[skyline::main(name = "sound_diddy_smashline")]
pub fn main() {
	diddy::install();
}
