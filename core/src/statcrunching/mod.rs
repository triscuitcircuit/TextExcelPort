#[derive(Debug)]
pub enum StatError{
    NoDelimiter,
    NoInput,
    ParseError,
    IndexError
}

pub fn parse(input: &str, delimiter: &str) -> Vec<f32>{
    return input.split(delimiter).flat_map(|x|x.parse::<f32>()).collect();
}


pub mod iqr;

