use yard::{parser, evaluator};
use serde::{Serialize, Deserialize};
use serde_json;
use lazy_static::lazy_static;


use std::{
  fs,fs::{File}, borrow::Borrow, path::Path,
  io,env,sync::{Arc, Mutex}, num::ParseIntError,
  collections::HashMap,rc::Rc, cell::RefCell,
  io::{ Write, BufReader, Empty,Read, stdout}
};
use std::default::default;
use crate::exec::*;


pub type Spreadcell = Arc<RefCell<Cell>>;

pub struct WorkBook{
    sheets: Vec<Worksheet>
}

pub struct Worksheet{
    cells: Vec<Spreadcell>
}


lazy_static!{
    pub static ref GRID: Mutex<Vec<Vec<Cell>>> = Mutex::new(vec![vec![Cell::Empty; 17]; 17]);
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
unsafe impl Send for Cell{}
#[derive(Debug,Clone)]
pub struct FormulaCell{
    cell_ref: Option<Rc<Vec<RefCell<Cell>>>>,
    command: String
}
impl FormulaCell{
    pub fn new(input: String)->Self{
        self::FormulaCell{
            cell_ref: default(),
            command:input,
        }
    }
    pub fn string_to_f64(&self)-> Result<f64,SpreadsheetError>{
        let input = &self.command[1..self.command.len()-1];
        let mut input_arr:Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
        for i in 0..input_arr.len(){
            if input_arr[i].len() == 2 as usize && &input_arr[i].to_uppercase().as_bytes()[0] >= &65 {
                let row: u8 = input_arr[i].to_uppercase().as_bytes()[0] - 65;
                let col: u8 = match input_arr[i].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                    Ok(output) => output,
                    Err(_e) => 0,
                };
                {
                    let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                    let spreadsheet = db.clone();
                    std::mem::drop(db);
                    input_arr[i] = match spreadsheet[row as usize][(col - 1) as usize].get_value(){
                        Some(num)=> num.to_string(),
                        None=> "0".to_string(),

                    };
                }
            }
        }
        return match input_arr[0].to_uppercase().as_ref() {
            "AVG" => {
                if input_arr.len() >= 2 {
                    let input_loc: Vec<String> = input_arr[1]
                        .split("-").map(|x| x.to_string()).collect();
                    let (start_row, start_col): (u8, u8) = (input_loc[0].to_uppercase().as_bytes()[0] - 65,
                                                            match input_loc[1].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                                                Ok(output) => output,
                                                                Err(_e) => 0,
                                                            });
                    let (end_row, end_col): (u8, u8) = (input_loc[1].to_uppercase().as_bytes()[0] - 65,
                                                        match input_loc[1].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                                            Ok(output) => output,
                                                            Err(_e) => 0,
                                                        });
                    let mut var: f64 = 0.0;
                    let mut times = 0.0;
                    for c in start_col..=end_col {
                        for r in start_row..=end_row {
                            times += 1.0;
                            {
                                let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                                let spreadsheet = db.clone();
                                std::mem::drop(db);
                                var += match spreadsheet[c as usize][r as usize].get_value() {
                                    Some(t) => t,
                                    None => 0.0,
                                } as f64;
                            }
                        }
                    }
                    println!("{}", var / times);
                    return Ok(var / times)
                }

                Ok(0.0)
            }
            "SUM" => {
                if input_arr.len() >= 2 {
                    let input_loc: Vec<String> = input_arr[1]
                        .split("-").map(|x| x.to_string()).collect();
                    let (start_row, start_col): (u8, u8) = (input_loc[0].to_uppercase().as_bytes()[0] - 65,
                                                            match input_loc[1].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                                                Ok(output) => output,
                                                                Err(_e) => 0,
                                                            });
                    let (end_row, end_col): (u8, u8) = (input_loc[1].to_uppercase().as_bytes()[0] - 65,
                                                        match input_loc[1].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                                            Ok(output) => output,
                                                            Err(_e) => 0,
                                                        });
                    let mut var: f64 = 0.0;
                    for c in start_col..=end_col {
                        for r in start_row..=end_row {
                            {
                                let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                                let spreadsheet = db.clone();
                                std::mem::drop(db);
                                var += match spreadsheet[c as usize][r as usize].get_value() {
                                    Some(t) => t,
                                    None => 0.0,
                                } as f64;
                            }
                        }
                    }
                    println!("{}", var);
                    return Ok(var)
                }
                Ok(0.0)
            }
            _ => {
                if let Ok(tokens) = parser::parse(&input_arr.join(" ")) {
                    let result = evaluator::eval(&tokens);
                    return Ok(result as f64);
                }
                Ok(0.0)
            }
        }
    }
    pub fn get_text(&self) -> String{ (&self.command).parse().unwrap() }
}
pub trait CellADT: Sized{

