
use bevy::prelude::*;

fn main() {
	App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, add_people)
    .add_systems(Update, (
      hello_world, 
      (update_people, greet_people).chain()))
    .run();
}
// note: .chain() makes them run in order instead of the
// most optimized way

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

fn update_people(mut query: Query<&mut Name, With<Person>>){
  /*First mutable query function.
  Takes in a name and changes it.*/
  for mut name in &mut query {
    if name.0 == "Rudy Reed" {
      name.0 = "Rudy Francis".to_string();
      break;
    }
  }
}

fn greet_people(query: Query<&Name, With<Person>>){
  /*query means that it iterates over entities who have 
  person and name components. (pretty sure)*/

  for name in &query {
    println!("hello {}!", name.0)
  }
}
