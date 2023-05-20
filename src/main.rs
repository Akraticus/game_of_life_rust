use std::time;
use std::thread;
use grid::Grid;

mod grid;
mod game_of_life;
mod cmdline;

fn main() {
    let width = loop{
        match cmdline::query_user_for_value_with_message::<usize>(String::from("Set width of grid:")){
            Ok(v) => break v,
            Err(e) => println!("{}", e)
        }
    };

    let height = loop{
        match cmdline::query_user_for_value_with_message::<usize>(String::from("Set height of grid:")){
            Ok(v) => break v,
            Err(e) => println!("{}", e)
        }
    };

    let update_ms = loop{
        match cmdline::query_user_for_value_with_message::<u64>(String::from("Set update rate (ms):")){
            Ok(v) => break v,
            Err(e) => println!("{}", e)
        }
    };

    // spawning this thread doesn't actually do anything, just wanted to try
    let thread_handle = thread::spawn(move || {
        let mut grid = grid::Grid::random_filled(&vec![0, 1], height, width);
        print_grid(&grid);
        loop {
            // make next grid
            grid = game_of_life::create_next_grid(&grid);
            // print result
            print_grid(&grid);
            // rest for a bit
            thread::sleep(time::Duration::from_millis(update_ms));
        }
    });

    // if we don't join to the thread, it's fire and forget, and it can run forever, so...
    thread_handle.join();
}

fn print_grid(grid:&Grid<i32>){
    clearscreen::clear();
    println!();
    println!("{}", grid);
}