    fn cell_text(&self) -> String;

    fn full_text(&self) -> String;

    fn base_text(&self) -> String;

    fn get_value(&self) -> Option<f64>;
}
impl CellADT for Cell{
    fn cell_text(&self) -> String {
        match self {
            Cell::Text(string) =>{cell_text_spaces(string.to_string().borrow())},
            Cell::Number(number) =>{cell_text_spaces(number.to_string().borrow())},
            Cell::Formula(_form) => {cell_text_spaces(&self.get_value().unwrap_or(0.0).to_string())},
            Cell::Empty => " ".repeat(10).to_string(),
        }
    }

    fn full_text(&self) -> String {
        match self {
            Cell::Text(string) => format!("\"{}\"", string.clone()),
            Cell::Number(number) => number.to_string(),
            Cell::Formula(string)=> string.get_text(),
            Cell::Empty => String::from(" "),
        }
    }
    fn base_text(&self)-> String{
        match self {
            Cell::Text(string) =>format!("\"{}\"", string.clone()),
            Cell::Number(number) =>number.to_string(),
            Cell::Formula(_formula_cell) => {self.get_value().unwrap_or(0.0).to_string().to_string()},
            Cell::Empty => " ".to_string(),
        }
    }
    fn get_value(&self) -> Option<f64> {
        match self {
            Cell::Number(number) => Some(*number),
            Cell::Formula(formula) => Some(match formula.string_to_f64(){
                Ok(out) => out,
                Err(_e) => 0.0,
            }),
            Cell::Text(_) => None,
            Cell::Empty => None,
        }
    }
}
fn cell_text_spaces(string: &String) -> String {
    let mut spaces = String::new();
    for _i in 0..10 - (string.len() as i32) {
        spaces.push(' ');
    }

    if string.len() > 10 {
        string[0..10].parse().unwrap()
    } else {
        format!("{}{}", &string, spaces)
    }

}
pub fn process_command(input:String) -> Result<String,SpreadsheetError>{
    let input: Vec<&str> = input.splitn(3,' ').collect();
    match input[0].to_uppercase().as_ref() {
        "SAVE"=>{
            Ok(String::from("Saved"))
        }
        "SORTA"=>{
            Ok(String::from("Command SORTA entered"))
        }
        "PRINT"|"SPREADSHEET"|"SPREAD"=>{
            Ok(get_grid_text().expect(""))
        }
        "CLEAR"=> {
            println!("{}",input.len());
            if input.len() == 2 {
                let row = (input[1].to_uppercase().as_bytes()[0] - 65) as usize;
                let col_str = input[0].trim_end_matches(|c: char| !c.is_ascii_digit());
                let col = match col_str.parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                };
            } else {
                {
                    let mut db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                    let row = db.len();
                    let col = db[0].len();
                    for r in 0..row {
                        for c in 0..col {
                            db[r][c] = Cell::Empty;
                        }
                    }
                }
            }
            return Ok(get_grid_text().expect(""));
        }
        "SORTD"=>{
            Ok(String::from("Command SORTD entered"))
        }
        "QUIT"|"STOP"=> {
            println!("Quitting...");
            Err(SpreadsheetError::ExitRequested)
        }
        _ =>{
            {
                {
                    let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                    let row: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                    let col: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                        Ok(output) => output - 1,
                        Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                    };
                }
                if input.len() >= 3 {
                    let mut input_two = input[2];
                    if &input_two[0..1] == "(" && &input_two[input_two.len() - 1..input_two.len()] == ")" {
                        //lock is called on db grid here, causing error when lock is called by FormulaCell creation. TODO: Sanitize with values here
                        let form_val = Cell::Formula(FormulaCell::new(input_two.parse().unwrap()));
                        let mut db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                        let row: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                        let col: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                            Ok(output) => output - 1,
                            Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                        };
                        db[row as usize][col as usize] = form_val.clone();
                    } else if &input_two[0..1] == "\"" && &input_two[input_two.len() - 1..input_two.len()] == "\"" {
                        {
                            let mut db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                            let row: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                            let col: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                Ok(output) => output - 1,
                                Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                            };
                            db[row as usize][col as usize] = Cell::Text(input_two[1..input_two.len() - 1].to_string());
                        }
                    } else {
                        {
                            let mut db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                            let row: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                            let col: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                                Ok(output) => output - 1,
                                Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                            };
                            db[row as usize][col as usize] = Cell::Number(match input_two.parse::<f64>() {
                                Ok(num) => num,
                                Err(e) => return Err(SpreadsheetError::ParseFloatError(e))
                            });
                        }
                    }
                } else {
                    {
                        let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
                        let row: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                        let col: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                            Ok(output) => output - 1,
                            Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                        };
                        return Ok("cell value: ".to_owned() + &db[row as usize][col as usize].full_text());
                    }
                }
            }
            return Ok(get_grid_text().expect(""));
        }
    }
}
pub fn get_grid_text() -> Result<String,SpreadsheetError> {
    let db = GRID.lock().map_err(|_| SpreadsheetError::MutexError)?;
    let row: u8 = db.len() as u8;
    let col: u8 = db[0].len() as u8;
    let start: u8 = 65;
    let mut top = String::from("   |");
    let mut bottom = String::new();
    let grid = db.clone();
    std::mem::drop(db);
    for i in 0..col {
        top.push_str(&format!("{}{}|", (start + i) as char, " ".repeat(9)));
    }
    for i in 0..row {
        if i >= 9 {
            bottom.push_str(&format!("{} |", i + 1));
        } else {
            bottom.push_str(&format!("{}  |", i + 1));
        }
        for a in 0..col {

            bottom.push_str(&format!("{}|", grid[a as usize][i as usize].cell_text()));
        }
        bottom.push('\n');
    }
    Ok(format!("{}\n{}", top, bottom))

}
pub fn text_version(){
    loop {
        let mut readin = String::new();
        io::stdin().read_line(&mut readin).unwrap();
        let mut iterate = readin.lines();
        let input = iterate.next().unwrap();
        if 2 > input.len() {
            println!("Please enter a proper command");
        } else {
            match input.to_uppercase().as_ref() {
                "LOAD" => {
                    println!("{}\n{:#?}", "Loading...", get_grid_text())
                },
                _ => {
                    match process_command(String::from(input)) {
                        Ok(output) => {
                            print!("\x1B[2J");
                            println!("{}", output);
                        },
                        Err(SpreadsheetError::ExitRequested) => std::process::exit(1),
                        Err(SpreadsheetError::IndexError) => println!("Index error, please try again"),
                        Err(SpreadsheetError::MutexError) => println!("Try again"),
                        Err(SpreadsheetError::ParseIntError(_e)) => println!("Try again"),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
            }
        }
    }
}
