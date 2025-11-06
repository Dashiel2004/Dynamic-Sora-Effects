#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
mod sora;


#[skyline::main(name = "sora_dynamic_effects")]
pub fn main() {
    sora::install();
}