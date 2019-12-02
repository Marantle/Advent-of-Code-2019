fn main() {
    let input: Vec<i32> = vec![
        1i32, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 9, 19, 23, 2, 23, 10,
        27, 1, 27, 5, 31, 1, 31, 6, 35, 1, 6, 35, 39, 2, 39, 13, 43, 1, 9, 43, 47, 2, 9, 47, 51, 1,
        51, 6, 55, 2, 55, 10, 59, 1, 59, 5, 63, 2, 10, 63, 67, 2, 9, 67, 71, 1, 71, 5, 75, 2, 10,
        75, 79, 1, 79, 6, 83, 2, 10, 83, 87, 1, 5, 87, 91, 2, 9, 91, 95, 1, 95, 5, 99, 1, 99, 2,
        103, 1, 103, 13, 0, 99, 2, 14, 0, 0,
    ];

    let example_input1 = vec![1i32, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let example_expected1 = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let example_input2 = vec![1i32, 1, 1, 4, 99, 5, 6, 0, 99];
    let example_expected2 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    let example_input3 = vec![2i32, 4, 4, 5, 99, 0];
    let example_expected3 = vec![2, 4, 4, 5, 99, 9801];

    let example_result1 = part1(example_input1.clone());
    let example_result2 = part1(example_input2.clone());
    let example_result3 = part1(example_input3.clone());

    assert!(compare_vecs(example_result1, example_expected1), "Whoops1!");
    assert!(compare_vecs(example_result2, example_expected2), "Whoops2!");
    assert!(compare_vecs(example_result3, example_expected3), "Whoops3!");

    let result1 = part1(input.clone());

    println!("day 1 result [{}]", result1[0]);

    let result2 =part2(input.clone());
    
    println!("day 2: result [{}]", result2);
}

fn part2(data: Vec<i32>) -> i32 {
    let expected = 19690720;
    let mut noun = 0;
    let mut verb = 0;
    let mut result = 0;
    let mut found_noun = -1;
    let mut found_verb = -1;
    while noun < 100 && result < expected {
        while verb < 100 && result < expected {
            let mut local_data = data.clone();
            local_data[1] = noun;
            local_data[2] = verb;
            let day1 = part1(local_data);
            result = day1[0];
            if result == expected {
                found_noun = noun;
                found_verb = verb;
                break;
            }
            verb += 1;
        }
        verb = 0;
        noun += 1;
    }
    return 100 * found_noun + found_verb;
}

fn part1(mut data: Vec<i32>) -> Vec<i32> {
    let mut done: bool = false;
    let mut i: usize = 0;
    while !done {
        let op = data[i];
        if op != 99 {
            let target = data[i + 3] as usize;
            let first_value = data[data[i + 1] as usize];
            let second_value = data[data[i + 2] as usize];
            match op {
                1 => data[target] = first_value + second_value,
                2 => data[target] = first_value * second_value,
                3..=98 => {}
                _ => {}
            }
        } else {
            done = true;
        }
        i += 4;
    }
    return data;
}

fn compare_vecs(data: Vec<i32>, expected: Vec<i32>) -> bool {
    let mut equal: bool = true;
    let mut i: usize = 0;
    while equal && i < data.len() {
        equal = data[i] == expected[i];
        i += 1;
    }
    return equal;
}
