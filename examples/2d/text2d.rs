use bevy::{prelude::*, text::Text2dBounds};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_translation)
        .add_system(animate_rotation)
        .add_system(animate_scale)
        .run();
}

#[derive(Component)]
struct AnimateTranslation;
#[derive(Component)]
struct AnimateRotation;
#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Demonstrate changing translation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("translation", text_style.clone(), text_alignment),
            ..default()
        })
        .insert(AnimateTranslation);
    // Demonstrate changing rotation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("rotation", text_style.clone(), text_alignment),
            ..default()
        })
        .insert(AnimateRotation);
    // Demonstrate changing scale
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("scale", text_style.clone(), text_alignment),
            ..default()
        })
        .insert(AnimateScale);
    // Demonstrate text wrapping
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 200.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        ..default()
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("this text wraps in the box", text_style, text_alignment),
        text_2d_bounds: Text2dBounds {
            // Wrap text in the rectangle
            size: Size::new(300.0, 200.0),
        },
        transform: Transform::from_xyz(0.0, -250.0, 1.0),
        ..default()
    });
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32 - 400.0;
        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32);
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);
        transform.scale = Vec3::splat((time.seconds_since_startup().sin() as f32 + 1.1) * 2.0);
    }
}
