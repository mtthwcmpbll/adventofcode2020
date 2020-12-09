use std::env;
use std::fs;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    let input = Map::new(raw_input.as_str());
    
    //TODO: implement the solver function
    println!("Solution Part 1:  {:?}", solve_part_1(&input, 3, 1));
    println!("Solution Part 2:  {:?}", solve_part_2(&input));
}

fn parse(raw_input: &str) {
    raw_input.lines()
        .map(|s| s)
        .collect::<Vec<&str>>();
}

fn solve_part_1(map: &Map, horizontal: usize, vertical: usize) -> i32 {
    let mut trees = 0;
    let mut position = Coordinate { x: 0, y: 0 };
    loop {
        //move
        position = Coordinate {
            x: position.x + horizontal,
            y: position.y + vertical,
        };
        
        // check if we're done
        if position.y >= map.size().height {
            break;
        } 
        
        // check if we hit a tree
        match map.get(position.x, position.y) {
            Space::OPEN => {},
            Space::TREE => trees += 1,
        }
    }
    trees
}

fn solve_part_2(map: &Map) -> u128 {
    let answer1: u128 = solve_part_1(map, 1, 1) as u128;
    let answer2: u128 =  solve_part_1(map, 3, 1) as u128;
    let answer3: u128 = solve_part_1(map, 5, 1) as u128;
    let answer4: u128 = solve_part_1(map, 7, 1) as u128;
    let answer5: u128 = solve_part_1(map, 1, 2) as u128;
    let final_answer = answer1 * answer2 * answer3 * answer4 * answer5;
    println!("part 2: {} * {} * {} * {} * {} = {}",
            answer1,
            answer2,
            answer3,
            answer4,
            answer5, 
            final_answer);
    
    final_answer
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Space {
    OPEN,
    TREE,
}
impl Space {
    fn from(c: char) -> Space {
        match c {
            '.' => Space::OPEN,
            '#' => Space::TREE,
            _ => panic!("Unknown map symbol!"),
        }
    }
}
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::OPEN => write!(f, "."),
            Space::TREE => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Size {
    height: usize,
    width: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Space>>,
}
impl Map {
    fn new(str_map: &str) -> Map {
        let mut map = Map {
            grid: Vec::new()
        };
        
        for row in str_map.lines() {
            // populate the map's row
            let mut row_spaces = Vec::new();
            for symbol in String::from(row).chars() {
                row_spaces.push(Space::from(symbol));
            }
            map.grid.push(row_spaces);
        }
        
        map
    }
    
    // returns (height, width)
    fn size(&self) -> Size {
        Size {
            height: self.grid.len(),
            width: self.grid[0].len()
        }
    }
    
    fn get(&self, x: usize, y: usize) -> &Space {
        &self.grid[y][x % self.size().width]
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, space) in self.grid[y].iter().enumerate() {
                write!(f, "{}", space);
            }
            writeln!(f, "");
        }
        //TODO: this seems wrong, need to find idiomatic way to append a bunch and aggregate the result
        return fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1_a() {
        let expected = 7;
        let raw_input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let mut input = Map::new(raw_input);

        assert_eq!(expected, solve_part_1(&input, 3, 1));
    }

    #[test]
    fn test_map_size() {
        let expected = Size { height: 11, width: 11 };
        let raw_input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = Map::new(raw_input);

        assert_eq!(expected, map.size());
    }

    #[test]
    fn test_map_index() {
        let expected = Space::OPEN;
        let raw_input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = Map::new(raw_input);
        println!("{}", map);
        
        assert_eq!(&expected, map.get(1, 9));
    }
}