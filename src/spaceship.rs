use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, collision_detection::{Collider, CollisionDamage}, health::Health, movement::{Acceleration, MovingObjectBundle, Velocity}, schedule::InGameSet, state::GameState};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_COLLIDER_RADIUS: f32 = 7.5;
const SPACESHIP_HEALTH: f32 = 10.0;
const SPACESHIP_DAMAGE: f32 = 5.0;

const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_OFFSET: f32 = 10.0;
const MISSILE_COLIDER_RADIUS: f32 = 0.5;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_DAMAGE: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, spawn_spaceship)
            .add_systems(OnExit(GameState::GameOver), spawn_spaceship)
            .add_systems(
                Update, 
                spaceship_destroyed.in_set(InGameSet::EntityUpdates),
            )
            .add_systems(
                Update, 
                (
                    spaceship_movement_controls, 
                    spaceship_weapon_controls,
                    spaceship_shield_controls,
                )
                .chain()
                .in_set(InGameSet::UserInput),
            );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::ZERO), 
                acceleration: Acceleration::new(Vec3::ZERO), 
                collider: Collider::new(SPACESHIP_COLLIDER_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.spaceship.clone(),
                    transform: Transform::from_translation(STARTING_TRANSLATION),
                    ..default()
                },
            },
            Spaceship,
            Health::new(SPACESHIP_HEALTH),
            CollisionDamage::new(SPACESHIP_DAMAGE),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::R) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::F) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around Y-axis
    transform.rotate_y(rotation);
    // Rotate around local Z-axis
    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands, 
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
){
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    collider: Collider::new(MISSILE_COLIDER_RADIUS),
                    model: SceneBundle {
                        scene: scene_assets.missiles.clone(),
                        transform: Transform::from_translation(
                            transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_OFFSET,
                        ),
                        ..default()
                    },
                },
                SpaceshipMissile,
                Health::new(MISSILE_HEALTH),
                CollisionDamage::new(MISSILE_DAMAGE),
        ));
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
