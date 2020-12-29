use std::collections::HashMap;

#[derive(Debug)]
pub enum SpreadsheetError{
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    MutexError,
    IndexError,
    NotNumberError,
    ExitRequest
}
#[derive(Debug)]
enum SpreadsheetResult<T>{
    Error(SpreadsheetError),
    Result(T)
}
#[derive(Debug,Clone)]
pub enum Cell{
    Text(String),
    Number(f64),
    Formula(FormulaCell),
    Empty
}
#[derive(Debug,Clone)]
pub struct FormulaCell{
    cell_ref: Box<Option<Vec<Cell>>>,
    formula: String
}

pub trait CellADT: Sized{
    fn cell_text(&self) -> String;
    fn full_text(&self) -> String;
    fn value(&self)-> Option<f64>;
}
impl CellADT for Cell{
    fn cell_text(&self) -> String {
        unimplemented!()
    }

    fn full_text(&self) -> String {
        unimplemented!()
    }

    fn value(&self) -> Option<f64> {
        unimplemented!()
    }
}

