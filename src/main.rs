#![allow(unused)]

use std::collections::HashSet;
use bevy::{prelude::*, render::view::window, ecs::query, sprite::collide_aabb::collide, math::Vec3Swizzles, app::AppExit};
mod player;
use player::PlayerPlugin;
mod enemy;
use enemy::{EnemyPlugin, Enemy};

const PLAYER_SPRITE: &str = "player.png";
const LASER_SPRITE: &str = "laser.png";
const ENEMY_SPRITE: &str = "enemy.png";
const EXPLOSION_SHEET: &str = "explosion.png";
const ENEMY_LASER_SPRITE: &str = "enemy_laser.png";
const TIME_STEP: f32 = 1. / 60.;
const SCALE: f32 = 1. / 3.;
// structs
struct Textures {
    player_texture: Handle<Image>,
    laser_texture: Handle<Image>,
    enemy_texture: Handle<Image>,
    explosion_sheet: Handle<TextureAtlas>,
    enemy_laser_texture: Handle<Image>,
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
struct FromEnemy;
#[derive(Component)]
struct FromPlayer;

#[derive(Component)]
struct PlayerCanShoot(bool);
struct WinSize{
    height: f32,
    width: f32,
}
#[derive(Component)]
struct ActiveEnemies(i32);

#[derive(Component)]
struct Explosion;
#[derive(Component)]
struct ExplosionSpawnLocation(Vec3);
#[derive(Component)]
struct ExplosionAnimationTimer {
    timer: Timer,
}

// app setup
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "space_invadors".to_string(),
            width: 500.0,
            height: 700.0,
            ..Default::default()
        })      
        .insert_resource(ActiveEnemies(0))
        .add_plugin(EnemyPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)   
        .add_system(player_laser_hit_enemy)
        .add_system(enemy_laser_hit_player)
        .add_system(explosion_spawn)
        .add_system(animate_explosion)
        .run();
    
}

fn setup(
    mut commands: Commands,  
    asset_server: Res<AssetServer>,         
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    //cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());            
    
    let mut window = windows.get_primary_mut().unwrap();

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,Vec2::new(64., 64.), 4, 4
    );
    //init textures
    commands.insert_resource(
        Textures {
            player_texture: asset_server.load(PLAYER_SPRITE).into(),
            laser_texture: asset_server.load(LASER_SPRITE).into(),
            enemy_texture: asset_server.load(ENEMY_SPRITE).into(),
            explosion_sheet: texture_atlases.add(texture_atlas),
            enemy_laser_texture: asset_server.load(ENEMY_LASER_SPRITE).into(),
        }
    );

    commands.insert_resource(
        WinSize {
            width: window.width(),
            height: window.height(),
        }
    );
}

fn player_laser_hit_enemy (
    mut commands: Commands,
    mut laser_query: Query<(&Transform, Entity, &Sprite), (With<Laser>, With<FromPlayer>)>,
    mut enemy_query: Query<(&Transform, Entity, &Sprite), With<Enemy>>,
    mut active_enemies_count: ResMut<ActiveEnemies>,
) {
    let mut despawed_entities:HashSet<Entity> = HashSet::new();

    for(laser_tf, laser_entity, laser_sprite) in laser_query.iter_mut() {
        if despawed_entities.contains(&laser_entity) {
            continue;
        }

        for(enemy_tf, enemy_entity, enemy_sprite) in enemy_query.iter_mut() {
            if despawed_entities.contains(&enemy_entity) {
                continue;
            }

            let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);
            let enemy_scale = Vec2::new(enemy_tf.scale.x, enemy_tf.scale.y);


            let collision = collide(
                laser_tf.translation, laser_scale * laser_sprite.custom_size.unwrap(), 
                enemy_tf.translation, enemy_scale * enemy_sprite.custom_size.unwrap()
            );
            
            if let Some(_) =  collision {
                commands.entity(enemy_entity).despawn();
                despawed_entities.insert(enemy_entity);

                commands.entity(laser_entity).despawn();
                despawed_entities.insert(laser_entity);

                active_enemies_count.0 -= 1;

                commands
                    .spawn()
                    .insert(ExplosionSpawnLocation(enemy_tf.translation.clone()));
            }

            //spawn explosion

        }
        
    }
}

fn explosion_spawn (
    mut commands: Commands,
    textures: Res<Textures>,
    query: Query<(Entity, &ExplosionSpawnLocation)>
) {
    for(explosion_spawn_entity, explosion_spawn_location) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.explosion_sheet.clone(),
                transform: Transform { 
                    translation: explosion_spawn_location.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionAnimationTimer {
                timer: Timer::from_seconds(0.05, true)
            });
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn animate_explosion (
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut ExplosionAnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),With<Explosion>>
) {
    for(entity, mut timer, mut texture_atlas_sprite, texture_atlas_handle) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                texture_atlas_sprite.index += 1;

                if texture_atlas_sprite.index == texture_atlas.textures.len() {
                    commands.entity(entity).despawn();
                }
            }

        }
    }
}
fn enemy_laser_hit_player (
    mut commands: Commands,
    mut laser_query: Query<(&Transform, Entity, &Sprite), (With<Laser>, With<FromEnemy>)>,
    mut player_query: Query<(&Transform, Entity, &Sprite), With<Player>>,
    mut exit: EventWriter<AppExit>,
) {
    
    for(laser_tf, laser_entity, laser_sprite) in laser_query.iter_mut() {
        if let Some((player_tf, player_entity, player_sprite)) = Some(player_query.single()) {
 
            let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);
            let player_scale = Vec2::new(player_tf.scale.x, player_tf.scale.y);

            let collision = collide(
                laser_tf.translation, laser_scale * laser_sprite.custom_size.unwrap(), 
                player_tf.translation, player_scale * player_sprite.custom_size.unwrap()
            );

            if let Some(_) = collision {           
                info_span!("game_over!");
                
                commands.entity(laser_entity).despawn();
                commands.entity(player_entity).despawn();


                commands
                    .spawn()
                    .insert(ExplosionSpawnLocation(player_tf.translation.clone()));

                //exit.send(AppExit);
            }
        }
       
    }
}
