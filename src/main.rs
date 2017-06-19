fn main() {
    let player = Player {name: "Devmane144", health: 100, attack: 5, stamina: 100, dexterity: 100, mana: 100, vitality: 100, defense: 100, points: 8};
    display_stats(&player);
    level_up(&player);
}
fn level_up(player: &Player) {
    println!("Level up points to spend: {}", player.points)
}

fn display_stats(player: &Player) {
    println!("Name:      {}", &player.name);
    println!("Health:    {}", &player.health);
    println!("Attack:    {}", &player.attack);
    println!("Stamina:   {}", &player.stamina);
    println!("Dexterity: {}", &player.dexterity);
    println!("Mana:      {}", &player.mana);
    println!("Vitality:  {}", &player.vitality);
    println!("Defense    {}", &player.defense);
    println!("Points:    {}", &player.points);
}

struct Player<'a> {
    name: &'a str,
    health: u8,
    attack: u8,
    stamina: u8,
    dexterity: u8,
    mana: u8,
    vitality: u8,
    defense: u8,
    points: u8,
}
