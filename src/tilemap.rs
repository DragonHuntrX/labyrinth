use std::{
    borrow::Borrow,
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::{
    dynamics::{AdditionalMassProperties, Damping, LockedAxes, RigidBody},
    geometry::{Collider, Friction},
};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_level_system)
            .add_systems(Update, reload_map_system);
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct ShadowVisibility {
    pub shadow_path: Option<String>,
    pub light_path: String,
}

fn reload_map_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut map: Query<Entity, With<Map>>,
    ass_serv: Res<AssetServer>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        let map = commands.entity(map.single_mut());
        map.despawn_recursive();
        load_level(&mut commands, "begin.txt", ass_serv);
    }
}

fn load_level_system(mut commands: Commands, ass_serv: Res<AssetServer>) {
    load_level(&mut commands, "begin.txt", ass_serv);
}

fn load_level(commands: &mut Commands, levelname: &str, ass_serv: Res<AssetServer>) {
    let file = File::open(format!("assets/maps/{}", levelname)).unwrap();

    let mut map_entity = commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        GlobalTransform::default(),
        Map,
        InheritedVisibility::VISIBLE,
    ));

    for (r, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (c, character) in line.chars().enumerate() {
                match character {
                    'O' => {
                        map_entity.with_children(|commands| {
                            commands.spawn((
                                RigidBody::Dynamic,
                                ShadowVisibility {
                                    shadow_path: None,
                                    light_path: "push_block.png".to_string(),
                                },
                                LockedAxes::ROTATION_LOCKED,
                                Damping {
                                    linear_damping: 10.,
                                    angular_damping: 0.0,
                                },
                                Collider::cuboid(31., 31.),
                                SpriteBundle {
                                    texture: ass_serv.load("push_block.png"),
                                    visibility: Visibility::Hidden,
                                    transform: Transform::from_xyz(
                                        c as f32 * 64.,
                                        r as f32 * -64.,
                                        0.,
                                    ),
                                    ..default()
                                },
                            ));
                        });
                    }
                    'X' => {
                        map_entity.insert(Transform::from_xyz(c as f32 * -64., r as f32 * 64., 0.));
                    }
                    _ => (),
                }

                match character {
                    '#' => {
                        map_entity.with_children(|commands| {
                            commands.spawn((
                                RigidBody::Fixed,
                                ShadowVisibility {
                                    shadow_path: Some("wall_shadow.png".to_string()),
                                    light_path: "wall_light.png".to_string(),
                                },
                                Collider::cuboid(32.1, 32.1),
                                SpriteBundle {
                                    texture: ass_serv.load("wall_shadow.png"),
                                    visibility: Visibility::Hidden,
                                    transform: Transform::from_xyz(
                                        c as f32 * 64.,
                                        r as f32 * -64.,
                                        0.,
                                    ),
                                    ..default()
                                },
                            ));
                        });
                    }
                    '.' => (),
                    _ => {
                        map_entity.with_children(|commands| {
                            commands.spawn((
                                ShadowVisibility {
                                    shadow_path: Some("ground_shadow.png".to_string()),
                                    light_path: "ground_light.png".to_string(),
                                },
                                SpriteBundle {
                                    texture: ass_serv.load("ground_shadow.png"),
                                    visibility: Visibility::Hidden,
                                    transform: Transform::from_xyz(
                                        c as f32 * 64.,
                                        r as f32 * -64.,
                                        -1.,
                                    ),
                                    ..default()
                                },
                            ));
                        });
                    }
                }
            }
        }
    }
}
