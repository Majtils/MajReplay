//! This module provides types to represent a Riichi Mahjong game

use core::fmt;

use crate::{Direction, MahjongTile};
use chrono::{DateTime, Local};

/// The Game struct combines metadata information about the game and all rounds that have occurred
/// to hold all the information about a game of Riichi Mahjong
#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub game_config: GameConfig,
    pub rounds: Vec<Round>,
}

/// The GameConfig type holds all the metadata information about a Riichi Mahjong game
#[derive(Debug, PartialEq, Eq)]
pub struct GameConfig {
    /// This represents the number of players in the game of Riichi Mahjong, either three or four.
    /// By default, this is set to `NumPlayers::Four`
    pub num_players: NumPlayers,
    /// This represents the number of cycles of rounds to play at minimum. East games last one
    /// cycle minimum and West games last two cycles minimum. By default, this is set to
    /// `Some(Length::East)`
    pub length: Option<Length>,
    /// This represents the number of seconds given before a tile must be played. This only starts
    /// to count down once the delay time is over. If this time runs out, tiles are auto-discarded.
    /// At the start of every round, the thinking time is rest to this number. By default this is
    /// set to `Some(Seconds(20))`
    pub main_thinking_time: Option<Seconds>,
    /// This represents the number of seconds allowed before the main thinking time begins to tick
    /// down. This is reset during each action. By default, this is set to `Some(Seconds(5))`.
    pub delay_thinking_time: Option<Seconds>,
    /// This represents the number of red fives in the tile set. Each red five is a dora. By
    /// default, this is set to `Some(RedFive::Three)`.
    pub red_five: Option<RedFive>,
    /// This represents the name of the hero, the player whose perspective you see. By default,
    /// this is set to `"player1"`.
    pub hero: String,
    /// This represents the name of the player to the right of the hero. By default, this is set to
    /// `"player2"`.
    pub right: String,
    /// This represents the name of the player across the hero. By default, this is set to `"player3"`.
    pub across: String,
    /// This represents the name of the player to the left of the hero. By default, this is set to
    /// `"player4"`.
    pub left: String,
    /// This represents the event the game took place during. By default, this is set to `None`
    pub event: Option<String>,
    /// This represents the location the game took place at. By default, this is set to `None`
    pub site: Option<String>,
    /// This represents the date the game took place. By default, this is set to `None`.
    pub date: Option<DateTime<Local>>,
    /// This represents the result of the game as an array of player and point tuples sorted from
    /// most to least points. By default, this is set to `None`.
    pub result: Option<[(PlayerLocation, u32); 4]>,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            num_players: NumPlayers::Four,
            length: Some(Length::East),
            main_thinking_time: Some(Seconds(20)),
            delay_thinking_time: Some(Seconds(5)),
            red_five: Some(RedFive::Three),
            hero: "player1".to_string(),
            right: "player2".to_string(),
            across: "player3".to_string(),
            left: "player4".to_string(),
            event: None,
            site: None,
            date: None,
            result: None,
        }
    }
}

/// The Round struct combines information about the round metadata and all round events to hold all details about
/// a particular Riichi Mahjong round.
#[derive(Debug, PartialEq, Eq)]
pub struct Round {
    pub round_config: RoundConfig,
    /// Indicates the events in the round in chronological order
    pub game_events: Vec<RoundEvent>,
}

/// The RoundConfig struct holds metadata about the round being played.
#[derive(Debug, PartialEq, Eq)]
pub struct RoundConfig {
    /// Indicates the direction of the round wind.
    pub round_wind: Direction,
    /// Indicates the number in the round.
    pub round_number: RoundNumber,
    /// Indicates how many repeats we are on for the current direction and number
    pub round_repeat: u8,
    /// Indicates the seat of the hero
    pub hero_location: Direction,
    /// Indicates the initial hand state of the hero
    pub initial_hero_hand_state: Hand,
    /// This represents the result of the round as an array of player and point tuples sorted from
    /// most to least points. By default, this is set to `None`.
    pub result: Option<[(PlayerLocation, u32); 4]>,
    /// Indicates the dora tiles for the round
    pub dora: Vec<MahjongTile>,
    /// Indicates the ura dora tiles for the round
    pub ura_dora: Vec<MahjongTile>,
}

