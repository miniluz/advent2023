fn to_number_list(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn parse_numbers(line: &str) -> Vec<u32> {
    let mut numbers = Vec::<u32>::new();

    let line_chars: Vec<char> = line.chars().collect();

    for i in 0..line_chars.len() {
        if let Some(n) = line_chars[i].to_digit(10) {
            numbers.push(n);
            continue;
        }

        let slice = line_chars.get(i..i + 3);
        if let Some(slice) = slice {
            let slice = slice.iter().collect::<String>().to_lowercase();
            if slice == "one" {
                numbers.push(1);
                continue;
            } else if slice == "two" {
                numbers.push(2);
                continue;
            } else if slice == "six" {
                numbers.push(6);
                continue;
            }
        } else {
            continue;
        };

        let slice = line_chars.get(i..i + 4);
        if let Some(slice) = slice {
            let slice = slice.iter().collect::<String>().to_lowercase();
            if slice == "four" {
                numbers.push(4);
                continue;
            } else if slice == "five" {
                numbers.push(5);
                continue;
            } else if slice == "nine" {
                numbers.push(9);
                continue;
            }
        } else {
            continue;
        };

        let slice = line_chars.get(i..i + 5);
        if let Some(slice) = slice {
            let slice = slice.iter().collect::<String>().to_lowercase();
            if slice == "three" {
                numbers.push(3);
                continue;
            } else if slice == "seven" {
                numbers.push(7);
                continue;
            } else if slice == "eight" {
                numbers.push(8);
                continue;
            }
        }
    }

    return numbers;
}

fn get_first_and_last(numbers: Vec<u32>) -> Option<u32> {
    let first = numbers.get(0);
    let last = numbers.get(numbers.len() - 1);

    if let (Some(&first), Some(&last)) = (first, last) {
        return Some(first * 10 + last);
    } else {
        return None;
    }
}

fn main() {
    println!("Part 1 - Test input:");
    let lines = include_str!("test-input-1.txt").lines();

    println!(
        "{}",
        lines
            .map(|line| get_first_and_last(to_number_list(line)).unwrap())
            .sum::<u32>()
    );

    println!("Part 1 - Real input:");
    let lines = include_str!("real-input.txt").lines();

    println!(
        "{}",
        lines
            .map(|line| get_first_and_last(to_number_list(line)).unwrap())
            .sum::<u32>()
    );

    println!("Part 2 - Test input:");
    let lines = include_str!("test-input-2.txt").lines();

    println!(
        "{}",
        lines
            .map(|line| get_first_and_last(parse_numbers(line)).unwrap())
            .sum::<u32>()
    );

    println!("Part 2 - Real input:");
    let lines = include_str!("real-input.txt").lines();

    println!(
        "{}",
        lines
            .map(|line| get_first_and_last(parse_numbers(line)).unwrap())
            .sum::<u32>()
    );
}
