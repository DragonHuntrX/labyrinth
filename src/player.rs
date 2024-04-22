use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::{
    control::KinematicCharacterController,
    dynamics::{LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::{PLAYER_SPEED, TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system)
            .add_systems(Update, player_move_system);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player_system(mut commands: Commands, ass_serv: Res<AssetServer>) {
    commands.spawn((
        Player,
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(31.0, 31.0),
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: ass_serv.load("player.png"),
            ..Default::default()
        },
    ));
}

fn player_move_system(
    mut player_query: Query<&mut KinematicCharacterController>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player = player_query.single_mut();
    let mut translation = vec2(0., 0.);
    if input.pressed(KeyCode::KeyW) {
        translation.y += PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyS) {
        translation.y -= PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        translation.x += PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyA) {
        translation.x -= PLAYER_SPEED;
    }
    player.translation = Some(translation * time.delta_seconds());
}
