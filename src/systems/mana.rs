use bevy::prelude::*;

#[derive(Component)]
pub struct Mana {
    current: f32,
    max: f32,
}

impl Mana {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn get_current(&self) -> f32 {
        self.current
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }

    pub fn add_mana(&mut self, amount: f32) {
        if self.current + amount > self.max {
            self.current = self.max;
        } else {
            self.current += amount;
        }
    }

    pub fn remove_mana(&mut self, amount: f32) {
        if self.current - amount < 0.0 {
            self.current = 0.0;
        } else {
            self.current -= amount;
        }
    }
}
