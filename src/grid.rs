use core::panic;
use std::{fmt, default, cmp::{max, min}, vec, collections::{HashSet, hash_set}};
use rand::prelude::*;
use array2d::{Array2D, Error};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
pub struct Index{
    pub x:usize,
    pub y:usize
}

/// The supported selection-shapes to get from a grid.
/// Number is the extent of the selection. Minimum 1, which will return a single cell for all shapes.
pub enum SelectionType{
    Cross(usize),
    DiagonalCross(usize),
    Circle(usize),
    Square(usize)
}

pub struct Grid<T>(pub Array2D<T>)
where 
    T: Clone,
    T: Copy,
    T:Default;

impl<T> Grid<T> 
where 
    T: Copy,
    T:Default {

    pub fn get_height(&self) -> usize {
        self.0.num_rows()
    }

    pub fn get_width(&self) -> usize {
        self.0.num_columns()
    }

    pub fn from_rows(elements: &[Vec<T>]) -> Result<Grid<T>, Error>{
        match Array2D::from_rows(elements) {
            Ok(arr) => Ok(Grid::<T>(arr)),
            Err(e) => Err(e)
        }
    }

    pub fn from_columns(elements: &[Vec<T>]) -> Result<Grid<T>, Error>{
        match Array2D::from_columns(elements) {
            Ok(arr) => Ok(Grid::<T>(arr)),
            Err(e) => Err(e)
        }
    }

    pub fn get(&self, x:usize, y:usize) -> Option<&T> {
        self.0.get(x, y)
    }

    pub fn get_index(&self, index:Index) -> Option<&T>{
        self.get(index.x, index.y)
    }

    pub fn get_mut(&mut self, x:usize, y:usize) -> Option<&mut T> {
        self.0.get_mut(x, y)
    }

    pub fn get_mut_index(&mut self, index:Index) -> Option<&mut T>{
        self.get_mut(index.x, index.y)
    }

    pub fn set(&mut self, x:usize, y:usize, value:T) -> Result<(), Error> {
        self.0.set(x, y, value)
    }

    pub fn set_index(&mut self, index:Index, value:T) -> Result<(), Error>{
        self.set(index.x, index.y, value)
    }

    pub fn filled_with(initial_value:T, width:usize, height:usize) -> Self {
        Grid( Array2D::filled_with(initial_value, height, width))
    }

    pub fn new(width:usize, height:usize) -> Self{
        Grid::filled_with(default::Default::default(), width, height)
    }

    pub fn random_filled(choice_collection:&Vec<T>, width:usize, height:usize) -> Self {
        let mut grid = Grid::new(width, height);
        let mut rng = thread_rng();

        for x in 0..height{
            for y in 0..width{
                let val = *choice_collection.choose(&mut rng).unwrap();
                grid.set(x, y, val);
            }
        }

        grid
    }

    pub fn get_selection_indexes(&self, selection_type:SelectionType, index:Index) -> Option<HashSet<Index>>{
        match selection_type{
            // subtract one from extent, because we always get the cell at index
            SelectionType::Cross(extent) =>         self.select_cross(extent, index),
            SelectionType::DiagonalCross(extent) => self.select_diagonal_cross(extent, index),
            SelectionType::Circle(extent) =>        self.select_circle(extent, index),
            SelectionType::Square(extent) =>        self.select_square(extent, index),
        }
    }

    fn select_cross(&self,  extent:usize, index:Index) -> Option<HashSet<Index>>{
        let mut indexes = HashSet::<Index>::with_capacity(extent * 4 + 1);
        
        // we subtract one, because we always get the cell at index requested
        let extent = if extent > 0 { extent - 1} else {0};
        let start = self.get_start_index(extent, &index);
        let end = self.get_end_index(extent, &index);


        for x in start.x..=end.x{
            indexes.insert(Index{x, y: index.y});
        }

        for y in start.y..=end.y{
            indexes.insert(Index{x: index.x, y});
        }

        Some(indexes)
    }

    fn select_diagonal_cross(&self, extent:usize, index:Index) -> Option<HashSet<Index>>{
        todo!()
    }

    fn select_circle(&self, extent:usize, index:Index) -> Option<HashSet<Index>>{
        todo!()
    }

    fn select_square(&self, extent:usize, index:Index) -> Option<HashSet<Index>>{
        let mut indexes = HashSet::<Index>::with_capacity(extent);
        
        // we subtract one, because we always get the cell at index requested
        let extent = if extent > 0 { extent - 1} else {0};

        let start = self.get_start_index(extent, &index);
        let end = self.get_end_index(extent, &index);


        for x in start.x..=end.x{
            for y in start.y..=end.y{
                indexes.insert(Index{x, y});
            }
        }

        Some(indexes)
    }

    pub fn get_selection(&self, selection_type:SelectionType, index:Index) -> Option<HashSet<T>>{
        let indexes = self.get_selection_indexes(selection_type, index);
        todo!()
    }

    fn get_start_index(&self, extent:usize, index:&Index) -> Index {
        Index { 
            x: if index.x > extent { index.x - extent } else { 0 },
            y: if index.y > extent { index.y - extent } else { 0 }
        }
    }

    fn get_end_index(&self, extent:usize, index:&Index) -> Index {
        let width = self.get_width();
        let height = self.get_height();
        Index { 
            x: if index.x + extent < width { index.x + extent } else { width - 1},
            y: if index.y + extent < height { index.y + extent } else { height - 1}
        }
    }

    fn clamp_to_grid_bounds(&self, index:Index) -> Index{
        let width = self.get_width();
        let height = self.get_height();

        Index{
            x: min(width, max(0, index.x)),
            y: min(height, max(0, index.y)),
        }
    }
}

