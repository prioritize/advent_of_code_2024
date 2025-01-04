use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Guard {
    row: usize,
    col: usize,
    direction: Direction,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Debug)]
pub struct Collision {
    loc: Point,
    direction: Direction,
}

impl Collision {
    pub fn new(row: usize, col: usize, direction: Direction) -> Self {
        Collision {
            loc: Point { row, col },
            direction,
        }
    }
    pub fn find_loop_candidate(
        collision: &Collision,
        objects: HashMap<Direction, Vec<Collision>>,
    ) -> Option<Vec<Collision>> {
        let mut possible_loop: Vec<Collision> = vec![collision.clone()];
        let mut current = collision;
        loop {
            if possible_loop.len() >= 3 {
                break;
            }
            // Get the list of all context correct collisions
            let collisions = objects.get(&current.direction.next()).unwrap();
            println!(
                "Looking for obstacles in {:?} and they are: {:?}",
                current.direction.next(),
                collisions
            );
            let obstacles: Vec<&Collision> = collisions
                .iter()
                .filter(|x| match current.direction {
                    Direction::Up => {
                        x.loc.row == current.loc.row + 1 && x.loc.col > current.loc.col
                    }
                    Direction::Down => {
                        x.loc.row.checked_sub(1).is_some()
                            && x.loc.row == current.loc.row - 1
                            && x.loc.col > current.loc.col
                    }
                    Direction::Left => {
                        x.loc.row > current.loc.row
                            && current.loc.col.checked_sub(1).is_some()
                            && x.loc.col == current.loc.col - 1
                    }
                    Direction::Right => {
                        x.loc.row < current.loc.row && x.loc.col == current.loc.col + 1
                    }
                })
                .collect();
            let mut delta = i32::MAX;
            if obstacles.is_empty() {
                return None;
            }
            let mut closest: &Collision = obstacles.first()?;
            obstacles.iter().for_each(|x| match current.direction {
                Direction::Up => {
                    let temp = i32::abs(x.loc.col as i32 - current.loc.col as i32);
                    if temp < delta {
                        delta = temp;
                        closest = x;
                    }
                }
                Direction::Down => {
                    let temp = i32::abs(x.loc.col as i32 - current.loc.col as i32);
                    if temp < delta {
                        delta = temp;
                        closest = x;
                    }
                }
                Direction::Left => {
                    let temp = i32::abs(x.loc.row as i32 - current.loc.row as i32);
                    if temp < delta {
                        delta = temp;
                        closest = x;
                    }
                }
                Direction::Right => {
                    let temp = i32::abs(x.loc.row as i32 - current.loc.row as i32);
                    if temp < delta {
                        delta = temp;
                        closest = x;
                    }
                }
            });
            println!("Current is {:?}", current);
            println!("closest is {:?}", closest);
            possible_loop.push(closest.clone());
            current = closest;
        }
        Some(possible_loop)
    }

