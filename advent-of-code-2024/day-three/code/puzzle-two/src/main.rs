use lazy_static::lazy_static;
use regex::Regex;
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
    let mut res = 0;
    let mut do_count = true;

    if let Ok(lines) = read_lines("/Users/mel/Desktop/repos/rust-learning-playground/advent-of-code-2024/day-three/inputs/input.txt") {
        for line in lines.map_while(Result::ok){
            lazy_static! {
                static ref RE: Regex = Regex::new(r"don't\(\)|do\(\)|(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
            }
            
            let matches : Vec<&str> = RE.find_iter(&line).map(|m| m.as_str()).collect();
            

            for m in matches {
                if m.starts_with("don") {
                    do_count = false;
                    continue;
                }
                else if m.starts_with("do") {
                    do_count = true;
                    continue;
                }

                if do_count {
                    let s : Vec<_> = m.split(",").collect();
                    let num1_array : Vec<_> = s[0].split("(").collect();
                    let num1 : i32 = num1_array[1].parse().unwrap();
                    let num2_array : Vec<_> = s[1].split(")").collect();
                    let num2 : i32 = num2_array[0].parse().unwrap();
                    res += num1 * num2;
                }
            }
            
            
        }
    
    
    }
    println!("{:?}", res);
}
