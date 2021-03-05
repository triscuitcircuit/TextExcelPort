use crate::statcrunching::StatError;

pub fn iqr_calc_string(mut input: Vec<f32>) -> Result<String,StatError>{
    let mut value = String::new();
    input.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let q2 = if let Ok(q2) = iqr_calc_helper(&input){
        q2
    }else { return Err(StatError::IndexError) };
    value.push_str(format!("(Median)Q2: {}    ",q2).as_str());
    let q1 = if let Ok(q1) = iqr_calc_helper(&input
        [0..input.len() / 2].to_vec()){
        q1
    }else { return Err(StatError::IndexError) };
    value.push_str(format!("Q1: {}    ",q1).as_str());

    let q3 = if let Ok(q3) = iqr_calc_helper(&input
                                 [input.len()/2..input.len()].to_vec()){
        q3
    }else { return Err(StatError::IndexError) };
    value.push_str(format!("Q3: {}    ",q3).as_str());
    value.push_str(format!("IQR:{}    Lower-bound: {}     Upper-bound:{}",q3-q1,q1-1.5*(q3-q1),q3+1.5*(q3-q1)).as_str());
    return Ok(value)
}
fn iqr_calc_helper(input: &Vec<f32>) -> Result<f32,StatError>{
    let size = input.len();
    return if size %2 == 0{
        if let a = (input[size/2] + input[(size/2)-1]) / 2.0{
            Ok(a)
        }else {
            return Err(StatError::IndexError)
        }
    }else{

        if let a = input[size/2] / 2.0 {
            Ok(a)
        }else {
            return Err(StatError::IndexError)
        }
    }
}

pub fn standard_deviation(input: Vec<i64>)-> Result<String,StatError>{
    unimplemented!()
}