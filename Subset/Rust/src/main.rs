use std::io::{self, BufRead};
use std::time::Instant;
use console::Style;

use crate::error::CustomError;

mod error;

fn main() {
    match get_input() {
        Ok(number) => {
            let now = Instant::now();
            let _ = get_all_subsets(number);
            let res = now.elapsed().as_micros();

            let red = Style::new().red();
            let seconds = res as f64 / 1_000_000 as f64;
            println!("Time elapsed: {} microseconds (that equals {} seconds)!", red.apply_to(&res), red.apply_to(&seconds));
        },
        Err(err) => {
            println!("Error! {}", err);
        }
    }
}


fn get_input() -> Result<i64, CustomError> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    
    //try to parse the number
    let number:i64 = line.trim().parse::<i64>().map_err(|_| CustomError::Parse)?;

    //make sure it's a valid number
    if number >= 1 && number <= 20 {
        Ok(number)
    } else {
        Err(CustomError::InvalidNumber)
    }
}

fn get_all_subsets(n: i64) -> Vec<Vec<i64>> {
    let numbers = (1..(n + 1)).rev().collect::<Vec<i64>>();

    let result = numbers
        .iter()
        .map(|number| get_subsets_for_number(*number))
        .collect::<Vec<Vec<Vec<i64>>>>();

    let mut result_vec: Vec<Vec<i64>> = Vec::new();
    for vec in result.into_iter() {
        for vec_deeper in vec.into_iter() {
            result_vec.push(vec_deeper);
        }
    }
    
    return result_vec;
}

fn get_subsets_for_number(n: i64) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    let mut numbers = vec![n];
    let mut target = n + 1;
    let mut target_count: i64 = -1;
    
    while !row_complete(n, &numbers) {
        numbers.sort();
        results.push(numbers.clone());

        if target == (*numbers.iter().min().unwrap() + 1) || target == 1 {
            numbers.push(n - 1);
            target = n - 1;
            target_count += 1;
        }else{
            numbers.remove(target_count as usize);
            target -= 1;
            numbers.push(target);
        }
    }

    numbers.sort();
    results.push(numbers.clone());

    return results;
}

fn row_complete(n: i64, vec: &Vec<i64>) -> bool {
    return (vec.len() as i64)  == n;
}

/*
 * 
 * l = 4                --> 4
 * l = 4, 3             --> 3, 4
 * l = 4, 2             --> 2, 4
 * l = 4, 1             --> 1, 4
 * l = 4, 3, 1          --> 1, 3, 4
 * l = 4, 2, 1          --> 1, 2, 4
 * l = 4, 3, 2, 1       --> 1, 2, 3, 4
 * ------------------------------------------
 * l = 3                --> 3
 * l = 3, 2             --> 2, 3
 * l = 3, 1             --> 1, 3
 * l = 3, 2, 1          --> 1, 2, 3 
 * ------------------------------------------
 * l = 2                --> 2
 * l = 2,1              --> 1,2
 * ------------------------------------------
 * l = 1                --> 1
 */