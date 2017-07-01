mod entity;
use entity::Entity;
use entity::EntityFunctionality;
fn main() {
    let mut plyr = Entity { name: "Player 1", ..Default::default() };
    plyr.level_up();
    plyr.display_all_stats();
}

