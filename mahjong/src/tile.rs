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
    /// use mahjong::tile::{NumberTile, HonorTile, DragonColor};
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
    /// use mahjong::tile::{NumberTile, HonorTile, DragonColor};
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

    /// The build function takes in a string representation of a tile such as
    /// 1z or 3p and converts into the corresponding White Dragon or 3 dots
    /// tile type.
    ///
    /// If successful, it returns an Ok with a type that implements the Tile trait.
    /// Otherwise, it errors with a String error message.
    /// # Errors
    /// The function expects a string of length 2, a number and then a character
    /// If there are more than 2 letters or the string is not a valid mahjong tile,
    /// an error is returned with a String message.
    /// # Examples
    /// ```rust
    /// use mahjong::tile::Tile;
    /// let three_dot_tile = Tile::build("3p");
    /// assert!(three_dot_tile.is_ok());
    ///
    /// let too_many_letters = Tile::build("3pp");
    /// assert!(too_many_letters.is_err());
    ///
    /// let invalid_tile = Tile::build("10z");
    /// assert!(invalid_tile.is_err());
    /// ```
    fn build<T: Tile>(tile_string: &str) -> Result<T, String>;
}

/// This type represents a number tile. There are three suits of number tiles:
/// characters (manzu), dots (pinzu), and bamboo (souzu).
/// The number represents the number of the tile except for a 0 which represents
/// a red five tile of that suit.
pub enum NumberTile {
    /// The character or manzu tiles are written in Chinese/Kanji.
    Character(u8),
    /// The dot or pinzu tiles have a certain number of dots on them.
    Dot(u8),
    /// The bamboo or souzu tiles have a certain number bamboo sticks on them.
    Bamboo(u8),
}

impl Tile for NumberTile {
    fn number(&self) -> u8 {
        use NumberTile::*;
        match *self {
            Character(num) | Dot(num) | Bamboo(num) => num,
        }
    }

    fn suit(&self) -> char {
        'a'
    }

    fn build<T: Tile>(tile_string: &str) -> Result<T, String> {
        Err("".to_string())
    }
}

use crate::Direction;
/// This type represents an honor tile. The honors are broken up to wind tiles
/// and dragon tiles. There are four types of wind tiles and three types of
/// dragon tiles.
pub enum HonorTile {
    /// This represents a wind tile. There are four variants depending on the
    /// direction: north, east, south, west.
    Wind(Direction),

    /// This represents a dragon tile. There are three variants depending on the
    /// color: white, green, red.
    Dragon(DragonColor),
}

impl Tile for HonorTile {
    fn build<T: Tile>(tile_string: &str) -> Result<T, String> {
        Err("".to_string())
    }

    fn suit(&self) -> char {
        'a'
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
pub enum DragonColor {
    White,
    Green,
    Red,
}

#[cfg(test)]
mod tile_tests {
    mod get_number_tests {
        mod number_tile_tests {
            use crate::tile::{NumberTile::*, Tile};
            #[test]
            fn get_number_character_tile() {
                for num in 0..9 {
                    let character_tile = Character(num);
                    assert_eq!(character_tile.number(), num);
                }
            }

            #[test]
            fn get_number_dot_tile() {
                for num in 0..9 {
                    let dot_tile = Dot(num);
                    assert_eq!(dot_tile.number(), num);
                }
            }

            #[test]
            fn get_number_bamboo_tile() {
                for num in 0..9 {
                    let bamboo_tile = Bamboo(num);
                    assert_eq!(bamboo_tile.number(), num);
                }
            }
        }

        mod honor_tile_tests {
            use crate::tile::{DragonColor::*, HonorTile, HonorTile::*, Tile};
            use crate::Direction::*;
            fn test_with_pairs(tile_number_pairs: Vec<(HonorTile, u8)>) {
                for (tile, num) in tile_number_pairs {
                    assert_eq!(tile.number(), num);
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
                let tile_number_pairs =
                    vec![(Dragon(White), 5), (Dragon(Green), 6), (Dragon(Red), 7)];
                test_with_pairs(tile_number_pairs);
            }
        }
    }
    mod get_suit_tests {}
    mod build_tests {}
    mod fmt_tests {}
}
