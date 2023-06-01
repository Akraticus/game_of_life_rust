use bevy::prelude::*;


pub struct PersonPlugin;

#[derive(Resource)]
pub struct GreetTimer(Timer);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);


impl Plugin for PersonPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(increment_greet_timer)
            .add_startup_system(add_people)
            .add_system(greet_named_person);
    }
}

fn add_people(mut commands:Commands){
    commands.spawn((Person, Name("Cnut".to_string())));
    commands.spawn((Person, Name("Thorfinn".to_string())));
    commands.spawn((Person, Name("Einar".to_string())));
    commands.spawn((Person, Name("Hebi".to_string())));
}

fn increment_greet_timer(time:Res<Time>, mut timer:ResMut<GreetTimer>){
    timer.0.tick(time.delta());
}

fn greet_named_person(timer:Res<GreetTimer>, query:Query<&Name, With<Person>>){
    if timer.0.just_finished() {    
        for name in &query{
            println!("Hello, {}.", name.0);
        }
    }
}