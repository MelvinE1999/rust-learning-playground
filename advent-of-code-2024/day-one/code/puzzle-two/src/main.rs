use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    str::FromStr,
};



fn run() -> Result<(), Box<dyn Error>> {
    let mut similarity_scores : Vec<i32> = Vec::new();
    let file = File::open("/Users/mel/Desktop/repos/rust-learning-playground/advent-of-code-2024/day-one/inputs/puzzle-one-data.csv")?;
    let mut list1 : Vec<i32> = Vec::new();
    let mut counter_for_list2 = HashMap::new();
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;
        let num1 : i32 = FromStr::from_str(&record[0]).unwrap();
        let num2 : i32 = FromStr::from_str(&record[1]).unwrap();

        list1.push(num1);
        let counter_for_list2_pair = counter_for_list2.entry(num2).or_insert(0);
        *counter_for_list2_pair += 1;
    }

    list1.sort();

    for i in 0..list1.len() {
       similarity_scores.push(list1[i] * (counter_for_list2.get(&list1[i]).copied().unwrap_or(0)));
    }

    let total_similarity : i32 = similarity_scores.iter().sum();

    println!("{}", total_similarity);
    

    Ok(())
}

fn main() {
    let _ = run();
}
