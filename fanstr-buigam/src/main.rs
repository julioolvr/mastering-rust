use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum Being {
    Orc,
    Human,
}

enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug, PartialEq)]
enum MovementError {
    NoBeingInSquare,
    OutOfBounds,
    BeingInDestinationSquare,
    StoneInDestinationSquare
}

impl Error for MovementError {
    fn description(&self) -> &str {
        "MovementError"
    }
}

impl Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MovementError")
    }
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    beings: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}

impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square {
                             ground: TerrainGround::Soil,
                             block: None,
                             beings: None,
                         });
        }

        Grid {
            size: (size_x, size_y),
            squares: squares,
        }
    }

    fn take_being_from_square(&mut self, coord: (usize, usize)) -> Being {
        let mut square = self.squares.get_mut(coord.0 + coord.1 * self.size.1).unwrap();
        let being = square.beings.take();
        being.unwrap()
    }

    fn put_being_in_square(&mut self, coord: (usize, usize), being: Being) {
        let mut square = self.squares.get_mut(coord.0 + coord.1 * self.size.1).unwrap();
        square.beings = Some(being);
    }

    fn move_being_in_coord(&mut self,
                           coord: (usize, usize),
                           dir: Direction)
                           -> Result<(usize, usize), MovementError> {
        let mut new_x;
        let mut new_y;

        {
            let origin_square = self.squares
                .get(coord.0 + coord.1 * self.size.1)
                .expect("Index out of bounds trying to get being");

            match origin_square.beings {
                Some(ref being) => being,
                None => return Err(MovementError::NoBeingInSquare),
            };

            new_x = match dir {
                Direction::East => coord.0 + 1,
                Direction::West => {
                    if let Some(val) = coord.0.checked_sub(1) {
                        val
                    } else {
                        return Err(MovementError::OutOfBounds);
                    }
                }
                _ => coord.0,
            };

            new_y = match dir {
                Direction::South => coord.1 + 1,
                Direction::North => {
                    if let Some(val) = coord.1.checked_sub(1) {
                        val
                    } else {
                        return Err(MovementError::OutOfBounds);
                    }
                }
                _ => coord.1,
            };

            if new_x >= self.size.0 || new_y >= self.size.1 {
                return Err(MovementError::OutOfBounds);
            }

            let destination_square = self.squares
                .get(new_x + new_y * self.size.1)
                .expect("Index out of bounds trying to get being");

            if destination_square.beings.is_some() {
                return Err(MovementError::BeingInDestinationSquare);
            }

            if let Some(ref block) = destination_square.block {
                if *block == TerrainBlock::Stone {
                    return Err(MovementError::StoneInDestinationSquare);
                }
            }
        }

        // So far so good, let's mutate everything
        let being = self.take_being_from_square(coord);
        let new_coords = (new_x, new_y);
        self.put_being_in_square(new_coords, being);

        Ok(new_coords)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.beings, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5 * 13);
        assert_eq!(number_of_squares, 5 * 13);
    }

    #[test]
    fn test_move_being_without_being_in_square() {
        let mut grid = ::Grid::generate_empty(3, 3);
        assert_eq!(grid.move_being_in_coord((0, 0), ::Direction::West),
                   Err(::MovementError::NoBeingInSquare));
    }

    #[test]
    fn test_move_being_out_of_grids_beginning() {
        let mut grid = ::Grid::generate_empty(3, 3);
        grid.squares[0].beings = Some(::Being::Human {});
        assert_eq!(grid.move_being_in_coord((0, 0), ::Direction::North),
                   Err(::MovementError::OutOfBounds));
    }

    #[test]
    fn test_move_being_out_of_grids_end() {
        let mut grid = ::Grid::generate_empty(3, 3);
        grid.squares[3 * 2 + 2].beings = Some(::Being::Human {});
        assert_eq!(grid.move_being_in_coord((2, 2), ::Direction::East),
                   Err(::MovementError::OutOfBounds));
    }

    #[test]
    fn test_move_on_square_occupied_by_being() {
        let mut grid = ::Grid::generate_empty(3, 3);
        grid.squares[0].beings = Some(::Being::Human {});
        grid.squares[1].beings = Some(::Being::Human {});
        assert_eq!(grid.move_being_in_coord((0, 0), ::Direction::East),
                   Err(::MovementError::BeingInDestinationSquare));
    }

    #[test]
    fn test_move_on_square_with_stone() {
        let mut grid = ::Grid::generate_empty(3, 3);
        grid.squares[0].beings = Some(::Being::Human {});
        grid.squares[1].block = Some(::TerrainBlock::Stone);
        assert_eq!(grid.move_being_in_coord((0, 0), ::Direction::East),
                   Err(::MovementError::StoneInDestinationSquare));
    }

    #[test]
    fn test_move_successfully() {
        let mut grid = ::Grid::generate_empty(3, 3);
        grid.squares[0].beings = Some(::Being::Human {});
        let result = grid.move_being_in_coord((0, 0), ::Direction::East);
        assert!(result.is_ok());
        assert!(grid.squares[0].beings.is_none());
        assert!(grid.squares[1].beings.is_some());
    }
}
