use forge_game_data::Entity;

#[derive(Entity)]
#[entity(
    name = "worldspawn",
    display_name = "World entity",
    properties(base(String, 0 0 0))
)]
struct WorldSpawn;

fn main() {
    println!("{}", WorldSpawn::name());
    println!("{}", WorldSpawn::display_name());
}
