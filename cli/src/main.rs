/*
Copyright: Tristan Zippert
University of Maine Computer Science
Friday, Nov 29 2019
*/

mod lib;
use textexcelport_core::grid::*;
use std::io;

extern crate getopts;
use getopts::Options;

fn main(){
    text_version()
}

fn version() -> Result<i32,()>{
    println!("TextExcelPort {}", env!("CARGO_PKG_VERSION"));
    println!("Copyright 2019 Tristan Zippert");
    Ok((0))
}