use crate::grid::{Grid, Index};

#[derive(PartialEq, Debug)]
pub enum CellState{
    Alive(i32),
    Dead(i32)
}

pub fn create_next_grid(grid:&Grid<i32>) -> Grid<i32>{
    let mut next_grid = Grid(grid.0.clone());
    
    for x in 0..grid.get_width(){
        for y in 0..grid.get_height(){
            let v = match get_next_state_for_index(grid, Index { x, y  }){
                Ok(v) => v,
                Err(e) => panic!("Could not get next state for index [{}, {}]. Error: {}", x, y, e)
            };

            next_grid.set(x, y, v);
        }
    }

    next_grid
}

pub fn get_next_state(cell_state:CellState) -> i32{
    match cell_state {
        CellState::Alive(n) => if n < 2 {0} else if n < 4 {1} else {0},
        CellState::Dead(n) => if n == 3 {1} else {0}
    }
}

pub fn get_next_state_for_index(grid:&Grid<i32>, index:Index) -> Result<i32, String>{
    match get_cell_state(grid, index){
        Ok(cell_state) => Ok(get_next_state(cell_state)),
        Err(e) => Err(e)
    }
}

pub fn get_cell_state(grid:&Grid<i32>, index:Index) -> Result<CellState, String>{
    let x_max = grid.get_width() - 1;
    let y_max = grid.get_height() - 1;
    
    let start_x = if index.x > 0 { index.x - 1} else if index.x > x_max { x_max } else{ index.x};
    let end_x = if index.x + 1 > x_max { x_max} else {index.x + 1};
    let start_y = if index.y > 0 { index.y - 1} else if index.x > y_max { y_max } else {index.y};
    let end_y = if index.y + 1 > y_max {y_max } else {index.y + 1};
    
    let mut alive_count = 0;
    for x in start_x..=end_x{
        for y in start_y..=end_y{          
            // skip self
            if x == index.x && y == index.y{
                continue;
            }
            
            let cell_val = match grid.get(x, y){
                Some(v) => v,
                None => panic!("Missed index [{}, {}]. Grid [{},{}].", x, y, x_max, y_max)
            };
            
            alive_count += cell_val;
        }
    }
    
    match grid.get(index.x, index.y){
        Some(1) => Ok(CellState::Alive(alive_count)),
        Some(0)       => Ok(CellState::Dead(alive_count)),
        Some(_) => Err(String::from("Unknown cell state.")),
        None => Err(String::from("Index out of bounds.")),
    }
}

#[test]
fn alive_cell_with_less_than_two_neighbours_should_die(){
    let cell_state = CellState::Alive(1);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Alive(0);
    assert_eq!(get_next_state(cell_state), 0);
}

#[test]
fn alive_cell_with_two_or_three_neighbours_should_live(){
    let cell_state = CellState::Alive(2);
    assert_eq!(get_next_state(cell_state), 1);

    let cell_state = CellState::Alive(3);
    assert_eq!(get_next_state(cell_state), 1);
}

#[test]
fn alive_cell_with_four_or_more_neighbours_should_die(){
    let cell_state = CellState::Alive(4);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Alive(5);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Alive(6);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Alive(7);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Alive(8);
    assert_eq!(get_next_state(cell_state), 0);
}

#[test]
fn dead_cell_with_less_than_three_neighbours_should_stay_dead(){
    let cell_state = CellState::Dead(0);
    assert_eq!(get_next_state(cell_state), 0);
    
    let cell_state = CellState::Dead(1);
    assert_eq!(get_next_state(cell_state), 0);
    
    let cell_state = CellState::Dead(2);
    assert_eq!(get_next_state(cell_state), 0);
}

#[test]
fn dead_cell_with_three_neighbours_should_come_alive(){
    let cell_state = CellState::Dead(3);
    assert_eq!(get_next_state(cell_state), 1);
}

#[test]
fn dead_cell_with_four_or_more_neighbours_should_stay_dead(){
    let cell_state = CellState::Dead(4);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Dead(5);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Dead(6);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Dead(7);
    assert_eq!(get_next_state(cell_state), 0);

    let cell_state = CellState::Dead(8);
    assert_eq!(get_next_state(cell_state), 0);
}

#[test]
fn get_cell_state_of_alive_center_cell_with_no_neighbours(){
    let rows = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let grid = match Grid::from_rows(&rows) {
        Ok(arr) => arr,
        Err(_e) => panic!()
    };

    println!("{}", grid);

    let cell_state = get_cell_state(&grid, Index{x:1, y:1}).unwrap();
    assert_eq!(cell_state, CellState::Alive(0));
}

#[test]
fn get_cell_state_of_alive_center_cell_with_all_neighbours(){
    let rows = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    let grid = match Grid::from_rows(&rows) {
        Ok(arr) => arr,
        Err(_e) => panic!()
    };

    println!("{}", grid);

    let cell_state = get_cell_state(&grid, Index{x:1, y:1}).unwrap();
    assert_eq!(cell_state, CellState::Alive(8));
}

#[test]
fn alive_center_cell_of_3x3_grid_with_four_neighbours_should_die(){
    let rows = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
    let grid = match Grid::from_rows(&rows) {
        Ok(arr) => arr,
        Err(_e) => panic!()
    };

    println!("{}", grid);

    let cell_state = get_cell_state(&grid, Index{x:1, y:1}).unwrap();
    assert_eq!(cell_state, CellState::Alive(4));

    let next_state = get_next_state_for_index(&grid, Index{x: 1, y: 1}).unwrap();
    assert_eq!(next_state, 0);
}

#[test]
fn horizontal_middle_line_in_3x3_grid_should_shift_to_vertical_line(){
    let rows = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
    let grid = match Grid::from_rows(&rows) {
        Ok(arr) => arr,
        Err(_e) => panic!()
    };

    println!("{}", grid);
    assert_eq!(get_cell_state(&grid, Index{x:0, y:0}).unwrap(), CellState::Dead(2));
    assert_eq!(get_cell_state(&grid, Index{x:0, y:1}).unwrap(), CellState::Dead(3));
    assert_eq!(get_cell_state(&grid, Index{x:0, y:2}).unwrap(), CellState::Dead(2));
    assert_eq!(get_cell_state(&grid, Index{x:1, y:0}).unwrap(), CellState::Alive(1));
    assert_eq!(get_cell_state(&grid, Index{x:1, y:1}).unwrap(), CellState::Alive(2));
    assert_eq!(get_cell_state(&grid, Index{x:1, y:2}).unwrap(), CellState::Alive(1));
    assert_eq!(get_cell_state(&grid, Index{x:2, y:0}).unwrap(), CellState::Dead(2));
    assert_eq!(get_cell_state(&grid, Index{x:2, y:1}).unwrap(), CellState::Dead(3));
    assert_eq!(get_cell_state(&grid, Index{x:2, y:2}).unwrap(), CellState::Dead(2));
}

#[test]
fn dead_center_cell_of_3x3_grid_with_three_neighbours_should_come_alive(){
    let rows = vec![vec![1, 0, 0], vec![1, 0, 0], vec![1, 0, 0]];
    let grid = match Grid::from_rows(&rows) {
        Ok(arr) => arr,
        Err(_e) => panic!()
    };

    println!("{}", grid);
    let cell_state = get_cell_state(&grid, Index{x:1, y:1}).unwrap();
    assert_eq!(cell_state, CellState::Dead(3));

    let next_grid = create_next_grid(&grid);
    assert_eq!(next_grid.get(1, 1), Option::Some(&1));
}