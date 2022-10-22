use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};

mod components;
mod enemy;
mod player;

use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromPlayer, Laser, Movable, Player,
    SpriteSize, Velocity,
};
use enemy::*;
use player::*;

const PLAYER_IMAGE: &str = "player_a_01.png";
const LAGER_IMAGE: &str = "laser_a_01.png";

const SPRITE_SIZE: f32 = 0.3;
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const ENEMY_IMAGE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_IMAGE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_IMAGE: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "practice Bevy".to_string(),
            width: 300.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut texture_atlease: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion
    let texture_handle = asset.load(EXPLOSION_IMAGE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlease.add(texture_atlas);

    let game_textures = GameTextures {
        player: asset.load(PLAYER_IMAGE),
        laser: asset.load(LAGER_IMAGE),
        enemy: asset.load(ENEMY_IMAGE),
        enemy_laser: asset.load(ENEMY_LASER_IMAGE),
        explosion,
    };

    commands.insert_resource(game_textures);
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                // remove the enemy
                commands.entity(enemy_entity).despawn();

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn explosion
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (entity, spawn) in query.iter() {
        // spawn explosion

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn()
            }
        }
    }
}
