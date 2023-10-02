use crate::game::Game;

pub mod sheep;
pub mod the_coin;

pub fn execute_blueprints(game: &mut Game) {
    sheep::blueprint(game);
    the_coin::blueprint(game);
}
