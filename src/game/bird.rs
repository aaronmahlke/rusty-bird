use crate::game::base::*;
use bevy::prelude::*;
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_bird);
    }
}

const BIRD_COLOR: Color = Color::rgb(255.0, 0.0, 0.0);
const BIRD_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const BIRD_STARTING_POSITION: Vec3 = Vec3::new(-100.0, 0.0, 0.0);
const BIRD_SPEED: f32 = 400.0;
const INITIAL_BIRD_DIRECTION: Vec2 = Vec2::new(0.0, 1.0);

#[derive(Component)]
pub struct Bird;

fn move_bird(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Bird>>) {
    let mut bird_velocity = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Flap");
        bird_velocity.y = 1.0 * BIRD_SPEED;
    }
}

fn setup(mut commands: Commands) {
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
