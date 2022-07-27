#![allow(unused)]

use bevy::{prelude::*, render::view::window};

const PLAYER_SPRITE: &str = "player.png";

struct Textures {
    player_texture: Handle<Image>,
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
        .add_startup_system(setup)   
        .add_startup_system(spawn_player)
        .run();
    
}

fn setup(
    mut commands: Commands,  
    asset_server: Res<AssetServer>,
) {
    //cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());            

    //init textures
    commands.insert_resource(
        Textures {
            player_texture: asset_server.load("player.png").into(),
        }
    )
}

fn spawn_player(
    mut commands: Commands,  
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    
) {
    let mut window = windows.get_primary_mut().unwrap();
    let bottom = - window.height() / 2.;

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {                        
            custom_size: (Some(Vec2::new(200.0, 100.0))),            
            ..Default::default()},
        texture: asset_server.load("player.png").into(),
        transform: Transform {
            translation: Vec3::new(0.,  bottom + 100. /4. , 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
fn move_player(
    mut commands: Commands
) {

}
