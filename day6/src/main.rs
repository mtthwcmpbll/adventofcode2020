use std::env;
use std::fs;
use std::fmt;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");


    //TODO: implement the solver function
    println!("Solution Part 1:  {:?}", solve_part_1(raw_input.as_str()));
    println!("Solution Part 2:  {:?}", solve_part_2(raw_input.as_str()));
}

fn solve_part_1(raw_input: &str) -> usize {
    let mut total = 0;
    
    for group in raw_input.split("\n\n") {
        let mut group_answers = HashSet::new();
        for person in group.lines() {
            for answer in person.chars() {
                group_answers.insert(answer);
            }
        }
        
        let group_yes = group_answers.len();
        total += group_yes;
    }
    
    total
}

fn solve_part_2(raw_input: &str) -> usize {
    let mut total = 0;

    for group in raw_input.split("\n\n") {
        let mut group_answers = HashSet::new();

        // add the answers from the first one
        for answer in group.lines().next().unwrap().chars() {
            group_answers.insert(answer);
        }
        
        for (i, person) in group.lines().enumerate() {
            //not the first one, remove all of the answers from the set that aren't in this answer too
            for answer in group_answers.clone() {
                if !person.contains(answer) {
                    group_answers.remove(&answer);
                }
            }
        }

        let group_yes = group_answers.len();
        total += group_yes;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1_a() {
        let expected = 11;
        let raw_input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(expected, solve_part_1(raw_input));
    }

    #[test]
    fn test_solve_part_2_a() {
        let expected = 6;
        let raw_input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(expected, solve_part_2(raw_input));
    }

}