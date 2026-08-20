use std::collections::HashMap;

fn main() {
    let samples: Vec<Vec<i32>> = vec![
        vec![1, 3, 3, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 8, 9],
        vec![5, 5, 2, 2, 7],
        vec![],
    ];

    for numbers in &samples {
        match median(numbers) {
            Some(m) => println!("The median of {:?} is: {}", numbers, m),
            None => println!("The median of {:?} is: None", numbers),
        }
        match mode(numbers) {
            Some(m) => println!("The mode of {:?} is: {}", numbers, m),
            None => println!("The mode of {:?} is: None", numbers),
        }
        println!();
    }
}

fn median(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_unstable();

    let mid = sorted_numbers.len() / 2;
    // if the number of elements is even, take the average of the two middle numbers
    // if is odd, take the middle number.
    if sorted_numbers.len() % 2 == 0 {
        Some((sorted_numbers[mid - 1] as f64 + sorted_numbers[mid] as f64) / 2.0)
    } else {
        Some(sorted_numbers[mid] as f64)
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut counts: HashMap<i32, u32> = HashMap::new();
    for &n in numbers {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mut best_value = 0;
    let mut best_count = 0;

    for (&value, &count) in &counts {
        if count > best_count {
            best_count = count;
            best_value = value;
        }
    }

    Some(best_value)
}
