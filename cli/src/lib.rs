use async_trait::async_trait;
use crossterm::{cursor,event,execute,style,terminal, tty::IsTty,QueueableCommand};
use textexcelport_core::console;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;

fn crossterm_e_ioerror(e: crossterm::ErrorKind)-> io::Error{


}
pub struct TextConsole{
    is_tty: bool,
    buffer: VecDeque<console::Key>
}