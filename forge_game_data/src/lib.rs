mod model;
pub mod parsers;

#[cfg(feature = "derive")]
pub use forge_game_data_derive::Entity;
// pub use model::*;

pub struct Property {
    
}

pub trait Entity {
    fn name() -> &'static str;
    fn display_name() -> &'static str;
}
