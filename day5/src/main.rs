use std::env;
use std::fs;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");


    //TODO: implement the solver function
    println!("Solution Part 1:  {:?}", solve_part_1(raw_input.as_str()));
    println!("Solution Part 2:  {}", solve_part_2(raw_input.as_str()));
}

fn solve_part_1(raw_input: &str) -> u32 {
    let mut max = 0;
    for ticket_number in raw_input.lines() { 
        let this_id = TicketNumber::new(ticket_number).id();
        if this_id > max {
            max = this_id;
        }
    }
    max
}

fn solve_part_2(raw_input: &str) -> Plane {
    let mut plane = Plane::new();
    for ticket_number in raw_input.lines() {
        plane.add(&TicketNumber::new(ticket_number));
    }
    plane
}

#[derive(Debug)]
struct Plane {
    seats: [[char; 8]; 128],
}
impl Plane {
    fn new() -> Plane {
        Plane {
            seats: [['.'; 8]; 128],
        }
    }
    
    fn add(&mut self, ticket: &TicketNumber) {
        self.seats[ticket.row as usize][ticket.column as usize] = 'X';
    }
}
impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (r, row) in self.seats.iter().enumerate() {
            write!(f, "{}: ", r);
            for (c, col) in self.seats[r].iter().enumerate() {
                write!(f, "{}", self.seats[r][c]);
            }
            writeln!(f, "");
        }
        return fmt::Result::Ok(())
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct TicketNumber {
    row: u8,
    column: u8,
}
impl TicketNumber {
    fn new(ticket_number: &str) -> TicketNumber {
        let mut r: (u8, u8) = (0, 127);
        let mut c: (u8, u8) = (0, 7);
        for (i, nextchar) in ticket_number.chars().enumerate() {
            if i < 7 {
                // front and back (rows)
                match nextchar {
                    'F' => r = TicketNumber::first_half(r),
                    'B' => r = TicketNumber::last_half(r),
                    _ => panic!("Invalid front/back symbol!"),
                }
            } else {
                match nextchar {
                    'L' => c = TicketNumber::first_half(c),
                    'R' => c = TicketNumber::last_half(c),
                    _ => panic!("Invalid front/back symbol!"),
                }
            }
        }

        if r.0 == r.1 && c.0 == c.1 {
            // we've converged correctly on a row and column
            return TicketNumber {
                row: r.0,
                column: c.0,
            }
        }
        
        println!("Final tally: r: {:?}, c: {:?}", r, c);
        panic!("There's an error in the ticket number parsing logic!");
    }
    
    fn first_half(range: (u8, u8)) -> (u8, u8) {
        (range.0, range.1 - (((range.1+1) - range.0) / 2))
    }

    fn last_half(range: (u8, u8)) -> (u8, u8) {
        (range.0 + (((range.1+1) - range.0) / 2), range.1)
    }
    
    fn id(&self) -> u32 {
        (self.row as u32 * 8u32) + (self.column as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1_a() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        let expected = TicketNumber {
            row: 70,
            column: 7
        };
        let raw_input = "BFFFBBFRRR";

        assert_eq!(567, expected.id());
        assert_eq!(expected, TicketNumber::new(raw_input));
    }

    #[test]
    fn test_solve_part_1_b() {
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        let expected = TicketNumber {
            row: 14,
            column: 7
        };
        let raw_input = "FFFBBBFRRR";

        assert_eq!(119, expected.id());
        assert_eq!(expected, TicketNumber::new(raw_input));
    }

    #[test]
    fn test_solve_part_1_c() {
        // BBFFBBFRLL: row 102, column 4, seat ID 820
        let expected = TicketNumber {
            row: 102,
            column: 4
        };
        let raw_input = "BBFFBBFRLL";

        assert_eq!(820, expected.id());
        assert_eq!(expected, TicketNumber::new(raw_input));
    }

    #[test]
    fn test_range_first_half_1() {
        let expected = (0, 63);
        let raw_input = (0, 127);
        assert_eq!(expected, TicketNumber::first_half(raw_input));
    }

    #[test]
    fn test_range_first_half_2() {
        let expected = (64, 95);
        let raw_input = (64, 127);
        assert_eq!(expected, TicketNumber::first_half(raw_input));
    }

    #[test]
    fn test_range_last_half_1() {
        let expected = (32, 63);
        let raw_input = (0, 63);
        assert_eq!(expected, TicketNumber::last_half(raw_input));
    }
}