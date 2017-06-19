fn main() {
    let player = Player {name: "DragonMaster385", ..Default::default()};
    player.display_stats();
    player.level_up();
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

impl<'a> Default for Player<'a> {
    fn default () -> Player<'a> {
      Player {
          name: "Player",
          health: 100, 
          attack: 5,
          stamina: 100, 
          dexterity: 100,
          mana: 100, 
          vitality: 100, 
          defense: 100, 
          points: 8}
    }
}

trait PlayerFunctionality {
    fn display_stats(&self);
    fn level_up(&self);
}

impl<'a> PlayerFunctionality for Player<'a> {
    fn display_stats(&self) {
        println!("Name:      {}", &self.name);
        println!("Health:    {}", &self.health);
        println!("Attack:    {}", &self.attack);
        println!("Stamina:   {}", &self.stamina);
        println!("Dexterity: {}", &self.dexterity);
        println!("Mana:      {}", &self.mana);
        println!("Vitality:  {}", &self.vitality);
        println!("Defense    {}", &self.defense);
        println!("Points:    {}", &self.points);
    }
    fn level_up(&self) {
        println!("Level up points to spend: {}", &self.points)
    }
}
