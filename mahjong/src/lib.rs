//! This crate provides types to represent informatin about a Riichi Mahjong game
//! The tile module provides information to create and access information about
//! the tiles used in the game.

pub mod tile;

/// This type represents the directions in a game of Riichi Mahjong. This is
/// applicable to both categorizing wind tiles and also the seats of players
/// and rounds.
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    East,
    South,
    West,
    North,
}
