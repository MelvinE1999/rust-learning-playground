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

fn check_if_safe(numbers : Vec<i32>) -> u32 {
    let mut prev : i32 = numbers[0];
    let mut is_increasing : bool = if numbers[0] < numbers[1] {true} else {false};

    for i in 1..numbers.len() {

        if (numbers[i] - prev).abs() < 1 || (numbers[i] - prev).abs() > 3 || (is_increasing == (prev > numbers[i])) || (!is_increasing == (prev < numbers[i])) {
            return 0;
        } 
        else {
            prev = numbers[i];
        }
    }
    println!("{:?}", numbers);
    1
}

fn main() {
    let mut safe_count : u32 = 0;


    if let Ok(lines) = read_lines("/Users/mel/Desktop/repos/rust-learning-playground/advent-of-code-2024/day-two/inputs/input.txt") {
        for line in lines.map_while(Result::ok){
            let numbers : Vec<i32> = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
            let numbers_reversed : Vec<i32> = numbers.clone().into_iter().rev().collect(); 
            
            let mut temp = 0;
            

            for j in 0..numbers.len(){
                let mut temp_numbers = numbers.clone();
                temp_numbers.remove(j);
                temp = check_if_safe(temp_numbers);
                if temp == 1 {
                    break;
                }
            }

            safe_count += temp;
        }
    }

    println!("{}", safe_count);
}