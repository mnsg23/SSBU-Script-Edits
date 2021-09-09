#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod eflame;
mod elight;

#[skyline::main(name = "element_attack100start_smashline")]
pub fn main() {
	eflame::install();
	elight::install();
}
