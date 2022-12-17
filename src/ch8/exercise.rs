use std::collections::HashMap;

pub(crate) fn one() {
    let mut nums = vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9, 10];
    let avg = average(&nums);
    let median = median(&mut nums);
    let mode = mode(&nums);
    println!("{:?}", avg);
    println!("{:?}", median);
    println!("{:?}", mode);
}

fn mode(nums: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (num, count) in &map {
        if *count > max_count {
            max_count = *count;
            mode = **num;
        }
    }
    mode
}

fn median(nums: &mut [i32]) -> i32 {
    nums.sort();
    nums[nums.len() / 2]
}

fn average(nums: &[i32]) -> i32 {
    nums.iter().sum::<i32>() / nums.len() as i32
}

pub(crate) fn two() {
    let mut str = String::from("first");
    let vowel_result = pig_latin(str);
    println!("{:?}", vowel_result);

    let mut str = String::from("apple");
    let consonant_result = pig_latin(str);
    println!("{:?}", consonant_result);
}

fn pig_latin(mut str: String) -> String {
    match &str[0..1] {
        "a" | "e" | "i" | "o" | "u" => str.push_str("-hay"),
        _ => {
            let first_letter = str.remove(0);
            str.push_str(&format!("-{}ay", first_letter));
        }
    }
    str
}

// 待修改
pub(crate) fn three() {
    let str = String::from("Add Sally to Engineering");
    let str_vec: Vec<&str> = str.split_whitespace().collect();
    let employee = str_vec[1];
    let department = str_vec[3];
    let mut department_map: HashMap<&str, Vec<&str>> = HashMap::new();
    department_map
        .entry(department)
        .or_insert(vec![])
        .push(employee);
    println!("{:?}", department_map);

    let employees = department_map.get("Engineering").unwrap();
    println!("{:?}", employees);
}
