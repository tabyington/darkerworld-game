mod systems;
mod plugins;

use bevy::prelude::*;
use plugins::physics;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Update, hello_world);
    }
}

fn main() {
    //println!("Hello, world!");
    App::new()
    .add_plugins(HelloPlugin)
    .add_plugins(DefaultPlugins)
    .run();

}

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world!");
    }   
}

#[derive(Resource)]
struct GreetTimer(Timer);