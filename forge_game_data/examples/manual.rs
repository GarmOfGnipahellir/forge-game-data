use forge_game_data::Entity;

struct WorldSpawn;

impl Entity for WorldSpawn {
    fn name() -> &'static str {
        "WorldSpawn"
    }
    fn display_name() -> &'static str {
        "World entity"
    }
}

fn main() {
    println!("{}", WorldSpawn::display_name());
}
