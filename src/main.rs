
use bevy::prelude::*;

fn main() {
	App::new()
    .add_systems(Startup, add_people)
    .add_systems(Update, (hello_world, greet_people))
    .run();
}

// Components

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


// Startup Systems

fn add_people(mut commands: Commands){
  /* Spawn some entites with person and name components */
  commands.spawn((Person, Name("John Francis".to_string())));
  commands.spawn((Person, Name("Rudy Reed".to_string())));
}


// Other Systems

fn hello_world() {
  println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>){
  /*query means that it iterates over entities who have 
  person and name components. (pretty sure)*/

  for name in &query {
    println!("hello {}!", name.0)
  }
}
