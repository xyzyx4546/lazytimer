use rand::Rng;

#[derive(Clone)]
enum Moves {
    R,
    L,
    U,
    D,
    F,
    B,
}

#[derive(Clone)]
enum Modifiers {
    None,
    Prime,
    Double,
}

#[derive(Clone)]
pub struct Scramble {
    moves: Vec<(Moves, Modifiers)>,
}

impl Scramble {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let moves = (0..20)
            .map(|_| {
                let mv = match rng.random_range(0..6) {
                    0 => Moves::R,
                    1 => Moves::L,
                    2 => Moves::U,
                    3 => Moves::D,
                    4 => Moves::F,
                    _ => Moves::B,
                };
                let modifier = match rng.random_range(0..3) {
                    0 => Modifiers::None,
                    1 => Modifiers::Prime,
                    _ => Modifiers::Double,
                };
                (mv, modifier)
            })
            .collect();
        Scramble { moves }
    }

    pub fn to_string(&self) -> String {
        self.moves
            .iter()
            .map(|(mv, modifier)| {
                let mv_str = match mv {
                    Moves::R => "R",
                    Moves::L => "L",
                    Moves::U => "U",
                    Moves::D => "D",
                    Moves::F => "F",
                    Moves::B => "B",
                };
                let modifier_str = match modifier {
                    Modifiers::None => "",
                    Modifiers::Prime => "'",
                    Modifiers::Double => "2",
                };
                format!("{}{}", mv_str, modifier_str)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
