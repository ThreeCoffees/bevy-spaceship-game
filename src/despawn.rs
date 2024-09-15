use bevy::prelude::*;

use crate::{schedule::InGameSet, spaceship::Spaceship};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            despawn_far_away_entities.in_set(InGameSet::DespawnEntities),
        );
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
