#![allow(dead_code)]

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let filename = &args[1];

  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let lines: Vec<&str> = contents.split("\r\n").collect();
  
  let total_fuel: i32 = calculate_total_fuel(lines.clone());
  println!("total fuel for part 1 is [{:?}]", total_fuel);
  
  let total_fuel2: i32 = calculate_total_fuel2(lines.clone());
  println!("total fuel for part 2 is [{:?}]", total_fuel2);
}

fn calculate_total_fuel(contents: Vec<&str>) -> i32 {
  let mut total_fuel: i32 = 0;
  for l in contents {
    let mass: i32 = l.parse().unwrap();
    let required_fuel: i32 = mass / 3 -2;
    total_fuel += required_fuel;
  }
  return total_fuel;
}

fn calculate_total_fuel2(contents: Vec<&str>) -> i32 {
  let mut total_fuel: i32 = 0;
  for l in contents {
    let mass: i32 = l.parse().unwrap();
    let mass_fuel = calculate_one_fuel(mass);
    total_fuel += mass_fuel;
  }
  return total_fuel;
}

fn calculate_one_fuel(mass: i32) -> i32 {
  let required_fuel: i32 = mass / 3 -2;
  if required_fuel > 0 {
    return required_fuel + calculate_one_fuel(required_fuel);
  } else {
    return 0;
  }
}