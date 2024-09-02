//! This module provides types used to work with Riichi Mahjong tiles

/// This trait represents a Riichi Mahjong tile which has a suit and number.
/// Mahjong tiles can either by a NumberTile or an HonorTile.
pub trait Tile {
    /// The number function extracts the number associated with the tile in
    /// accordance with standard Mahjong notation.
    /// For the numerical tiles, it extracts the number. If the numeric tile is
    /// a red five tile, then it gets the number 0.
    /// For honor tiles, it extracts the number based on the following convention:
    ///
    /// East Wind       ->  1\
    /// South Wind      ->  2\
    /// West Wind       ->  3\
    /// North Wind      ->  4\
    /// White Dragon    ->  5\
    /// Green Dragon    ->  6\
    /// Red Dragon      ->  7
    /// # Examples
    /// ```rust
    /// use mahjong::Direction;
    /// use mahjong::tile::{NumberTile, HonorTile, DragonColor, Tile};
    /// let three_dot_tile = NumberTile::Dot(3);
    /// let three_dot_number = three_dot_tile.number();
    /// assert_eq!(three_dot_number, 3);
    ///
    /// let west_wind_tile = HonorTile::Wind(Direction::West);
    /// let west_wind_number = west_wind_tile.number();
    /// assert_eq!(west_wind_number, 3);
    ///
    /// let white_dragon_tile = HonorTile::Dragon(DragonColor::White);
    /// let white_dragon_number = white_dragon_tile.number();
    /// assert_eq!(white_dragon_number, 5);
    /// ```
    fn number(&self) -> u8;

    /// The suit function extracts the suit associated with the tile.
    /// For numeric tiles, this is 'm' for characters (manzu), 'p' for dots
    /// (pinzu), and 's' for bamboo (souzu). For honor tiles, they get the
    /// character 'z' from zi/jihai.
    /// # Examples
    /// ```rust
    /// use mahjong::Direction;
    /// use mahjong::tile::{NumberTile, HonorTile, DragonColor, Tile};
    /// let three_dot_tile = NumberTile::Dot(3);
    /// let three_dot_suit = three_dot_tile.suit();
    /// assert_eq!(three_dot_suit, 'p');
    ///
    /// let west_wind_tile = HonorTile::Wind(Direction::West);
    /// let west_wind_suit = west_wind_tile.suit();
    /// assert_eq!(west_wind_suit, 'z');
    ///
    /// let white_dragon_tile = HonorTile::Dragon(DragonColor::White);
    /// let white_dragon_suit = white_dragon_tile.suit();
    /// assert_eq!(white_dragon_suit, 'z');
    /// ```
    fn suit(&self) -> char;
}

/// This enum represents a "Union" type to bring together the two different types
/// of Riichi Mahjong tiles
#[derive(Debug, PartialEq, Eq)]
pub enum MahjongTile {
    Honor(HonorTile),
    Number(NumberTile),
}

// TODO: Implement is_honor, is_number, and other helper functions
impl MahjongTile {}

impl fmt::Display for MahjongTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number(), self.suit())
    }
}

impl Tile for MahjongTile {
    fn suit(&self) -> char {
        use crate::tile::MahjongTile::*;
        match self {
            Honor(tile) => tile.suit(),
            Number(tile) => tile.suit(),
        }
    }
    fn number(&self) -> u8 {
        use crate::tile::MahjongTile::*;
        match self {
            Honor(tile) => tile.number(),
            Number(tile) => tile.number(),
        }
    }
}

/// This type represents a number tile. There are three suits of number tiles:
/// characters (manzu), dots (pinzu), and bamboo (souzu).
/// The number represents the number of the tile except for a 0 which represents
/// a red five tile of that suit.
#[derive(Debug, PartialEq, Eq)]
pub enum NumberTile {
    /// The character or manzu tiles are written in Chinese/Kanji.
    Character(u8),
    /// The dot or pinzu tiles have a certain number of dots on them.
    Dot(u8),
    /// The bamboo or souzu tiles have a certain number bamboo sticks on them.
    Bamboo(u8),
}

