use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::render_resource::Texture,
};
use bevy_rapier2d::{
    dynamics::Velocity,
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use player::{Player, PlayerPlugin};
use tilemap::{ShadowVisibility, TileMapPlugin};

pub mod player;
pub mod tilemap;

const PLAYER_SPEED: f32 = 256.;
const VIEW_DISTANCE: f32 = 300.;
const TILE_SIZE: f32 = 1.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_systems(Startup, setup_physics)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(TileMapPlugin)
        .add_systems(Startup, spawn_camera_system)
        .add_systems(Update, (tile_visibility_system, move_camera_system))
        .run();
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut config: ResMut<RapierConfiguration>) {
    config.gravity = vec2(0., 0.);
}

fn tile_visibility_system(
    player_query: Query<&Transform, With<Player>>,
    mut tile_query: Query<(
        &mut Visibility,
        &mut Handle<Image>,
        &GlobalTransform,
        &ShadowVisibility,
    )>,
    ass_serv: Res<AssetServer>,
) {
    let ptransform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => {
            println!("Couldn't find player!");
            return;
        }
    };

    for (mut vis, mut texture, transform, shadow_vis) in &mut tile_query {
        if ptransform.translation.floor().distance(transform.translation().floor()) <= VIEW_DISTANCE {
            if texture.set_if_neq(ass_serv.load(shadow_vis.light_path.clone())) {
                texture.set_changed();
            }
            if vis.set_if_neq(Visibility::Visible) {
                vis.set_changed();
            }
        } else {
            if let Some(path) = shadow_vis.shadow_path.clone() {
                if texture.set_if_neq(ass_serv.load(path)) {
                    texture.set_changed();
                }
            } else {
                if vis.set_if_neq(Visibility::Hidden) {
                    vis.set_changed();
                }
            }
        }
    }
}

fn move_camera_system(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player = player_query.single();
    let mut camera = camera_query.single_mut();

    let difx = player.translation.x - camera.translation.x;
    let dify = player.translation.y - camera.translation.y;

    camera.translation.x += f32::max(difx - 200., 0.) + f32::min(difx + 200., 0.);
    camera.translation.y += f32::max(dify - 200., 0.) + f32::min(dify + 200., 0.);
}

#[derive(Component)]
struct Menu;
fn setup_menu(mut commands: Commands) {}
