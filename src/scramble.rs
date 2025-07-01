use rand::prelude::IndexedRandom;

use crate::{app::App, sessions::PuzzleType};

impl PuzzleType {
    fn get_base_moves(&self) -> Vec<&'static str> {
        match self {
            PuzzleType::TwoByTwo => vec!["R", "U", "F"],
            PuzzleType::ThreeByThree => vec!["R", "L", "U", "D", "F", "B"],
            PuzzleType::FourByFour => vec!["R", "L", "U", "D", "F", "B", "Rw", "Lw", "Uw", "Dw", "Fw", "Bw"],
            PuzzleType::FiveByFive => vec!["R", "L", "U", "D", "F", "B", "Rw", "Lw", "Uw", "Dw", "Fw", "Bw"],
            PuzzleType::Skewb => vec!["R", "L", "U", "B"],
            PuzzleType::Pyraminx => vec!["R", "L", "U", "B", "R", "L", "U", "B", "r", "l", "u", "b"],
        }
    }

    fn get_modifiers(&self) -> Vec<&'static str> {
        match self {
            PuzzleType::TwoByTwo | PuzzleType::ThreeByThree | PuzzleType::FourByFour | PuzzleType::FiveByFive => vec!["", "'", "2"],
            PuzzleType::Skewb | PuzzleType::Pyraminx => vec!["", "'"],
        }
    }

    fn get_scramble_length(&self) -> usize {
        match self {
            PuzzleType::TwoByTwo => 10,
            PuzzleType::ThreeByThree => 20,
            PuzzleType::FourByFour => 40,
            PuzzleType::FiveByFive => 60,
            PuzzleType::Skewb => 10,
            PuzzleType::Pyraminx => 10,
        }
    }
}

impl App {
    pub fn next_scramble(&mut self) {
        let puzzle_type = &self.selected_session().puzzle_type;
        let modifiers = puzzle_type.get_modifiers();
        let scramble_length = puzzle_type.get_scramble_length();
        let mut rng = rand::rng();
        let mut scramble = Vec::with_capacity(scramble_length);
        let mut last_move: Option<&str> = None;

        for _ in 0..scramble_length {
            let mut base_moves = puzzle_type.get_base_moves();
            if let Some(prev_move) = last_move {
                base_moves.retain(|&m| m != prev_move);
            }

            let base_move = *base_moves.choose(&mut rng).unwrap();
            let modifier = *modifiers.choose(&mut rng).unwrap();

            scramble.push(format!("{}{}", base_move, modifier));
            last_move = Some(base_move);
        }
        
        self.current_scramble = scramble.join(" ");
    }
}
