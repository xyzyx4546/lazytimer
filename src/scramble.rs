use rand::prelude::IndexedRandom;

use crate::{app::App, sessions::PuzzleType};

impl PuzzleType {
    fn get_base_moves(&self) -> Vec<&'static str> {
        match self {
            PuzzleType::TwoByTwo => vec!["R", "U", "F"],
            PuzzleType::ThreeByThree => vec!["R", "L", "U", "D", "F", "B"],
            PuzzleType::Skewb => vec!["R", "L", "U", "B"],
        }
    }

    fn get_modifiers(&self) -> Vec<&'static str> {
        match self {
            PuzzleType::TwoByTwo | PuzzleType::ThreeByThree => vec!["", "'", "2"],
            PuzzleType::Skewb => vec!["", "'"],
        }
    }

    fn get_scramble_length(&self) -> usize {
        match self {
            PuzzleType::TwoByTwo => 10,
            PuzzleType::ThreeByThree => 20,
            PuzzleType::Skewb => 10,
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

            let base = *base_moves.choose(&mut rng).unwrap();
            let modifier = *modifiers.choose(&mut rng).unwrap();

            scramble.push(format!("{}{}", base, modifier));
            last_move = Some(base);
        }
        
        self.current_scramble = scramble.join(" ");
    }
}
