use crate::game::base::*;
use bevy::prelude::*;
use rand::random;

use crate::game::walls::*;

const PIPE_WIDTH: f32 = 50.0;
const PIPE_HEIGHT: f32 = 400.0;
const PIPE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const PIPE_SPACE_IN_SECONDS: f32 = 3.0;
const PIPE_GAP: f32 = 200.0;

const SCROLL_SPEED: f32 = 1.0;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, pipe_setup).add_systems(
            FixedUpdate,
            (move_pipes, spawn_pipes, despawn_pipes).run_if(in_state(GameState::Play)),
        );
    }
}

#[derive(Component)]
pub struct Pipe;

struct PipePositions {
    top: Vec2,
    bottom: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct PipeTick(Timer);

#[derive(Bundle)]
struct PipeBundle {
    sprite_bundle: SpriteBundle,
    pipe: Pipe,
    collider: Collider,
}

#[derive(Component)]
struct PipePair {
    top: PipeBundle,
    bottom: PipeBundle,
}

impl PipeBundle {
    fn new(position: Vec2) -> Self {
        PipeBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 0.0),
                    scale: Vec3::new(PIPE_WIDTH, PIPE_HEIGHT, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PIPE_COLOR,
                    ..default()
                },
                ..default()
            },
            pipe: Pipe,
            collider: Collider,
        }
    }
}

impl PipePair {
    fn new(positions: &PipePositions) -> Self {
        let top = PipeBundle::new(positions.top);
        let bottom = PipeBundle::new(positions.bottom);

        PipePair { top, bottom }
    }
}

fn move_pipes(mut query: Query<&mut Transform, With<Pipe>>) {
    for mut transform in &mut query {
        transform.translation.x -= SCROLL_SPEED;
    }
}

fn get_random_pipe_positions_at(pos_x: f32) -> PipePositions {
    let gap_position = random::<f32>() * (TOP_WALL);
    let top_pipe_y = (PIPE_HEIGHT / 2.0) + gap_position;
    let top_pipe_position = Vec2::new(pos_x, top_pipe_y);

    let bottom_pipe_y = top_pipe_y - PIPE_GAP - PIPE_HEIGHT;
    let bottom_pipe_position = Vec2::new(pos_x, bottom_pipe_y);

    PipePositions {
        top: top_pipe_position,
        bottom: bottom_pipe_position,
    }
}

fn spawn_pipes(mut commands: Commands, time: Res<Time>, mut query: Query<&mut PipeTick>) {
    for mut timer in &mut query {
        if timer.tick(time.delta()).just_finished() {
            let positions = get_random_pipe_positions_at(RIGHT_WALL + PIPE_WIDTH);
            commands.spawn(PipePair::new(&positions).top);
            commands.spawn(PipePair::new(&positions).bottom);
        }
    }
}

fn despawn_pipes(mut commands: Commands, mut query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &mut query {
        if transform.translation.x < LEFT_WALL - PIPE_WIDTH {
            commands.entity(entity).despawn();
        }
    }
}

fn pipe_setup(mut commands: Commands) {
    // Pipe Timer
    commands.spawn(PipeTick(Timer::from_seconds(
        PIPE_SPACE_IN_SECONDS,
        TimerMode::Repeating,
    )));
}
