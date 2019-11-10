fn lucy_hold_the_football( football: &String ){
    println!("Lucy hold the {}", football);
}

fn charlie_kick_the_football( football: String ){
    println!("Carlie Brown kick the {}", football);
}

fn main() {
    let football : String = "FootBall".to_string();
    lucy_hold_the_football(&football);
    charlie_kick_the_football(football);
}
