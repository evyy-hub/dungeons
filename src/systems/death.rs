use bevy::prelude::*;

#[derive(Event)]
pub struct DeathEvent {
    pub target: Entity,
}

pub fn death_system(trigger: On<DeathEvent>, mut commands: Commands) {
    let entity = trigger.event().target;
    commands.entity(entity).despawn();
}
