#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    GameWon,
}

impl TurnState {
    pub fn transition(self) -> Self {
        match self {
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => self,
        }
    }

    pub fn next(&mut self) {
        *self = match self {
            TurnState::AwaitingInput => TurnState::PlayerTurn,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn | TurnState::GameOver | TurnState::GameWon => TurnState::AwaitingInput,
        }
    }

    pub fn game_over(&mut self) {
        *self = TurnState::GameOver;
    }

    pub fn game_won(&mut self) {
        *self = TurnState::GameWon;
    }
}
