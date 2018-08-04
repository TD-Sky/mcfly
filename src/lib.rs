#[macro_use]
extern crate clap;
extern crate rusqlite;
extern crate termion;
extern crate unicode_segmentation;
extern crate core;
extern crate libc;

pub mod history;
pub mod settings;
pub mod bash_history;
pub mod interface;
pub mod fake_typer;
