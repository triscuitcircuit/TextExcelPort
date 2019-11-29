extern crate lazy_static;

use std::io;
use std::borrow::Borrow;
use std::fs;
use std::io::{BufReader, Empty, Read};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use core::borrow::BorrowMut;
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::rc::Rc;
use serde_json::value::Value::Array;
use std::sync::mpsc::{channel, Sender};

lazy_static!{
    static ref GRID: Mutex<Vec<Vec<Cell>>> = Mutex::new(vec![vec![Cell::Empty; 10]; 10])
}

fn main() {
    let game: bool = true; 
    let mut spread: Spreadsheet = Spreadsheet::new();
    while game {
        let mut readin = String::new();
        io::stdin().read_line(&mut readin).unwrap();
        let mut iterate = readin.lines();
        let input = iterate.next().unwrap();
        if 2 > input.len() {
            println!("please enter an actual command");
        } else {
            match input.to_uppercase().as_ref(){
                "LOAD"=> {
                    let json_file = File::open("/tmp/save.json").expect("file not found");
                    spread= serde_json::from_reader(json_file).expect("error while reading json");
                    println!("{}\n{}","Loading...", spread.get_grid_text());
                },
                _=>{
                    match spread.process_command(String::from(input)) {
                        Ok(output) => println!("{}", output),
                        Err(SpreadsheetError::ExitRequested) => std::process::exit(1),
                        Err(SpreadsheetError::IndexError) => println!("index error, please try again"),
                        Err(e) => eprintln!("{:#?}",e),
                    }
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FormulaCell {
    command: String,
    grid:  Arc<Mutex<Vec<Vec<Cell>>>>,

}
impl FormulaCell{
    pub fn new(input: String,input2: &Spreadsheet)-> FormulaCell {
        self::FormulaCell {
            command: input,
            grid: Arc::clone(&input2.sheet),
        }
    }
    pub fn update(&mut self, input: &mut Spreadsheet){
        self.grid =  Arc::clone(&input.sheet)
    }
    pub fn string_to_f64(&self)->f64 {
        let input = &self.command[1..self.command.len()-1];
        let mut input_arr:Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
        for i in 0..input_arr.len(){
            if input_arr[0].len()==2 as usize && &input_arr[0].to_uppercase().as_bytes()[0] >= &65{
                let row:u8 = input_arr[0].to_uppercase().as_bytes()[0]-65;
                let col: u8 = match input_arr[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                    Ok(output) => output,
                    Err(_e) => 0,
                };
                let row_num = row as usize;
                let column_num = (col - 1) as usize;
                let arr = self.grid.lock().unwrap();
                input_arr[i] = arr[row_num][column_num].base_text();
            }
        }
        match input_arr[0].to_uppercase().as_ref() {
            "AVG" => {
                let mut var = 0.0;
                let mut times = 0;
                //let input_arr:Vec<String> = input.split('-');
                var

            },
            "SUM" =>{
                7.6
            },
            _=> {
                //shunting yard needed
                let mut poor:f64 = input_arr[0].parse().unwrap_or(0.0);
                for i in  0..input_arr.len(){
                    if input_arr[i]=="+"{
                        poor += input_arr[i+1].parse::<f64>().unwrap_or(0.0);
                    }
                    if input_arr[i]=="-"{
                        poor -= input_arr[i+1].parse::<f64>().unwrap_or(0.0);
                    }
                    if input_arr[i]=="*"{
                        poor *= input_arr[i+1].parse::<f64>().unwrap_or(0.0);
                    }
                    if input_arr[i]=="/"{
                        poor /= input_arr[i+1].parse::<f64>().unwrap_or(0.0);
                    }
                }
                poor
            }
        }
    }
    pub fn get_text(&self) -> String{ (&self.command).parse().unwrap() }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Cell {
    Text(String),
    Number(f64),
    Formula(FormulaCell),
    Empty,
}
impl Cell{
    pub fn cell_text(&self) -> String {
        match self {
            Cell::Text(string) =>{cell_text_spaces(string.to_string().borrow())},
            Cell::Number(number) =>{cell_text_spaces(number.to_string().borrow())},
            Cell::Formula(_formula_cell) => {cell_text_spaces(&self.get_value().unwrap_or(0.0).to_string())},
            Cell::Empty => " ".repeat(10).to_string(),
        }
    }

    pub fn full_text(&self) -> String {
        match self {
            Cell::Text(string) => format!("\"{}\"", string.clone()),
            Cell::Number(number) => number.to_string(),
            Cell::Formula(string)=> string.get_text(),
            Cell::Empty => String::from(" "),
        }
    }
    pub fn base_text(&self)-> String{
        match self {
            Cell::Text(string) =>format!("\"{}\"", string.clone()),
            Cell::Number(number) =>number.to_string(),
            Cell::Formula(_formula_cell) => {self.get_value().unwrap_or(0.0).to_string().to_string()},
            Cell::Empty => " ".to_string(),
        }
    }

    pub fn get_value(&self) -> Option<f64> {
        match self {
            Cell::Number(number) => Some(*number),
            Cell::Formula(formula) => Some(formula.string_to_f64()),
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
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Spreadsheet{
    pub sheet: Arc<Mutex<Vec<Vec<Cell>>>>,
}
#[derive(Debug, Clone)]
enum SpreadsheetError {
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    IndexError,
    NotNumberError,
    ExitRequested,
}
impl Spreadsheet {
    pub fn get_grid_text(&mut self) -> String {
        let (tx, rx) = channel();
        let (arr, tx) = (Arc::clone(&self.sheet), tx.clone());
        thread::spawn(move||{
           let mut arr =  arr.lock().unwrap();
            let row: u8 = arr.to_vec().len() as u8;
            let col: u8 = arr.to_vec()[0].len() as u8;
            let start: u8 = 65;
            let mut top = String::from("   |");
            let mut bottom = String::new();

            for i in 0..col {
                top.push_str(&format!("{}{}|", (start + i) as char, " ".repeat(9)));
            }
            for i in 0..row {
                if i >= 9{
                    bottom.push_str(&format!("{} |", i + 1));
                }else {
                    bottom.push_str(&format!("{}  |", i + 1));
                }
                for a in 0..col {
                    bottom.push_str(&format!("{}|",arr.to_vec()[a as usize][i as usize].cell_text() ));
                }

                bottom.push('\n');
            }
            tx.send(format!("{}\n{}", top, bottom)).unwrap();
        });
        rx.recv().unwrap()
    }
    pub fn new() -> Spreadsheet {
        self::Spreadsheet {
            sheet: Arc::new(Mutex::new(vec![vec![Cell::Empty; 10]; 10])),
        }
    }
    pub fn process_command(&mut self, input: String) -> Result<String, SpreadsheetError> {
        // TODO: implement update function for FormulaCell struct, since FormulaCell isn't implemnting clone
            let input: Vec<&str> = input.splitn(3,' ').collect();
            match input[0].to_uppercase().as_ref() {
                "SORTA" => Ok(String::from("Command SORTA entered")),
                "SORTD" => Ok(String::from("Command SORTD entered")),
                "SAVE" =>{
                    let serialized = serde_json::to_string(&self).unwrap();
                    fs::write("/tmp/save.json", serialized.clone()).expect("Unable to write file");
                    return Ok(String::from("Saved"))},
                "QUIT" =>{
                    print!("quitting");
                    Err(SpreadsheetError::ExitRequested)
                },
                "CLEAR" => {
                    if input.len() == 2 {
                        let row = (input[1].to_uppercase().as_bytes()[0] - 65) as usize;
                        let col_str = input[0].trim_start_matches(|c: char| !c.is_ascii_digit());
                        let col = match col_str.parse::<usize>() {
                            Ok(num) => num - 1,
                            Err(e) => return Err(SpreadsheetError::ParseIntError(e)),
                        };
                    } else {
                        let row = self.sheet.lock().unwrap().to_vec().len();
                        let col = self.sheet.lock().unwrap()[0].len();
                        for i in 0..row {
                            for a in 0..col {
                                self.sheet.lock().unwrap().to_vec()[a][i] = Cell::Empty;
                            }
                        }
                    }
                    Ok(self.get_grid_text())
                }
                _ => {
                    //let (tx, rx) = channel();
                    let arr = Arc::clone(&self.sheet);

                    let col: u8 = input[0].to_uppercase().as_bytes()[0] - 65;
                    let row: u8 = match input[0].trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<u8>() {
                        Ok(output) => output - 1,
                        Err(e) => return  Err(SpreadsheetError::ParseIntError(e)),
                    };
                    let row_num = row as usize;
                    let column_num = (col) as usize;
                    if input.len() >= 3 {
                        let input_two = input[2];
                        if &input_two[0..1] == "("&& &input_two[input_two.len() - 1..input_two.len()] == ")"
                        {
                            thread::spawn(move||{
                                let data = arr.lock().unwrap();
                                data.to_vec()[row_num][column_num] = Cell::Formula(FormulaCell::new(input_two.parse().unwrap(), self));
                            });

                        }
                        else if &input_two[0..1] == "\"" && &input_two[input_two.len() - 1..input_two.len()] == "\""
                        {
                            self.sheet.lock().unwrap()[row_num][column_num] = Cell::Text(input_two[1..input_two.len() - 1].to_string());
                        } else {
                            self.sheet.lock().unwrap().to_vec()[row_num][column_num] = Cell::Number(match input_two.parse::<f64>() {
                                Ok(num) => num,
                                Err(e) => return Err(SpreadsheetError::ParseFloatError(e)),
                            });
                        }
                    } else {
                        return Ok(self.sheet.lock().unwrap().to_vec()[row_num][column_num].full_text());
                    }

                    return Ok(self.get_grid_text())
                }
            }

    }
}