impl fmt::Display for NumberTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number(), self.suit())
    }
}

impl Tile for NumberTile {
    fn number(&self) -> u8 {
        use NumberTile::*;
        match *self {
            Character(num) | Dot(num) | Bamboo(num) => num,
        }
    }

    fn suit(&self) -> char {
        use NumberTile::*;
        match *self {
            Character(_) => 'm',
            Dot(_) => 'p',
            Bamboo(_) => 's',
        }
    }
}

use core::fmt;

use crate::Direction;
/// This type represents an honor tile. The honors are broken up to wind tiles
/// and dragon tiles. There are four types of wind tiles and three types of
/// dragon tiles.
#[derive(Debug, PartialEq, Eq)]
pub enum HonorTile {
    /// This represents a wind tile. There are four variants depending on the
    /// direction: north, east, south, west.
    Wind(Direction),

    /// This represents a dragon tile. There are three variants depending on the
    /// color: white, green, red.
    Dragon(DragonColor),
}

impl fmt::Display for HonorTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.number(), self.suit())
    }
}

impl Tile for HonorTile {
    fn suit(&self) -> char {
        'z'
    }

    fn number(&self) -> u8 {
        use Direction::*;
        use DragonColor::*;
        use HonorTile::*;
        match *self {
            Wind(East) => 1,
            Wind(South) => 2,
            Wind(West) => 3,
            Wind(North) => 4,
            Dragon(White) => 5,
            Dragon(Green) => 6,
            Dragon(Red) => 7,
        }
    }
}

/// This type represents the colors that a dragon tile can be.
#[derive(Debug, PartialEq, Eq)]
pub enum DragonColor {
    White,
    Green,
    Red,
}

/// The build function takes in a string representation of a tile such as
/// 1z or 3p and converts into the corresponding White Dragon or 3 dots
/// tile type.
///
/// If successful, it returns an Ok with a MahjongTile type.
/// Otherwise, it errors with a String error message.
/// # Errors
/// The function expects a string of length 2, a number and then a character
/// If there are more than 2 letters or the string is not a valid mahjong tile,
/// an error is returned with a String message.
/// # Examples
/// ```rust
/// use mahjong::tile;
/// let three_dot_tile = tile::build("3p");
/// assert!(three_dot_tile.is_ok());
///
/// let too_many_letters = tile::build("3pp");
/// assert!(too_many_letters.is_err());
///
/// let invalid_tile = tile::build("10z");
/// assert!(invalid_tile.is_err());
/// ```
pub fn build(tile_string: &str) -> Result<MahjongTile, String> {
    use crate::{
        tile::{DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*},
        Direction::*,
    };

    if tile_string.chars().count() != 2 {
        return Err("Invalid number of characters".to_string());
    };

    let mut chars = tile_string.chars();

    let number = chars
        .next()
        .expect("Previous error checking should make first character valid")
        .to_digit(10);
    if number.is_none() {
        return Err("First character is not a number".to_string());
    }

    let number =
        number.expect("Previous error checking should make first character a number") as u8;
    let suit = chars
        .next()
        .expect("Previous error checking should make second character valid");

    match (suit, number) {
        ('z', 1) => Ok(Honor(Wind(East))),
        ('z', 2) => Ok(Honor(Wind(South))),
        ('z', 3) => Ok(Honor(Wind(West))),
        ('z', 4) => Ok(Honor(Wind(North))),
        ('z', 5) => Ok(Honor(Dragon(White))),
        ('z', 6) => Ok(Honor(Dragon(Green))),
        ('z', 7) => Ok(Honor(Dragon(Red))),
        ('z', _) => Err("Invalid number for tile".to_string()),
        ('m', num) => Ok(Number(Character(num))),
        ('p', num) => Ok(Number(Dot(num))),
        ('s', num) => Ok(Number(Bamboo(num))),
        _ => Err("Invalid suit".to_string()),
    }
}

