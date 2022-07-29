#![allow(unused)]

use bevy::{prelude::*, render::view::window, ecs::query};


mod player;
use player::PlayerPlugin;
const PLAYER_SPRITE: &str = "player.png";
const LASER_SPRITE: &str = "laser.png";
const TIME_STEP: f32 = 1. / 60.;

struct Textures {
    player_texture: Handle<Image>,
    laser_texture: Handle<Image>,
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Velocity(f32);
impl Default for Velocity {
    fn default() -> Self {
        Self(500.)
    }
}
#[derive(Component)]
struct Laser;
#[derive(Component)]
struct PlayerCanShoot(bool);
struct WinSize{
    height: f32,
    width: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "space_invadors".to_string(),
            width: 1000.0,
            height: 500.0,
            ..Default::default()
        })      
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)   
        .run();
    
}

fn setup(
    mut commands: Commands,  
    asset_server: Res<AssetServer>,         
    mut windows: ResMut<Windows>,
) {
    //cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());            
    
    let mut window = windows.get_primary_mut().unwrap();
    //init textures
    commands.insert_resource(
        Textures {
            player_texture: asset_server.load(PLAYER_SPRITE).into(),
            laser_texture: asset_server.load(LASER_SPRITE).into(),
        }
    );

    commands.insert_resource(
        WinSize {
            width: window.width(),
            height: window.height(),
        }
    );
}

