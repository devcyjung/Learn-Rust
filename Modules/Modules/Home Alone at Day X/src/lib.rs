use first_floor::toys_room;
use ground_floor::storage_room;
use second_floor::astronomy_tower;

pub mod first_floor;
pub mod ground_floor;
pub mod second_floor;

pub fn day_x() {
    toys_room::i_love_toys();
    storage_room::i_found_telescope();
    astronomy_tower::i_see_the_stars();
}
