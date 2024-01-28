use crate::game::bird::*;
use crate::game::pipes::*;
use crate::game::walls::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_state::<GameState>()
            .insert_resource(Scoreboard { score: 0 })
            .add_systems(
                FixedUpdate,
                (apply_velocity, apply_gravity, check_for_collisions)
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(OnEnter(GameState::GameOver), game_over);
    }
}

pub const GRAVITY: f32 = 9.821 * 100.4;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Play,
    GameOver,
}
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
pub struct Gravity;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: u32,
}

pub fn apply_gravity(mut query: Query<&mut Velocity, With<Gravity>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut velocity in &mut query {
        velocity.y -= GRAVITY * delta;
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta;
    }
}

fn game_over() {
    println!("Game over!");
}

fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut bird_query: Query<&Transform, With<Bird>>,
    collider_query: Query<(&Transform, Option<&Pipe>, Option<&Wall>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let bird_transform = bird_query.single_mut();
    let bird_size = bird_transform.scale.truncate();

    // check collision with walls
    for (transform, maybe_pipe, maybe_wall) in &collider_query {
        let collision = collide(
            bird_transform.translation,
            bird_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_pipe.is_some() {
                println!("Collision with pipe");
                next_state.set(GameState::GameOver);
            }

            if maybe_wall.is_some() {
                println!("Collision with wall");
                next_state.set(GameState::GameOver);
            }
        }
    }
}
