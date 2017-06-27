mod player;
use player::Player;
use player::PlayerFunctionality;
fn main() {
    let mut plyr = Player { name: "Playa", ..Default::default() };
    plyr.level_up();
    plyr.display_all_stats();
}