/// The RoundNumber enum indicates which of the four rounds within the cycle it is
#[derive(Debug, PartialEq, Eq)]
pub enum RoundNumber {
    One,
    Two,
    Three,
    Four,
}

/// The RoundEvent type wraps a RoundAction and provides context on which player performed the action
/// and which player is the target of the action
#[derive(Debug, PartialEq, Eq)]
pub struct RoundEvent {
    pub subject: PlayerLocation,
    pub action: RoundAction,
    pub target: Option<PlayerLocation>,
}

/// The RoundAction lists out all the possible actions that can occur in a Mahjong round
#[derive(Debug, PartialEq, Eq)]
pub enum RoundAction {
    /// A player draws a tile, the None option represents a player who draws a tile we don't know
    /// about
    Draw(Option<MahjongTile>),
    /// A player discards a tile
    Discard(MahjongTile),
    /// A player takes a discarded tile from the player to their left and completes a sequence
    Chii(ChiiMeld),
    /// A player takes a discarded tile and completes a triplet of three of the same tile
    Pon(PonMeld),
    /// A player takes a tile in hand and completes a quadruple
    ClosedKan(ClosedKanMeld),
    /// A player takes a discarded tile and complets a quadruple
    OpenKan(OpenKanMeld),
    /// A player takes a draw tile and it adds it to an existing Pon triplet
    AddedOpenKan(AddedOpenKanMeld),
    /// A player places a bet indicating their hand is in tenpai
    Richii,
    /// A player completes a hand by drawing the tile
    Tsumo(Hand),
    /// A player completes a hand due to another player's discarded tile
    Ron(Hand),
    /// A round finishes due to exhaustive draw
    Exhaustive(Vec<(PlayerLocation, Hand)>),
}

/// This type represents a Mahjong hand. This can be a combination of tiles in hand and open melded
/// tiles displayed to all players
#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub hand: Vec<MahjongTile>,
    pub melds: Vec<Meld>,
}

/// This type represents a meld, which is a completed triplet, quadruplet, or sequence. There are
/// either open melds resulting from a chii, pon, open kan, or added open kan, or closed melds
/// resulting from a closed kan
#[derive(Debug, PartialEq, Eq)]
pub enum Meld {
    Chii(ChiiMeld),
    Pon(PonMeld),
    OpenKan(OpenKanMeld),
    AddedOpenKan(AddedOpenKanMeld),
    ClosedKan(ClosedKanMeld),
}

/// This type represents a meld formed by a chii. It is defined by the three tiles in the chii, the
/// tile that was chiied, and the player it was chiied from
#[derive(Debug, PartialEq, Eq)]
pub struct ChiiMeld {
    pub tiles: [MahjongTile; 3],
    pub chii_tile: MahjongTile,
    pub source: PlayerLocation,
}

/// This represents a pon meld. It is defined by the tile repeated and where the third tile was
/// pon'ed from
#[derive(Debug, PartialEq, Eq)]
pub struct PonMeld {
    pub tile: MahjongTile,
    pub source: PlayerLocation,
}

/// This represents an open kan meld. It is defined by the tile repeated and where the fourth tile
/// was pon'ed from.
#[derive(Debug, PartialEq, Eq)]
pub struct OpenKanMeld {
    pub tile: MahjongTile,
    pub source: PlayerLocation,
}

/// This represents an added open kan meld. It is defined by the tile repeated and where the tile
/// was originally pon'ed from
#[derive(Debug, PartialEq, Eq)]
pub struct AddedOpenKanMeld {
    pub tile: MahjongTile,
    pub source: PlayerLocation,
}

/// This represents a closed kan meld. This is defined by the tile repeated in the quadruplet
#[derive(Debug, PartialEq, Eq)]
pub struct ClosedKanMeld {
    pub tile: MahjongTile,
}

