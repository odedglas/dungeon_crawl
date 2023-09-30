use crate::prelude::TurnState::AwaitingInput;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

impl TurnState {
    pub fn transition(self) -> Self {
        match self {
            TurnState::AwaitingInput => AwaitingInput,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
        }
    }

    pub fn next(&mut self) {
        *self = match self {
            TurnState::AwaitingInput => TurnState::PlayerTurn,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
        }
    }
}
