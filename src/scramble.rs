use rand::Rng;

pub fn generate_scramble() -> String {
    // TODO: implement proper algorithm
    let moves = ["R", "L", "U", "D", "F", "B"];
    let modifiers = ["", "'", "2"];
    let mut rng = rand::rng();
    let mut scramble = Vec::new();

    for _ in 0..20 {
        let mv = moves[rng.random_range(0..moves.len())];
        let modifier = modifiers[rng.random_range(0..modifiers.len())];
        scramble.push(format!("{}{}", mv, modifier));
    }

    scramble.join("  ")
}
