use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input: Vec<Password> = contents.lines()
        .map(|s| Password::new(s))
        .collect();

    let valid_passwords = count_valid_passwords(input);

    println!("Valid Passwords: {}", valid_passwords);
}

#[derive(Debug)]
struct PasswordPolicy {
    min: i32,
    max: i32,
    char: char,
}

impl PasswordPolicy {
    fn new(policy_str: &str) -> PasswordPolicy {
        let parts: Vec<&str> = policy_str.split(" ").collect();
        let minmax = String::from(parts[0]);
        let char = parts[1].chars().collect::<Vec<char>>()[0];

        let parts: Vec<&str> = minmax.split("-").collect();
        let min = parts[0].parse::<i32>().unwrap();
        let max = parts[1].parse::<i32>().unwrap();

        PasswordPolicy { min, max, char }
    }
}

#[derive(Debug)]
struct Password {
    policy: PasswordPolicy,
    password: String,
}

impl Password {
    fn new(line: &str) -> Password {
        let parts: Vec<&str> = line.split(": ").collect();
        let policy = PasswordPolicy::new(parts[0]);
        let password = String::from(parts[1]);

        Password {
            policy,
            password
        }
    }
    
    fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.policy.char {
                count += 1;
            }
        }
        return count >= self.policy.min && count <= self.policy.max
    }

    fn is_valid_part2(&self) -> bool {
        println!("Testing password {:?}", self);
        let chars: Vec<char> = self.password.chars().collect();
        return ((chars[(self.policy.min-1) as usize] == self.policy.char && chars[(self.policy.max-1) as usize] != self.policy.char)
            || (chars[(self.policy.min-1) as usize] != self.policy.char && chars[(self.policy.max-1) as usize] == self.policy.char))
    }
}

fn count_valid_passwords(input: Vec<Password>) -> usize {
    input.iter()
        .map(|p| p.is_valid_part2())
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_passwords_2() {
        let expected = 2;
        let raw_input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let mut input: Vec<Password> = raw_input.split("\n")
            .map(|s| Password::new(s))
            .collect();

        assert_eq!(expected, count_valid_passwords(input));
    }

    #[test]
    fn test_count_valid_passwords_3() {
        let expected = 3;
        let raw_input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
2-9 z: zzzzzzzzz";
        let mut input: Vec<Password> = raw_input.split("\n")
            .map(|s| Password::new(s))
            .collect();

        assert_eq!(expected, count_valid_passwords(input));
    }
}