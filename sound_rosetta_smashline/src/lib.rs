#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod rosetta;

#[skyline::main(name = "sound_rosetta_smashline")]
pub fn main() {
	rosetta::install();
}
