use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

#[derive(Component)]
struct Square {}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    window.set_title("Wrapping Ball".to_string());
    window.set_resolution(WIN_WIDTH, WIN_HEIGHT);

    // Ball
    commands
        .spawn()
        .insert(Square {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.5, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Velocity(Vec2::new(100.0, 100.0)));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn accelerate_square(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Square>>,
) {
    let mut velocity = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        velocity.x -= 10.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        velocity.x += 10.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        velocity.y -= 10.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        velocity.y += 10.0;
    }
}

fn wrap_ball_around(mut query: Query<&mut Transform, With<Square>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let mut transform = query.single_mut();
    if transform.translation.x > width / 2.0 {
        transform.translation.x = -width / 2.0;
    }

    if transform.translation.x < -width / 2.0 {
        transform.translation.x = width / 2.0;
    }

    if transform.translation.y > height / 2.0 {
        transform.translation.y = -height / 2.0;
    }

    if transform.translation.y < -height / 2.0 {
        transform.translation.y = height / 2.0;
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(accelerate_square)
        .add_system(apply_velocity)
        .add_system(wrap_ball_around)
        .run();
}