    pub fn find_next_collision(
        &self,
        objects: &HashMap<Direction, Vec<Collision>>,
        move_op: fn(usize, usize) -> Option<usize>,
    ) -> Option<Vec<Collision>> {
        match self.direction {
            Direction::Up => {
                objects.get(&Direction::Right).map(|right_collisions| {
                    right_collisions
                        .iter()
                        .map(|c| {
                            match move_op(self.loc.row, 1).is_some()
                                && c.loc.row == move_op(self.loc.row, 1).unwrap()
                                && c.loc.col > self.loc.col
                            {
                                true => Some(c),
                                false => None,
                            }
                        })
                        .collect::<Vec<Option<&Collision>>>()
                        .iter()
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .collect::<Vec<&Collision>>()
                });
            }
            Direction::Down => {}
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
        todo!()
    }
}
impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
struct Loop {
    ul: Point,
    ur: Point,
    ll: Point,
    lr: Point,
}
#[derive(Debug, Clone)]
struct Point {
    row: usize,
    col: usize,
}
impl Guard {
    fn next_direction(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }
    pub fn forecast_next_position(&self, map: &[Vec<char>]) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => self.row.checked_sub(1).map(|v| (v, self.col)),
            Direction::Down => match self.row + 1 < map[0].len() {
                true => Some((self.row + 1, self.col)),
                false => None,
            },
            Direction::Left => self.col.checked_sub(1).map(|v| (self.row, v)),
            Direction::Right => match self.col + 1 < map.len() {
                true => Some((self.row, self.col + 1)),
                false => None,
            },
        }
    }
    pub fn move_position(&mut self, guard_map: &mut [Vec<char>]) -> bool {
        match self.forecast_next_position(guard_map) {
            Some((r, c)) => {
                match guard_map[r][c] == '#' {
                    true => self.next_direction(),
                    false => {
                        guard_map[self.row][self.col] = 'X';
                        self.row = r;
                        self.col = c;
                        println!("{}, {}", r, c);
                    }
                }
                true
            }
            None => {
                guard_map[self.row][self.col] = 'X';
                false
            }
        }
    }
    pub fn move_position_track_objects(
        &mut self,
        guard_map: &mut [Vec<char>],
    ) -> Option<Option<Collision>> {
        match self.forecast_next_position(guard_map) {
            Some((r, c)) => {
                match guard_map[r][c] == '#' {
                    true => {
                        let collision = Collision::new(r, c, self.direction.clone());
                        self.next_direction();
                        return Some(Some(collision));
                    }
                    false => {
                        guard_map[self.row][self.col] = 'X';
                        self.row = r;
                        self.col = c;
                    }
                }
                Some(None)
            }
            None => {
                guard_map[self.row][self.col] = 'X';
                None
            }
        }
    }
    pub fn perform_guard_duty(guard: &mut Guard, guard_map: &mut [Vec<char>]) {
        while guard.move_position(guard_map) {
            println!("-");
        }
        todo!()
    }
}
pub fn parse_map(fname: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    let mut starting_row = 0;
    let mut starting_col = 0;
    let map = lines
        .into_iter()
        .enumerate()
        .map(|(r_idx, l)| {
            let temp = l.unwrap();
            temp.chars()
                .enumerate()
                .map(|(c_idx, c)| {
                    if c == '^' {
                        starting_col = c_idx;
                        starting_row = r_idx;
                    }
                    c
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>();
    (map, (starting_row, starting_col))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day_6_parse_map() {
        let mut move_count: u32 = 0;
        let (mut map, (starting_row, starting_col)) = parse_map("input/day_6_input.txt");
        let mut guard = Guard {
            row: starting_row,
            col: starting_col,
            direction: Direction::Up,
        };
        while guard.move_position(&mut map) {
            move_count += 1;
        }

        println!("{:?}", map);
        println!("move count: {}", move_count);
        let spots: u32 = map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|j| if *j == 'X' { 1 } else { 0 })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
            .iter()
            .flatten()
            .sum();
        println!("Number of positions: {}", spots);
    }
    #[test]
    fn test_day_6_parse_test_map() {
        let mut move_count: u32 = 0;
        let (mut map, (starting_row, starting_col)) = parse_map("input/day_6_test_input.txt");
        let mut guard = Guard {
            row: starting_row,
            col: starting_col,
            direction: Direction::Up,
        };
        while guard.move_position(&mut map) {
            move_count += 1;
        }

        println!("{:?}", map);
        println!("move count: {}", move_count);
        let spots: u32 = map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|j| if *j == 'X' { 1 } else { 0 })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
            .iter()
            .flatten()
            .sum();
        println!("Number of positions: {}", spots);
    }
    #[test]
    fn test_day_6_parse_find_loops() {
        let mut obstacles: Vec<Collision> = vec![];
        let (mut map, (starting_row, starting_col)) = parse_map("input/day_6_test_input.txt");
        let mut guard = Guard {
            row: starting_row,
            col: starting_col,
            direction: Direction::Up,
        };
        while let Some(step) = guard.move_position_track_objects(&mut map) {
            if let Some(s) = step {
                obstacles.push(s)
            }
        }
        println!("{:?}", obstacles);
    }
    #[test]
    fn test_day_6_parse_find_all_loops() {
        let mut obstacles: Vec<Collision> = vec![];
        let (mut map, (starting_row, starting_col)) = parse_map("input/day_6_input.txt");
        let mut guard = Guard {
            row: starting_row,
            col: starting_col,
            direction: Direction::Up,
        };
        while let Some(step) = guard.move_position_track_objects(&mut map) {
            if let Some(s) = step {
                obstacles.push(s)
            }
        }
        let mut collision_map: HashMap<Direction, Vec<Collision>> = HashMap::new();

        obstacles.iter().for_each(|x| {
            collision_map
                .entry(x.direction.clone())
                .and_modify(|v| v.push(x.clone()))
                .or_insert(vec![x.clone()]);
        });
        let possible_loops: Vec<Option<Vec<Collision>>> = obstacles
            .iter()
            .map(|l| Collision::find_loop_candidate(l, collision_map.clone()))
            .collect();
        let count: u32 = possible_loops
            .iter()
            .map(|x| match x.is_some() {
                true => 1,
                false => 0,
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum();
        possible_loops.iter().for_each(|x| println!("{:?}", x));
        println!("num loops: {}", count);
        println!("possible_loops: {:?}", possible_loops);
        println!("{:?}", obstacles);
        println!("{}", obstacles.len());
    }
}