impl fmt::Display for ClosedKanMeld {
    // TODO: Implement
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// This represents the number of players in a Riichi Mahjong game, which changes the game rules.
/// There are only two options, either three-player or four-player mahjong.
#[derive(Debug, PartialEq, Eq)]
pub enum NumPlayers {
    Three,
    Four,
}

/// This type represents the length of the game. An east game lasts at least one cycle and a south
/// game lasts at least two cycles.
#[derive(Debug, PartialEq, Eq)]
pub enum Length {
    East,
    South,
}

/// This newtype holds a time in terms of seconds
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seconds(pub u32);

/// This type represents the number of red fives that can be in a game of Riichi. These count as
/// dora tiles.
#[derive(Debug, PartialEq, Eq)]
pub enum RedFive {
    Zero,
    Three,
    Four,
}

/// This type represents locations in relation to the hero, which is the player the game is being
/// viewed from.
#[derive(Debug, PartialEq, Eq)]
pub enum PlayerLocation {
    Hero,
    Right,
    Across,
    Left,
}

impl PlayerLocation {
    pub fn move_relative(&self, location: &PlayerLocation) -> PlayerLocation {
        use crate::PlayerLocation::*;
        fn to_int(player_location: &PlayerLocation) -> i32 {
            match player_location {
                Hero => 0,
                Right => 1,
                Across => 2,
                Left => 3,
            }
        }
        let result = (to_int(self) + to_int(location)) % 4;
        match result {
            0 => Hero,
            1 => Right,
            2 => Across,
            3 => Left,
            _ => panic!("This case is not possible due to the modulo operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    mod default_tests {
        #[test]
        fn game_config() {
            use crate::{GameConfig, Length, NumPlayers, RedFive, Seconds};
            let game_config: GameConfig = Default::default();
            assert_eq!(game_config.num_players, NumPlayers::Four);
            assert_eq!(game_config.length, Some(Length::East));
            assert_eq!(game_config.main_thinking_time, Some(Seconds(20)));
            assert_eq!(game_config.delay_thinking_time, Some(Seconds(5)));
            assert_eq!(game_config.red_five, Some(RedFive::Three));
            assert_eq!(game_config.hero, "player1");
            assert_eq!(game_config.right, "player2");
            assert_eq!(game_config.across, "player3");
            assert_eq!(game_config.left, "player4");
            assert!(game_config.event.is_none());
            assert!(game_config.site.is_none());
            assert!(game_config.date.is_none());
        }
    }
    mod display_tests {
        #[test]
        fn closed_kan_meld() {
            // TODO: Add additional tests
            use crate::{
                ClosedKanMeld, Direction::*, DragonColor::*, HonorTile::*, MahjongTile,
                NumberTile::*,
            };
            let character_5 = MahjongTile::Number(Character(5));
            let character_5_closed_kan_meld = ClosedKanMeld { tile: character_5 };
            assert_eq!(character_5_closed_kan_meld.to_string(), "--0m5m--")
        }
    }
    mod move_relative_location_test {
        use crate::game::PlayerLocation::*;
        #[test]
        fn move_relative_from_hero() {
            let player = Hero;
            assert_eq!(player.move_relative(&Hero), Hero);
            assert_eq!(player.move_relative(&Left), Left);
            assert_eq!(player.move_relative(&Right), Right);
            assert_eq!(player.move_relative(&Across), Across);
        }

        #[test]
        fn move_relative_from_left() {
            let player = Left;
            assert_eq!(player.move_relative(&Hero), Left);
            assert_eq!(player.move_relative(&Left), Across);
            assert_eq!(player.move_relative(&Right), Hero);
            assert_eq!(player.move_relative(&Across), Right);
        }

        #[test]
        fn move_relative_from_right() {
            let player = Right;
            assert_eq!(player.move_relative(&Hero), Right);
            assert_eq!(player.move_relative(&Left), Hero);
            assert_eq!(player.move_relative(&Right), Across);
            assert_eq!(player.move_relative(&Across), Left);
        }

        #[test]
        fn move_relative_from_across() {
            let player = Across;
            assert_eq!(player.move_relative(&Hero), Across);
            assert_eq!(player.move_relative(&Left), Right);
            assert_eq!(player.move_relative(&Right), Left);
            assert_eq!(player.move_relative(&Across), Hero);
        }
    }
}