impl fmt::Debug for Grid<i32>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in 0..self.get_width(){
            for y in 0..self.get_height(){
                let val = match self.get(x, y){
                    Some(v) => v,
                    _ => panic!("Index out of bounds")
                };
                let join_string = if *val >= 0 {", "} else {""};
                s = s + join_string + &val.to_string();
            }

            s = s + "\n";
        }
            
        write!(f, "{}", s)
    }
}

impl fmt::Display for Grid<i32>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for x in 0..self.get_width(){
            for y in 0..self.get_height(){
                let val = match self.get(x, y){
                    Some(v) => v,
                    _ => panic!("Index out of bounds")
                };
                let symbol = if *val == i32::from(1) {"■"} else {" "};
                let join_string = if *val >= 0 {" "} else {" "};
                s = s + join_string + symbol;
            }

            s = s + "\n";
        }
        write!(f, "{}", s)
    }
}

#[test]
fn iterate_over_grid_with_unequal_row_len_and_column_len(){
    let width = 4;
    let height = 6;
    let grid = Grid::filled_with(0, width, height);
    assert_eq!(width,     grid.get_width());
    assert_eq!(height,  grid.get_height());

    let grid2 = Grid::random_filled(&vec!(0, 1), 4, 6);
    assert_eq!(width, grid2.get_width());
    assert_eq!(height, grid2.get_height());
}

#[test]
fn get_cross_selection(){
    let grid = Grid::from_rows(&vec![vec![0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0], vec![1, 1, 1, 1, 1], vec![0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0]]).unwrap();
    println!("{}", grid);

    // extent of 1 should just return the target index
    let selection_extent_1 = grid.get_selection_indexes(SelectionType::Cross(1), Index{x: 1, y: 1}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 1, y: 1}]), selection_extent_1);
    
    // extent of 2 should return a cross-shape around the target index
    let selection_extent_2 = grid.get_selection_indexes(SelectionType::Cross(2), Index{x: 1, y: 1}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 0, y: 1}, Index{x: 1, y: 0}, Index{x: 1, y: 1}, Index{x: 1, y: 2}, Index{x: 2, y: 1}]), selection_extent_2);

    let selection_across_grid_bounds = grid.get_selection_indexes(SelectionType::Cross(2), Index{x: 0, y: 0}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 0, y: 0}, Index{x: 0, y: 1}, Index{x: 1, y: 0},  ]), selection_across_grid_bounds);

    let selection_across_grid_bounds_2 = grid.get_selection_indexes(SelectionType::Cross(2), Index{x: 4, y: 4}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 4, y: 4}, Index{x: 4, y: 3}, Index{x: 3, y: 4},  ]), selection_across_grid_bounds_2);
}

#[test]
fn get_square_selection(){
    let grid = Grid::from_rows(&vec![vec![0, 0, 0, 0, 0], vec![0, 1, 1, 1, 0], vec![0, 1, 1, 1, 0], vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]]).unwrap();
    println!("{}", grid);

    // extent of 1 should just return the target index
    let selection_extent_1 = grid.get_selection_indexes(SelectionType::Square(1), Index{x: 2, y: 2}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 2, y: 2}]), selection_extent_1);
    
    // extent of 2 should return a cross-shape around the target index
    let selection_extent_2 = grid.get_selection_indexes(SelectionType::Square(2), Index{x: 2, y: 2}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 1, y: 1}, Index{x: 1, y: 2}, Index{x: 1, y: 3}, Index{x: 2, y: 1}, Index{x: 2, y: 2}, Index{x: 2, y: 3}, Index{x: 3, y: 1}, Index{x: 3, y: 2}, Index{x: 3, y: 3}]), selection_extent_2);

    // extent of 2 should return a cross-shape around the target index
    let selection_extent_3 = grid.get_selection_indexes(SelectionType::Square(2), Index{x: 0, y: 0}).unwrap();
    assert_eq!(HashSet::from_iter(vec![Index{x: 0, y: 0}, Index{x: 0, y: 1}, Index{x: 1, y: 0}, Index{x: 1, y: 1}]), selection_extent_3);
}