#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    #[allow(dead_code)]
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

    fn move_being_in_coord(&self,
                           coord: (usize, usize),
                           dir: Direction)
                           -> Result<(usize, usize), MovementError> {
        let square = self.squares
            .get(coord.0 * self.size.0 + coord.1)
            .expect("Index out of bounds trying to get being");

        match square.beings {
            Some(ref being) => being,
            None => return Err(MovementError::NoBeingInSquare),
        };

        let new_x = match dir {
            Direction::West => coord.0 + 1,
            Direction::East => {
                if let Some(val) = coord.0.checked_sub(1) {
                    val
                } else {
                    return Err(MovementError::OutOfBounds);
                }
            }
            _ => coord.0,
        };

        let new_y = match dir {
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

        Ok((new_x, new_y))
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
        let grid = ::Grid::generate_empty(3, 3);
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
        assert_eq!(grid.move_being_in_coord((2, 2), ::Direction::West),
                   Err(::MovementError::OutOfBounds));
    }
}
