#![allow(unused)]

use bevy::{prelude::*, render::view::window, ecs::query};

const PLAYER_SPRITE: &str = "player.png";
const TIME_STEP: f32 = 1. / 60.;

struct Textures {
    player_texture: Handle<Image>,
}
#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerSpeed(f32);
impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(500.)
    }
}
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
        .add_startup_system(setup)   
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_system(move_player)
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
            player_texture: asset_server.load("player.png").into(),
        }
    );

    commands.insert_resource(
        WinSize {
            width: window.width(),
            height: window.height(),
        }
    );
}

fn spawn_player(
    mut commands: Commands,  
    window: Res<WinSize>,
    textures: Res<Textures>,
    asset_server: Res<AssetServer>,
    
) {
    
    let bottom = - window.height / 2.;

     let player_texture_image = textures.player_texture.clone();

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {                        
            custom_size: (Some(Vec2::new(210.0, 145.0))),            
            ..Default::default()},
        texture: player_texture_image,
        transform: Transform {
            translation: Vec3::new(0.,  bottom + 100. /4. , 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(PlayerSpeed::default());
}
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform), (With<Player>)>,
    time: Res<Time>
) {
    for (speed, mut transform) in query.iter_mut() {
        let dir = if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };  

        transform.translation.x += speed.0 * dir * time.delta_seconds();
    }
        
    
}
