use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BIRD_COLOR: Color = Color::rgb(255.0, 0.0, 0.0);
const BIRD_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const BIRD_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BIRD_SPEED: f32 = 400.0;
const INITIAL_BIRD_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);

const GRAVITY: f32 = 9.821 * 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_velocity, apply_gravity, move_bird))
        .run()
}

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Pipe;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct CollisionEvent;

#[derive(Component)]
struct Gravity;

#[derive(Resource)]
struct Scoreboard {
    score: usize,
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
}
