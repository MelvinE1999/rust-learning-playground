use std:: {
    fs::File,
    io::{self,BufRead},
    path::Path,
};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut prev : i32 = 0;
    let mut is_increasing : bool = true;
    let mut safe_count : u32 = 0;


    if let Ok(lines) = read_lines("/Users/mel/Desktop/repos/rust-learning-playground/advent-of-code-2024/day-two/inputs/input.txt") {
        'outer: for line in lines.map_while(Result::ok){
            let numbers : Vec<i32> = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
            is_increasing = if numbers[0] < numbers[1] {true} else {false};
            prev = numbers[0];

            for i in 1..numbers.len() {
                if (numbers[i] - prev).abs() < 1 || (numbers[i] - prev).abs() > 3{
                    continue 'outer;
                } 
                else if is_increasing == (prev > numbers[i]) {
                    continue 'outer;
                }
                else if !is_increasing == (prev < numbers[i]) {
                    continue 'outer;
                }
                else {
                    prev = numbers[i];
                }
            }

            safe_count += 1;
        }
    }

    println!("{}", safe_count);
}
