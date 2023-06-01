use std::time;
use std::thread;
use bevy::prelude::*;
use grid::Grid;

mod person_plugin;
mod grid;
mod game_of_life;
mod cmdline;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(person_plugin::PersonPlugin)
        .run();
}

fn print_grid(grid:&Grid<i32>){
    clearscreen::clear();
    println!();
    println!("{}", grid);
}