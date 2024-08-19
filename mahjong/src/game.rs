use std::fmt;

struct Config {
    num_players: u8,
    red_fives: RedFives,
    open_tanyao: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            num_players: 4,
            red_fives: RedFives::Four,
            open_tanyao: true,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
Riichi Majong Game Settings:
Number of Players: {}
Red Fives: {}
Open Tanyao: {}",
            self.num_players, self.red_fives, self.open_tanyao
        )
    }
}

enum RedFives {
    None,
    Two,
    Three,
    Four,
}

impl fmt::Display for RedFives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RedFives::None => "None",
                RedFives::Two => "2",
                RedFives::Three => "3",
                RedFives::Four => "4",
            }
        )
    }
}

#[cfg(test)]
mod red_fives_test {
    mod display_test {
        use crate::mahjong::game::RedFives;
        #[test]
        fn display_test_none() {
            assert_eq!("None", RedFives::None.to_string())
        }
        #[test]
        fn display_test_two() {
            assert_eq!("2", RedFives::Two.to_string())
        }
        #[test]
        fn display_test_three() {
            assert_eq!("3", RedFives::Three.to_string())
        }
        #[test]
        fn display_test_four() {
            assert_eq!("4", RedFives::Four.to_string())
        }
    }
}
