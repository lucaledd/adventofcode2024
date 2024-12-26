use std::fs;

fn part_one(reports: Vec<&str>) {
    let mut safe_reports: i32 = 0;

    for i in reports {
        let report: Vec<i32> = i.replace("\r", "")
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut j: i32 = -1;
        // false == subtracting ; true == adding
        let mut order: bool = false;
        
        let mut is_init: bool = true;
        let mut is_unsafe: bool = false;
        for k in report {
            if j != -1 {
                let temp_order: bool = k > j;
                if order != temp_order && !is_init {
                    is_unsafe = true;
                    break;
                }
                order = temp_order;
                is_init = false;

                let diff: i32 = i32::abs(k - j);
                if diff < 1 || diff > 3 {
                    is_unsafe = true;
                    break;
                }
            }
            
            j = k;
        }

        if !is_unsafe {
            safe_reports += 1;
        }
    }

    println!("{safe_reports}");
}

fn main() {
    let contents: String = fs::read_to_string("data.in").expect("cannot read file");
    
    let reports: Vec<&str> = contents.split("\n").collect();

    part_one(reports);
}