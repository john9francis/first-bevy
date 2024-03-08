
use bevy::prelude::*;

// Creating our own plugin
pub struct HelloPlugin;


// our own Implementation of the "Plugin" type
impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    // add things to app here
    app.add_systems(Startup, add_people)
      .add_systems(Update, (
      hello_world, 
      (update_people, greet_people).chain()));
  }
}
// note: .chain() makes them run in order instead of the
// most optimized way aka multithreaded

// Note: Default plugins opens up a window and 
// also a game loop

fn main() {
	App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
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

