use bevy::{app::AppExit, math::bool, prelude::*};
use bevy_mod_scripting::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(ScriptingPlugin)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (exit, fall, toggle_pause));

    app.run();

    println!("Hello, world!");
}

fn exit(mut app_exit: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyQ) {
        app_exit.send(AppExit::Success);
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.insert_resource(Paused(true));

    commands.spawn((
        Sprite::from_image(server.load("bevy.png")),
        Transform::from_xyz(100.0, 0.0, 0.0),
        Character,
    ));
}

fn scripts_load(mut commands: Commands, server: Res<AssetServer>) {}

#[derive(Resource)]
struct Paused(bool);

#[derive(Component)]
struct Character;

fn toggle_pause(input: Res<ButtonInput<KeyCode>>, mut pause: ResMut<Paused>) {
    if input.just_released(KeyCode::Space) {
        pause.0 = !pause.0;
        println!("{}", pause.0);
    }
}
fn fall(
    time: Res<Time>,
    mut commands: Commands,
    mut pos: Query<(Entity, &mut Transform), With<Character>>,
    paused: Res<Paused>,
) {
    if !paused.0 {
        for (e, mut s) in &mut pos {
            s.translation.y -= 100.0 * time.delta_secs();
            // println!("{}", s.translation.y);
            if s.translation.y < -300. {
                commands.entity(e).despawn();
                println!("Despawn");
            }
        }
    }
}