#[cfg(test)]
mod tests {
    mod is_honor_tile_tests {
        use crate::{Direction::*, DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*};
        #[test]
        fn wind_tiles() {
            let directions = [East, South, West, North];
            for direction in directions {
                let wind_tile = Honor(Wind(direction));
                assert!(wind_tile.is_honor());
            }
        }

        #[test]
        fn dragon_tiles() {
            let colors = [White, Green, Red];
            for color in colors {
                let dragon_tile = Honor(Dragon(color));
                assert!(dragon_tile.is_honor());
            }
        }
    }
    mod build_tile_tests {
        use crate::{
            tile,
            tile::{DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*},
            Direction::*,
        };

        #[test]
        fn build_valid_character_tile() {
            for num in 0..=9 {
                let build_string = format!("{num}m");
                let tile = tile::build(&build_string).expect("Tile should be valid");
                match tile {
                    Number(Character(n)) => assert_eq!(n, num),
                    Number(Bamboo(_)) => {
                        panic!("Created a bamboo tile when a character tile was expected")
                    }
                    Number(Dot(_)) => {
                        panic!("Created a dot tile when a character tile was expected")
                    }
                    Honor(Dragon(_)) => {
                        panic!("Created a dragon tile when a character tile was expected")
                    }
                    Honor(Wind(_)) => {
                        panic!("Created a wind tile when a character tile was expected")
                    }
                }
            }
        }

        #[test]
        fn build_valid_bamboo_tile() {
            for num in 0..=9 {
                let build_string = format!("{num}s");
                let tile = tile::build(&build_string).expect("Tile should be valid");
                match tile {
                    Number(Bamboo(n)) => assert_eq!(n, num),
                    Number(Character(_)) => {
                        panic!("Created a character tile when a bamboo tile was expected")
                    }
                    Number(Dot(_)) => {
                        panic!("Created a dot tile when a bamboo tile was expected")
                    }
                    Honor(Dragon(_)) => {
                        panic!("Created a dragon tile when a bamboo tile was expected")
                    }
                    Honor(Wind(_)) => {
                        panic!("Created a wind tile when a bamboo tile was expected")
                    }
                }
            }
        }

        #[test]
        fn build_valid_dot_tile() {
            for num in 0..=9 {
                let build_string = format!("{num}p");
                let tile = tile::build(&build_string).expect("Tile should be valid");
                match tile {
                    Number(Dot(n)) => assert_eq!(n, num),
                    Number(Character(_)) => {
                        panic!("Created a character tile when a dot tile was expected")
                    }
                    Number(Bamboo(_)) => {
                        panic!("Created a bamboo tile when a dot tile was expected")
                    }
                    Honor(Dragon(_)) => {
                        panic!("Created a dragon tile when a dot tile was expected")
                    }
                    Honor(Wind(_)) => {
                        panic!("Created a wind tile when a dot tile was expected")
                    }
                }
            }
        }

        #[test]
        fn build_valid_wind_tile() {
            let directions = [East, South, West, North];
            for num in 1..=4 {
                let build_string = format!("{num}z");
                let tile = tile::build(&build_string).expect("Tile should be valid");
                match tile {
                    Number(Dot(_)) => {
                        panic!("Created a dot tile when a wind tile was expected")
                    }
                    Number(Character(_)) => {
                        panic!("Created a character tile when a wind tile was expected")
                    }
                    Number(Bamboo(_)) => {
                        panic!("Created a bamboo tile when a wind tile was expected")
                    }
                    Honor(Dragon(_)) => {
                        panic!("Created a dragon tile when a wind tile was expected")
                    }
                    Honor(Wind(dir)) => {
                        assert_eq!(directions[num - 1], dir);
                    }
                }
            }
        }

