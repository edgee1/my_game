use bevy::{prelude::*, render::view::window, ecs::query};

use crate::{WinSize, Textures, Velocity, Player, PlayerCanShoot, Laser, TIME_STEP};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {        
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage("player_setup", SystemStage::single(spawn_player))
            .add_system(move_player)
            .add_system(player_fire)
            .add_system(laser_movement);
    }

}
fn spawn_player(
    mut commands: Commands,  
    window: Res<WinSize>,
    textures: Res<Textures>,
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
    .insert(PlayerCanShoot(true))
    .insert(Velocity::default());

    
}
fn move_player(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&Velocity, &mut Transform), (With<Player>)>,
    time: Res<Time>
) {
    let (speed, mut transform) = query.single_mut();
    let dir = if kb.pressed(KeyCode::A)  {
        -1.
    } else if kb.pressed(KeyCode::D) {
        1.
    } else {
        0.
    };  

    transform.translation.x += speed.0 * dir * time.delta_seconds();
    

}
fn player_fire(
    kb: Res<Input<KeyCode>>,
    mut commands: Commands,
    textures: Res<Textures>,
    mut query: Query<(&Transform, &mut PlayerCanShoot), With<Player>>
) { 
    let (player_tr, mut can_shoot) =  query.single_mut();
    if kb.pressed(KeyCode::Space) && can_shoot.0{

        let mut spawn_laser = |x_offset: f32| {
            
            let (x, y) = (player_tr.translation.x, player_tr.translation.y);
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite { custom_size: (Some(Vec2::new(50., 80.))), ..Default::default() },
                texture: textures.laser_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(x + x_offset,  y + 50., 0.),                
                    ..Default::default()
                },
            ..Default::default()
            })
            .insert(Laser)
            .insert(Velocity::default());
        
        };

        let x_offset: f32 = 144. / 4. + 10.;

        spawn_laser(x_offset);
        spawn_laser(-x_offset);  
        can_shoot.0 = false;
    }        

    if kb.just_released(KeyCode::Space) {
        can_shoot.0 = true;
    }
}
fn laser_movement (
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(&mut Transform, &Velocity, Entity), With<Laser>>
) {
    for(mut laser_tf, speed, laser_entity) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > win_size.height / 2. {
            commands.entity(laser_entity).despawn();
        }
    }
}