use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let numbers = find_sums_to_2020(contents.lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect());
    
    println!("Your numbers are {} and {}, multiplied makes {}", numbers.0, numbers.1, numbers.0*numbers.1);
}

fn find_sums_to_2020(input: Vec<i32>) -> (i32, i32) {
    for (i, num1) in input.iter().enumerate() {
        for (j, num2) in input.iter().enumerate() {
            if ((i != j) && (num1 + num2 == 2020)) {
                return (num1.clone(), num2.clone())
            }
        }
    }
    (0,0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sums_to_2020_1721_299() {
        let expected = (1721, 299);
        let raw_input = "\
1721
979
366
299
675
1456";
        let mut input: Vec<i32> = raw_input.split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        assert_eq!(expected, find_sums_to_2020(input));
    }

    #[test]
    fn test_find_sums_to_2020_1720_300() {
        let expected = (1720, 300);
        let raw_input = "\
1720
979
366
300
675
1456";
        let mut input: Vec<i32> = raw_input.split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        assert_eq!(expected, find_sums_to_2020(input));
    }
}