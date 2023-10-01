use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<&Point> for Camera {
    // Creates a centered camera view out of a given player position
    // Ensures he's position is always in the center of the screen
    fn from(player_position: &Point) -> Camera {
        Camera {
            left: player_position.x - DISPLAY_WIDTH / 2,
            right: player_position.x + DISPLAY_WIDTH / 2,
            top: player_position.y - DISPLAY_HEIGHT / 2,
            bottom: player_position.y + DISPLAY_HEIGHT / 2,
        }
    }
}

impl Camera {
    pub fn new(focal_point: Point) -> Self {
        Self::from(&focal_point)
    }

    pub fn center(&mut self, focal_point: Point) {
        *self = Self::from(&focal_point);
    }
}
