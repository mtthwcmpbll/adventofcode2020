#[macro_use]
extern crate nom;

use std::env;
use std::fs;
use std::collections::HashMap;

use nom::{
    lib::std::str::FromStr,
    IResult,
    combinator::map,
    sequence::tuple,
    character::complete::digit1,
    character::complete::space0,
    bytes::complete::tag,
    branch::alt,
};
use nom::multi::separated_list;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    
    //TODO: implement the solver function
    println!("Solution Part 1:  {:?}", solve_part_1(raw_input.as_str()));
    println!("Solution Part 2:  {:?}", solve_part_2(raw_input.as_str()));
}

fn solve_part_1(raw_input: &str) -> usize {
    let rules = Rules::new(raw_input);
    println!("There are {} known bag types.", rules.bags.len());
    
    let mut count: usize = 0;
    for k in rules.bags.keys() {
        if rules.can_bag_hold(&k, "shiny gold") {
            count += 1;
        }
    }
    count
}

fn solve_part_2(raw_input: &str) -> u128 {
    let rules = Rules::new(raw_input);
    println!("There are {} known bag types.", rules.bags.len());
    rules.total_bags("shiny gold") - 1 // subtract the root bag to answer "how many bags does this one hold?"
}

struct Rules {
    // Keyed on the rule's bag description, value is a list of bag relationships (ie "5 gold bags", "1 silver bag")
    bags: HashMap<String, Rule>,
}
impl Rules {
    fn new(raw_input: &str) -> Rules {
        let mut rules = Rules {
            bags: HashMap::new(),
        };

        for line in raw_input.lines() {
            let result = parse_rule(line);
            match result {
                Ok((i, rule)) => {
                    if !i.is_empty() {
                        panic!("The rule parser didn't consume the full input, there's probably something wrong!");
                    }
                    rules.bags.insert(rule.description.clone(), rule);
                }, 
                Err(_) => panic!("There's a problem parsing a rule!"),
            }
        }
        
        rules
    }

    fn can_bag_hold(&self, container: &str, contained_bag: &str) -> bool {
        self.can_bag_hold_helper(container, contained_bag, 0)
    }
    fn can_bag_hold_helper(&self, container: &str, contained_bag: &str, depth: u8) -> bool {
        let rule = self.bags.get(container).unwrap();

        match &rule.holds {
            None => false,
            Some(relationships) => {
                return if relationships.contains_key(contained_bag) {
                    println!("A {} bag, which can hold your {} bag directly.", container, contained_bag);
                    true
                } else {
                    // search the bags this one can contain
                    for (k, v) in relationships {
                        if self.can_bag_hold_helper(&k, contained_bag, depth+1) {
                            return true;
                        }
                    }
                    false
                }
            }
        }
    }

