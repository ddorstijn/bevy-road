use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
           .add_state::<GameState>()
           .add_systems(Update, change_mode.run_if(input_just_released(KeyCode::Escape)));
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Simulating,
    Building
}

fn change_mode(state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    match state.get() {
        GameState::Building => next_state.set(GameState::Simulating),
        GameState::Simulating => next_state.set(GameState::Building)
    }
}