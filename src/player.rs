use std::io;
pub struct Player<'a> {
    pub name: &'a str,
    pub health: i8,
    pub attack: i8,
    pub stamina: i8,
    pub dexterity: i8,
    pub mana: i8,
    pub vitality: i8,
    pub defense: i8,
    pub experience: i8,
    pub points: i8,
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
          experience: 0,
          points: 8}
    }
}

pub trait PlayerFunctionality {
    fn display_all_stats(&self);
    fn display_core_stats(&self);
    fn level_up(&mut self);
}

impl<'a> PlayerFunctionality for Player<'a> {
    fn display_all_stats(&self) {
        println!("Name:       {}", &self.name);
        println!("Health:     {}", &self.health);
        println!("Attack:     {}", &self.attack);
        println!("Stamina:    {}", &self.stamina);
        println!("Dexterity:  {}", &self.dexterity);
        println!("Mana:       {}", &self.mana);
        println!("Vitality:   {}", &self.vitality);
        println!("Defense     {}", &self.defense);
        println!("Experience: {}", &self.experience);
        println!("Points:     {}", &self.points);
    }
    fn display_core_stats(&self) {
        println!("1. Health:    {}", &self.health);
        println!("2. Attack:    {}", &self.attack);
        println!("3. Stamina:   {}", &self.stamina);
        println!("4. Dexterity: {}", &self.dexterity);
        println!("5. Mana:      {}", &self.mana);
        println!("6. Vitality:  {}", &self.vitality);
        println!("7. Defense    {}", &self.defense);
    }
    fn level_up(&mut self) {
        if &self.points == &0 {
            println!("You don't have any point's to spend leveling up.");
        } else {
            loop {
            println!("Level up points to spend: {}", &self.points);
            &self.display_core_stats();
            println!("Enter 0 to exit.");
            println!("Stat to level up? (1-7)");
            let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                      Ok(_)   => {
                          let input: i8 = match input.trim().parse() {
                              Ok(i)  => i,
                              Err(_) => {
                                  println!("Can't parse {}", &input);
                                  continue;
                              }
                          };
                          match input {
                              0 => break,
                              1...7 => {
                                  println!("You have {} points to spend.", &self.points);
                                  println!("Increase {} stat by how much?", match input {
                                      1 => "Health",
                                      2 => "Attack",
                                      3 => "Stamina",
                                      4 => "Dexterity",
                                      5 => "Mana",
                                      6 => "Vitality",
                                      7 => "Defense",
                                      _ => "Unknown",
                                  });
                                  loop {
                                      let mut point_amount = String::new();
                                          match io::stdin().read_line(&mut point_amount) {
                                              Ok(_)   => {
                                                  let point_amount: i8 = match point_amount.trim().parse() {
                                                      Ok(i)  => i,
                                                      Err(_) => {
                                                          println!("Can't parse {}", &point_amount);
                                                          continue;
                                                      }
                                                  };
                                                  if &point_amount <= &self.points && &point_amount >= &0 {
                                                      match input {
                                                          1 => self.health = &self.health + &point_amount,
                                                          2 => self.attack = &self.attack + &point_amount,
                                                          3 => self.stamina = &self.stamina + &point_amount,
                                                          4 => self.dexterity = &self.dexterity + &point_amount,
                                                          5 => self.mana = &self.mana + &point_amount,
                                                          6 => self.vitality = &self.vitality + &point_amount,
                                                          7 => self.defense = &self.defense + &point_amount,
                                                          _ => continue,
                                                      }
                                                      self.points = &self.points - &point_amount;
                                                      break;
                                                  } else {
                                                      println!("You have {} point(s), you can't spend {}. Sorry.", &self.points, &point_amount);
                                                  }
                                              }
                                          Err(_) => {
                                              println!("Can't parse {}", &point_amount);
                                          }
                                      }
                                  }
                              }
                              _ => {
                                  println!("{} is not a valid input (1-7)", &input);
                              }
                          }
                      }
                      Err(_) => {
                          println!("Can't parse {}", &input);
                      }
                };
            }
        }
    }
}
