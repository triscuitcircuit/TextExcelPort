use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::rc::Rc;

#[derive(Debug)]
pub enum SpreadsheetError{
    /// Error encountered when theres trouble parsing an int
    ParseIntError(std::num::ParseIntError),
    /// Error encountered when theres trouble parsing a floating point number
    ParseFloatError(std::num::ParseFloatError),
    /// Error encountered when theres a Mutex lock error
    MutexError,
    ///Error encountered when theres an index out of range
    IndexError,
    /// Error encountered when a String is entered instead of a number
    NotNumberError,
    /// Encounters when a exit is requested
    ExitRequested
}