        #[test]
        fn build_valid_dragon_tile() {
            let colors = [White, Green, Red];
            for num in 5..=7 {
                let build_string = format!("{num}z");
                let tile = tile::build(&build_string).expect("Tile should be valid");
                match tile {
                    Number(Dot(_)) => {
                        panic!("Created a dot tile when a dragon tile was expected")
                    }
                    Number(Character(_)) => {
                        panic!("Created a character tile when a dragon tile was expected")
                    }
                    Number(Bamboo(_)) => {
                        panic!("Created a bamboo tile when a dragon tile was expected")
                    }
                    Honor(Wind(_)) => {
                        panic!("Created a wind tile when a dragon tile was expected")
                    }
                    Honor(Dragon(color)) => {
                        assert_eq!(colors[num - 5], color);
                    }
                }
            }
        }

        #[test]
        fn build_invalid_honor_tile() {
            for num in [0, 8, 9] {
                let build_string = format!("{num}z");
                let error_message = tile::build(&build_string).expect_err("Tile should be invalid");
                assert_eq!(error_message, "Invalid number for tile");
            }
        }

        #[test]
        fn not_enough_characters() {
            let invalid_build_strings = ["", "a", "2"];
            for build_string in invalid_build_strings {
                let error_message = tile::build(&build_string).expect_err("Tile should be invalid");
                assert_eq!(error_message, "Invalid number of characters");
            }
        }

        #[test]
        fn too_many_characters() {
            let invalid_build_strings = ["sdafkja", "1zz", "22asdf"];
            for build_string in invalid_build_strings {
                let error_message = tile::build(&build_string).expect_err("Tile should be invalid");
                assert_eq!(error_message, "Invalid number of characters");
            }
        }

        #[test]
        fn first_character_not_digit() {
            let invalid_build_strings = ["aa", "ss", "gz", "$s"];
            for build_string in invalid_build_strings {
                let error_message = tile::build(&build_string).expect_err("Tile should be invalid");
                assert_eq!(error_message, "First character is not a number");
            }
        }

        #[test]
        fn invalid_suit() {
            let invalid_build_strings = ["1a", "3b", "4c"];
            for build_string in invalid_build_strings {
                let error_message = tile::build(&build_string).expect_err("Tile should be invalid");
                assert_eq!(error_message, "Invalid suit");
            }
        }
    }
    mod get_number_tests {
        use crate::{
            tile::{DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*, Tile},
            Direction::*,
        };

        fn test_with_pairs<T>(tile_number_pairs: Vec<(T, u8)>)
        where
            T: Tile,
        {
            for (tile, num) in tile_number_pairs {
                assert_eq!(tile.number(), num);
            }
        }

        #[test]
        fn get_number_mahjong_tile() {
            let tile_number_pairs = vec![
                (Number(Dot(0)), 0),
                (Number(Bamboo(3)), 3),
                (Number(Character(8)), 8),
                (Honor(Wind(East)), 1),
                (Honor(Dragon(Red)), 7),
            ];
            test_with_pairs(tile_number_pairs);
        }

        #[test]
        fn get_number_character_tile() {
            for num in 0..=9 {
                let character_tile = Character(num);
                assert_eq!(character_tile.number(), num);
            }
        }

        #[test]
        fn get_number_dot_tile() {
            for num in 0..=9 {
                let dot_tile = Dot(num);
                assert_eq!(dot_tile.number(), num);
            }
        }

        #[test]
        fn get_number_bamboo_tile() {
            for num in 0..=9 {
                let bamboo_tile = Bamboo(num);
                assert_eq!(bamboo_tile.number(), num);
            }
        }

        #[test]
        fn get_number_wind_tile() {
            let tile_number_pairs = vec![
                (Wind(East), 1),
                (Wind(South), 2),
                (Wind(West), 3),
                (Wind(North), 4),
            ];
            test_with_pairs(tile_number_pairs);
        }

        #[test]
        fn get_number_dragon_tile() {
            let tile_number_pairs = vec![(Dragon(White), 5), (Dragon(Green), 6), (Dragon(Red), 7)];
            test_with_pairs(tile_number_pairs);
        }
    }
    mod get_suit_tests {
        use crate::{
            tile::{DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*, Tile},
            Direction::*,
        };

