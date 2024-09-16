use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet, spaceship::Spaceship, state::GameState};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, 
                (
                    despawn_far_away_entities,
                    despawn_dead_enitites,
                )
                .in_set(InGameSet::DespawnEntities),
            )
            .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Spaceship>>,
) {
    for (entity, transfrom) in query.iter() {
        let distance = transfrom.translation().distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        } 
    }
}

fn despawn_dead_enitites(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(
    mut commands: Commands,
    query: Query<Entity, With<Health>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
