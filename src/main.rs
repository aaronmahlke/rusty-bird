use bevy::prelude::*;
use rand::random;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BIRD_COLOR: Color = Color::rgb(255.0, 0.0, 0.0);
const BIRD_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const BIRD_STARTING_POSITION: Vec3 = Vec3::new(-100.0, 0.0, 0.0);
const BIRD_SPEED: f32 = 400.0;
const INITIAL_BIRD_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);

const PIPE_WIDTH: f32 = 50.0;
const PIPE_HEIGHT: f32 = 400.0;
const PIPE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const PIPE_SPACE_IN_SECONDS: f32 = 3.0;
const PIPE_GAP: f32 = 200.0;

const SCROLL_SPEED: f32 = 6.0;

const GRAVITY: f32 = 9.821 * 100.4;

// wall
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

// x coordinates
const LEFT_WALL: f32 = -350.;
const RIGHT_WALL: f32 = 350.;

// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                apply_gravity,
                move_pipes,
                spawn_pipes,
                despawn_pipes,
            ),
        )
        .add_systems(Update, move_bird)
        .run()
}

#[derive(Component, Deref, DerefMut)]
struct PipeTick(Timer);

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Pipe;

struct PipePositions {
    top: Vec2,
    bottom: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct CollisionEvent;

#[derive(Component)]
struct Gravity;

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

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

fn apply_gravity(mut query: Query<&mut Velocity, With<Gravity>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut velocity in &mut query {
        velocity.y -= GRAVITY * delta;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta;
    }
}

fn move_bird(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Bird>>) {
    let mut bird_velocity = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Flap");
        bird_velocity.y = 1.0 * BIRD_SPEED;
    }
}

fn move_pipes(mut query: Query<&mut Transform, With<Pipe>>) {
    for mut transform in &mut query {
        transform.translation.x -= SCROLL_SPEED;
    }
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

impl PipePair {
    fn new(positions: &PipePositions) -> Self {
        let top = PipeBundle::new(positions.top);
        let bottom = PipeBundle::new(positions.bottom);

        PipePair { top, bottom }
    }
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
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

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Bird
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BIRD_STARTING_POSITION,
                scale: BIRD_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BIRD_COLOR,
                ..default()
            },
            ..default()
        },
        Bird,
        Collider,
        Gravity,
        Velocity(INITIAL_BIRD_DIRECTION.normalize() * BIRD_SPEED),
    ));

    // Walls

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // Pipe Timer
    commands.spawn(PipeTick(Timer::from_seconds(
        PIPE_SPACE_IN_SECONDS,
        TimerMode::Repeating,
    )));

    // Pipes
}
