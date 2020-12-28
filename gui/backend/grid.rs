use std::collections::HashMap;

pub trait CellADT: Sized{
    fn cell_text(&self) -> String;
    fn full_text(&self) -> String;
    fn value(&self)-> Option<f64>;
}

#[derive(Debug)]
enum SpreadsheetError{
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    MutexError,
    IndexError,
    NotNumberError,
    ExitRequest
}

#[derive(Debug,Clone)]
enum Cell{
    Text(String),
    Number(f64),
    Formula(FormulaCell),
    Empty
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

#[derive(Debug,Clone)]
struct FormulaCell{
    cell_ref: Box<Option<Vec<Cell>>>,
    formula: String
}
