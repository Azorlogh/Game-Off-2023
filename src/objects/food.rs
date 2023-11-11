use bevy::prelude::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
   
}

pub struct Food {
    hydration: f32,
    glucose: f32,
    health: f32,
}

pub fn food_spawn() {
    // spawn the food at a certain location in the game
}

pub fn food_on_eat() {
    // called when the food is being eaten
}