    fn total_bags(&self, container: &str) -> u128 {
        let rule = self.bags.get(container).unwrap();

        match &rule.holds {
            None => {
                println!("{} bags don't hold any additional bags", container);
                1
            },
            Some(relationships) => {
                println!("{} bags hold {} additional bags", container, relationships.len());
                // 1 + relationships.len() as u128
                relationships.values().into_iter().fold(1, |c, r| {
                    c + ((r.count as u128) * self.total_bags(&r.to))
                })
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    description: String,
    holds: Option<HashMap<String, BagRelationship>>,
}

#[derive(Debug, PartialEq, Eq)]
struct BagRelationship {
    count: usize,
    to: String,
}

named!(bag_count<&str, u16>,
    alt!(
        map_res!(digit1, FromStr::from_str) |
        map!(tag!("no"), |s| 0u16)
    )
);

named!(bag_description<&str, &str>,
    take_until!(" bag")
);

// macro doesn't seem to work :(
// named!(bags<&str, &str>,    
//     alt!(tag!("bags") | tag!("bag"))
// );

fn bags(input: &str) -> IResult<&str, &str> {
    alt((tag("bags"), tag("bag")))(input)
}

fn bag_holds(input: &str) -> IResult<&str, Option<BagRelationship>> {
    // example input:
    //    5 faded blue bags
    //    1 dark olive bag
    //    no other bags

    let (input, (count, _, description, _, _)) = tuple((
        bag_count,
        space0,
        bag_description,
        space0,
        bags
    ))(input)?;
    
    if count == 0 {
        Ok((input, None))
    } else {
        Ok((input, Some(BagRelationship {
            count: count as usize,
            to: String::from(description),
        })))
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    // example input:
    //    light red bags contain 1 bright white bag, 2 muted yellow bags.
    //    bright white bags contain 1 shiny gold bag.
    //    faded blue bags contain no other bags.
    
    let (input, (description, _, _, _, _, relationships, _)) = tuple((
        bag_description,
        space0,
        bags,
        space0,
        tag("contain "),
        map(separated_list(tag(", "), bag_holds), |v| {
            v.into_iter().fold(HashMap::new(), |mut m, r| {
                match r {
                    Some(holds) => {
                        let holds_desc = holds.to.clone();
                        m.insert(holds_desc, holds);
                        m
                    },
                    None => m,
                }
            })
        }),
        tag("."),
    ))(input)?;
    
    if relationships.is_empty() {
        Ok((input, Rule {
            description: String::from(description),
            holds: None,
        }))
    } else {
        Ok((input, Rule {
            description: String::from(description),
            holds: Some(relationships),
        }))
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1_a() {
        let expected = 4;
        let raw_input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        Rules::new(raw_input);

        assert_eq!(expected, solve_part_1(raw_input));
    }

    #[test]
    fn test_solve_part_2_a() {
        let expected = 126;
        let raw_input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        Rules::new(raw_input);

        assert_eq!(expected, solve_part_2(raw_input));
    }

    #[test]
    fn test_parse_bag_holds_multi() {
        // given
        let expected = Ok(("", Some(BagRelationship {
            count: 3,
            to: String::from("bright white"),
        })));
        let raw_input = "3 bright white bags";
        
        // when
        let result = bag_holds(raw_input);
        
        // then
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_bag_holds_single() {
        // given
        let expected = Ok(("", Some(BagRelationship {
            count: 1,
            to: String::from("silvery grey"),
        })));
        let raw_input = "1 silvery grey bag";

        // when
        let result = bag_holds(raw_input);

        // then
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_bag_holds_no_bags() {
        // given
        let expected = Ok(("", Option::None));
        let raw_input = "no other bags";

        // when
        let result = bag_holds(raw_input);

        // then
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_rule_many_relationships() {
        // given
        let mut expected_relationships = HashMap::new();
        expected_relationships.insert(String::from("bright white"), BagRelationship {
            count: 1,
            to: String::from("bright white"),
        });
        expected_relationships.insert(String::from("muted yellow"), BagRelationship {
            count: 2,
            to: String::from("muted yellow"),
        });
        let expected = Ok(("", Rule {
            description: String::from("light red"),
            holds: Some(expected_relationships)
        }));
        let raw_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        // when
        let result = parse_rule(raw_input);

        // then
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_rule_one_relationship() {
        // given
        let mut expected_relationships = HashMap::new();
        expected_relationships.insert(String::from("shiny gold"), BagRelationship {
            count: 1,
            to: String::from("shiny gold"),
        });
        let expected = Ok(("", Rule {
            description: String::from("bright white"),
            holds: Some(expected_relationships)
        }));
        let raw_input = "bright white bags contain 1 shiny gold bag.";

        // when
        let result = parse_rule(raw_input);

        // then
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_rule_no_relationships() {
        // given
        let expected = Ok(("", Rule {
            description: String::from("faded blue"),
            holds: None
        }));
        let raw_input = "faded blue bags contain no other bags.";

        // when
        let result = parse_rule(raw_input);

        // then
        assert_eq!(expected, result);
    }

}