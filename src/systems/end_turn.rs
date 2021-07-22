use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        &mut TurnState::AwaitingInput => return,
        &mut TurnState::PlayerTurn => TurnState::MonsterTurn,
        &mut TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn_state = new_state;
}
