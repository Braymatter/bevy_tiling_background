
use bevy::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, BackgroundMovementScale, SetImageRepeatingExt,
    TilingBackgroundPlugin,
};

/// Bevy doesn't render things that are attached to the camera, so this component will be used
/// on a parent entity to move our camera and background.
#[derive(Component)]
pub struct CameraRig;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilingBackgroundPlugin::<BackgroundMaterial>::default())
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(update_instructions)
        .run()
} 

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let image = asset_server.load("test.png");
    // Queue a command to set the image to be repeating once the image is loaded.
    commands.set_image_repeating(image.clone());

    // Spawn camera rig with camera and background as children
    commands
        .spawn((CameraRig, SpatialBundle::default()))
        .with_children(|child_builder| {
            child_builder.spawn(Camera2dBundle::default());
            child_builder.spawn(
                BackgroundImageBundle::from_image(image, materials.as_mut(), meshes.as_mut())
                    .at_z_layer(0.1),
            );
        });

    // Instructions
    commands.spawn((
        TextBundle::from_section(
            "Arrow keys to move\n\
        +/- for Parallax effect",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 32.0,
                ..default()
            },
        ),
        Instructions,
        Name::new("Instructions"),
    ));

    // Boxes as a simple environment to compare background movement to.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(10000.0, 100.0, 1.0))
                .with_translation(Vec3::new(0.0, -50.0, 1.0)),
            ..default()
        },
        Name::new("Green Box (Ground)"),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(100.0, 100.0, 1.0))
                .with_translation(Vec3::new(0.0, 50.0, 1.0)),
            ..default()
        },
        Name::new("Red Box"),
    ));
}

#[derive(Component)]
struct Instructions;

fn movement(
    mut camera: Query<&mut Transform, With<CameraRig>>,
    mut background_scales: Query<&mut BackgroundMovementScale>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let move_speed = 100.0;
    let mut camera_transform = camera.single_mut();
    if input.pressed(KeyCode::Left) {
        camera_transform.translation.x -= time.delta_seconds() * move_speed;
    }

    if input.pressed(KeyCode::Right) {
        camera_transform.translation.x += time.delta_seconds() * move_speed;
    }

    if input.pressed(KeyCode::Down) {
        camera_transform.translation.y -= time.delta_seconds() * move_speed;
    }

    if input.pressed(KeyCode::Up) {
        camera_transform.translation.y += time.delta_seconds() * move_speed;
    }

    for mut background_scale in background_scales.iter_mut() {
        if input.pressed(KeyCode::Plus) || input.pressed(KeyCode::NumpadAdd) {
            background_scale.scale += time.delta_seconds();
        }

        if input.pressed(KeyCode::Minus) || input.pressed(KeyCode::NumpadSubtract) {
            background_scale.scale -= time.delta_seconds();
        }
    }
}

fn update_instructions(
    mut query: Query<&mut Text, With<Instructions>>,
    background_movement: Query<&BackgroundMovementScale>,
) {
    let mut instructions = query.single_mut();
    instructions.sections.first_mut().unwrap().value = format!(
        "Arrow keys to move\n\
        +/- to change parallax \n\
        Current parallax multiplier {}",
        background_movement.single().scale
    );
}
