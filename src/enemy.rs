#![allow(unused)]

use bevy::{prelude::*, render::{view::window}, ecs::query, core::FixedTimestep};
use rand::{thread_rng, Rng};

use crate::{Textures, WinSize, ActiveEnemies, SCALE, FromEnemy, Laser, Velocity, TIME_STEP};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(enemy_laser_movement)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(spawn_enemy)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.9))
                    .with_system(enemy_fire)
            );

    }
}

fn spawn_enemy(
    mut commands: Commands,
    textures: Res<Textures>,
    win_size: Res<WinSize>,
    mut active_enemies_count: ResMut<ActiveEnemies>
) {
    let mut rng = thread_rng();
    let w_spawn = win_size.width / 2. - 100.;
    let h_spawn = win_size.height / 2. - 100.;
    
    let x = rng.gen_range(-w_spawn..w_spawn);
    let y = rng.gen_range(-h_spawn..h_spawn);

    if active_enemies_count.0 < 1 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {                        
                custom_size: (Some(Vec2::new(125., 125.0))), 
                flip_y: true,           
                ..Default::default()},
            texture: textures.enemy_texture.clone(),
            transform: Transform {
                translation: Vec3::new(x,  y , 0.),
                scale: Vec3::new(SCALE, SCALE, 0.),
                ..Default::default()
            },
            ..Default::default()
            
        })
        .insert(Enemy);
        active_enemies_count.0 += 1;
    }

}
fn enemy_fire (
    mut commands: Commands,
    textures: Res<Textures>,
    enemy_query: Query<(&Transform), With<Enemy>>
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite { custom_size: (Some(Vec2::new(15., 35.))), ..Default::default() },
            texture: textures.enemy_laser_texture.clone(),
            transform: Transform {
                translation: Vec3::new(x ,  y -15., 0.),                     
                ..Default::default()
            },
        ..Default::default()
        })
        .insert(Laser)
        .insert(FromEnemy)
        .insert(Velocity::default());
    }
}

fn enemy_laser_movement (
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut laser_query: Query<(Entity, &mut Transform, &Velocity), (With<Laser>, With<FromEnemy>)>
) {
    for(entity, mut tf, velocity) in laser_query.iter_mut() {
        tf.translation.y -= velocity.0 * TIME_STEP;
        if tf.translation.y < -win_size.height / 2. - 15. {
            commands.entity(entity).despawn();
        }
    }
}