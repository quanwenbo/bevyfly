use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        // .add_startup_system(add_people_system.system()) //run only once
        // .add_system(greet_people_system.system())
        // .add_system(hello_world.system())
        .run();
}

fn hello_world() {}

//components
#[derive(Debug)]
struct Person;
struct Name(String);

//system
fn add_people_system(mut commands: Commands) {
    commands
        .spawn((Person, Name("权".to_string())))
        .spawn((Person, Name("文".to_string())))
        .spawn((Person, Name("博".to_string())));
}

//resource
struct GreetTimerRes(Timer);

//system
fn greet_people_system(
    time: Res<Time>,
    mut timer: ResMut<GreetTimerRes>,
    mut query: Query<(&Person, &Name)>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
        timer.0.reset();
    }
}

//Plugin
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        println!("plugin created,and the app get it");
        app.add_resource(GreetTimerRes(Timer::from_seconds(1.5, false)))
            .add_startup_system(add_people_system.system())
            .add_system(hello_world.system())
            .add_system(greet_people_system.system());
    }
}
