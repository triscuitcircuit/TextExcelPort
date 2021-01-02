/*
Copyright: Tristan Zippert
University of Maine Computer Science
Friday, Nov 29 2019
*/

mod lib;
use textexcelport_core::grid::*;
use std::{io, env};

extern crate getopts;
use getopts::Options;
use std::rc::Rc;
use std::cell::RefCell;
use cli::TextConsole;

fn main(){
  //  let console = Rc::from(RefCell::from(TextConsole::from_stdio()));
    //let exit_code = Rc::from(RefCell::from(None));

    // let mut machine = {
    //
    // };
    text_version()
}

fn version() -> Result<i32,()>{
    println!("TextExcelPort {}", env!("CARGO_PKG_VERSION"));
    println!("Copyright 2019 Tristan Zippert");
    Ok(0)
}
