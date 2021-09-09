#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod donkey;

#[skyline::main(name = "sound_donkey_smashline")]
pub fn main() {
	donkey::install();
}
