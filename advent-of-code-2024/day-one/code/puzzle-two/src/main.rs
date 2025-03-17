use std::{
    error::Error,
    fs::File,
    io::Read,
    str::FromStr,
};



fn run() -> Result<(), Box<dyn Error>> {
    let mut absolute_differences : Vec<i32> = Vec::new();
    let file = File::open("/Users/mel/Desktop/repos/rust-learning-playground/advent-of-code-2024/day-one/inputs/puzzle-one-data.csv")?;
    let mut list1 : Vec<i32> = Vec::new();
    let mut list2 : Vec<i32> = Vec::new();
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;
        let num1 : i32 = FromStr::from_str(&record[0]).unwrap();
        let num2 : i32 = FromStr::from_str(&record[1]).unwrap();

        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        absolute_differences.push((list1[i]-list2[i]).abs());
    }

    let total_differences : i32 = absolute_differences.iter().sum();

    println!("{}", total_differences);
    

    Ok(())
}

fn main() {
    let _ = run();
}
