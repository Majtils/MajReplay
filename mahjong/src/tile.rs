use std::fmt;

enum Tile {
    NumberTile,
    HonorTile,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

enum NumberTile {
    Character(u8),
    Dot(u8),
    Bamboo(u8),
}

impl fmt::Display for NumberTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (suit, num) = match self {
            NumberTile::Character(num) => ('m', num),
            NumberTile::Dot(num) => ('p', num),
            NumberTile::Bamboo(num) => ('s', num),
        };
        write!(f, "{num}{suit}")
    }
}

enum HonorTile {
    Wind(Direction),
    Dragon(DragonColor),
}

impl fmt::Display for HonorTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = match self {
            HonorTile::Wind(dir) => match dir {
                Direction::East => 1,
                Direction::South => 2,
                Direction::West => 3,
                Direction::North => 4,
            },
            HonorTile::Dragon(color) => match color {
                DragonColor::White => 5,
                DragonColor::Green => 6,
                DragonColor::Red => 7,
            },
        };
        write!(f, "z{num}")
    }
}

enum Direction {
    East,
    South,
    West,
    North,
}

enum DragonColor {
    White,
    Green,
    Red,
}
