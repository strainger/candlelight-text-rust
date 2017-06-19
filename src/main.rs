use std::io;

fn main() {
    let mut player = Player {name: "DragonMaster385", ..Default::default()};
    player.level_up();
    player.display_all_stats();
}

struct Player<'a> {
    name: &'a str,
    health: i8,
    attack: i8,
    stamina: i8,
    dexterity: i8,
    mana: i8,
    vitality: i8,
    defense: i8,
    points: i8,
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
    fn display_all_stats(&self);
    fn display_core_stats(&self);
    fn level_up(&mut self);
}

impl<'a> PlayerFunctionality for Player<'a> {
    fn display_all_stats(&self) {
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
            println!("Level up points to spend: {}", &self.points);
            &self.display_core_stats();
            println!("Stat to level up? (1-7)");
            loop {
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

                              1 => {
                                  println!("Increase Health stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.health = &self.health + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }

                              2 => {
                                  println!("Increase Attack stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.attack = &self.attack + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }


                              3 => {
                                  println!("Increase Stamina stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.stamina = &self.stamina + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }


                              4 => {
                                  println!("Increase Dexterity stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.dexterity = &self.dexterity + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }

                              5 => {
                                  println!("Increase Mana stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.mana = &self.mana + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }

                              6 => {
                                  println!("Increase Vitality stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.vitality = &self.vitality + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }

                              7 => {
                                  println!("Increase Defense stat by how much?");
                                  loop {
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
                                                    if &input <= &self.points {
                                                        self.defense = &self.defense + &input;
                                                        self.points = &self.points - &input;
                                                        break;
                                                    } else {
                                                        println!("You can't spend points that you don't have.")
                                                    }
                                                }
                                          Err(_) => {
                                            println!("Can't parse {}", &input);
                                            continue;
                                          }
                                      }
                                  }
                                  break;
                              }
                              _ => {
                                  println!("{} is not a valid number (1-7)", &input);
                                  continue;
                              }
                          }
                      }

                      Err(_) => {
                        println!("Can't parse {}", &input);
                        continue;
                      }
                };
            }
        }
    }
}
