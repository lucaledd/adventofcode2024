use std::fs;

fn main() {
    let contents: String = fs::read_to_string("data.in").expect("cannot read file");
    
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    let nline: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    for i in 0..nline.len() {
        let sline: Vec<&str> = nline[i].split("   ").collect::<Vec<&str>>();

        let left: i32 = sline[0].replace("\r", "").parse().unwrap();
        let right: i32 = sline[1].replace("\r", "").parse().unwrap();
        
        column1.push(left);
        column2.push(right);
    }

    let mut distance = 0;

    while column1.len() > 0 && column2.len() > 0 {
        let mut min_left = i32::MAX;
        let mut min_left_index = 0;
        for i in 0..column1.len() {
            let a = column1[i];
            if min_left > a {
                min_left = a;
                min_left_index = i;
            }
        }

        let mut min_right = i32::MAX;
        let mut min_right_index = 0;
        for i in 0..column2.len() {
            let a = column2[i];
            if min_right > a {
                min_right = a;
                min_right_index = i;
            }
        }

        let diff = i32::abs(min_left - min_right);
        distance += diff;

        column1.remove(min_left_index);
        column2.remove(min_right_index);
    }

    println!("{distance:#?}");
}