        fn test_with_pairs<T>(tile_number_pairs: Vec<(T, char)>)
        where
            T: Tile,
        {
            for (tile, suit) in tile_number_pairs {
                assert_eq!(tile.suit(), suit);
            }
        }

        #[test]
        fn get_number_mahjong_tile() {
            let tile_number_pairs = vec![
                (Number(Dot(0)), 'p'),
                (Number(Bamboo(3)), 's'),
                (Number(Character(8)), 'm'),
                (Honor(Wind(East)), 'z'),
                (Honor(Dragon(Red)), 'z'),
            ];
            test_with_pairs(tile_number_pairs);
        }

        #[test]
        fn get_suit_character_tile() {
            let character_suit = 'm';
            for num in 0..=9 {
                let character_tile = Character(num);
                assert_eq!(character_suit, character_tile.suit());
            }
        }
        #[test]
        fn get_suit_bamboo_tile() {
            let bamboo_suit = 's';
            for num in 0..=9 {
                let bamboo_tile = Bamboo(num);
                assert_eq!(bamboo_suit, bamboo_tile.suit());
            }
        }

        #[test]
        fn get_suit_dot_tile() {
            let dot_suit = 'p';
            for num in 0..=9 {
                let dot_tile = Dot(num);
                assert_eq!(dot_suit, dot_tile.suit());
            }
        }

        #[test]
        fn get_suit_dragon_tile() {
            let dragon_suit = 'z';
            let dragon_colors = [White, Green, Red];
            for dragon_color in dragon_colors {
                let dragon_tile = Dragon(dragon_color);
                assert_eq!(dragon_suit, dragon_tile.suit());
            }
        }

        #[test]
        fn get_suit_wind_tile() {
            let wind_suit = 'z';
            let wind_directions = [East, South, West, North];
            for wind_direction in wind_directions {
                let wind_tile = Wind(wind_direction);
                assert_eq!(wind_suit, wind_tile.suit());
            }
        }
    }
    mod format_display_tests {
        use crate::{
            tile::{DragonColor::*, HonorTile::*, MahjongTile::*, NumberTile::*},
            Direction::*,
        };

        #[test]
        fn mahjong_tiles() {
            let dot_tile = Number(Dot(0));
            assert_eq!(format!("{}", dot_tile), "0p");
            let bamboo_tile = Number(Bamboo(3));
            assert_eq!(format!("{}", bamboo_tile), "3s");
            let character_tile = Number(Character(8));
            assert_eq!(format!("{}", character_tile), "8m");
            let wind_tile = Honor(Wind(East));
            assert_eq!(format!("{}", wind_tile), "1z");
            let dragon_tile = Honor(Dragon(Red));
            assert_eq!(format!("{}", dragon_tile), "7z");
        }

        #[test]
        fn character_tiles() {
            for num in 0..=9 {
                let tile_string = format!("{}", Character(num));
                let target_string = format!("{}m", num);
                assert_eq!(tile_string, target_string);
            }
        }
        #[test]
        fn bamboo_tiles() {
            for num in 0..=9 {
                let tile_string = format!("{}", Bamboo(num));
                let target_string = format!("{}s", num);
                assert_eq!(tile_string, target_string);
            }
        }
        #[test]
        fn dot_tiles() {
            for num in 0..=9 {
                let tile_string = format!("{}", Dot(num));
                let target_string = format!("{}p", num);
                assert_eq!(tile_string, target_string);
            }
        }

        #[test]
        fn wind_tiles() {
            assert_eq!(format!("{}", Wind(East)), "1z");
            assert_eq!(format!("{}", Wind(South)), "2z");
            assert_eq!(format!("{}", Wind(West)), "3z");
            assert_eq!(format!("{}", Wind(North)), "4z");
        }

        #[test]
        fn dragon_tiles() {
            assert_eq!(format!("{}", Dragon(White)), "5z");
            assert_eq!(format!("{}", Dragon(Green)), "6z");
            assert_eq!(format!("{}", Dragon(Red)), "7z");
        }
    }
}
