use std::fs;

fn main() {
    let filename = "../input.txt";

    let content =
        fs::read_to_string(filename).expect("Okay you dont have a file here with that name");

    let numberList: Vec<u32> = convert_content(&content);

    //println!("{:?}", numberList);
    count_increase(numberList);
}

pub fn count_increase(nums: Vec<u32>) {
    let mut temp: u32 = 0;
    let mut result: u32 = 0;

    for val in nums.iter() {
        if temp != 0 {
            if val.clone() > temp {
                println!("{} Increased", val);
                result += 1;
            } else {
                println!("{} Decreased", val);
                temp = val.clone();
            }
            temp = val.clone();
        } else {
            //println!("first number");
            println!("{} Decreased", val);
            temp = val.clone();
        }
    }

    println!("Counter Increase = {}", result)
}

pub fn convert_content<'a>(contents: &'a str) -> Vec<u32> {
    let mut numlist = Vec::new();

    for line in contents.lines() {
        numlist.push(line.parse::<u32>().unwrap());
    }

    numlist
}
