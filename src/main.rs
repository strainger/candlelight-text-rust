mod entity;
use entity::Entity;
use entity::EntityFunctionality;
fn main() {
    let mut player = Entity { name: "Player 1", ..Default::default() };
    player.level_up();
    player.display_all_stats();
}

