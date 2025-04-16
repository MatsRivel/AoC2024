use std::fs::read_to_string;

use crate::{position::Position, tile::Tile, IntType};

pub struct Matrix<T>{
    data: Vec<Vec<T>>,
    goal: Position<IntType>,
    start: Position<IntType>
}
impl <T:Copy>Matrix<T>{
    pub fn get(&self, pos: Position<IntType>)->Option<T>{
        if pos.x() < 0 || pos.y() < 0{return None;}
        let x = pos.x() as usize;
        let y = pos.y() as usize;
        if y < self.data.len() && x < self.data[y].len(){
            Some(self.data[y][x])
        }else{
            None
        }
    }
    pub fn set(&mut self, pos: Position<IntType>, value: T){
        if pos.x() < 0 || pos.y() < 0{ panic!("Out of (lower) bounds!");}
        let x = pos.x() as usize;
        let y = pos.y() as usize;
        if y < self.data.len() && x < self.data[y].len(){
            self.data[y][x] = value
        }else{ panic!("Out of (upper) bounds!")}
    }
    pub fn get_data(&self)->&Vec<Vec<T>>{
        &self.data
    }
    pub fn get_end(&self)->Position<IntType>{
        return self.goal;
    }
    pub fn get_start(&self)->Position<IntType>{
        return self.start;
    }
}
impl Matrix<Tile>{
    pub fn new(file_path: &str)->Self{
        let s = read_to_string(file_path).unwrap();
        Self::from(s)
    }

    pub fn get_map(&self)->Vec<Vec<char>>{
        self.data.iter().map(|row|{
            row.iter().map(|&tile| tile.into()).collect()
        }).collect()
    }
}
fn find_end_in_matrix(tiles: &Vec<Vec<Tile>>)->Position<IntType>{
    find_tile_in_matrix(tiles,Tile::End)
}
fn find_start_in_matrix(tiles: &Vec<Vec<Tile>>)->Position<IntType>{
    find_tile_in_matrix(tiles,Tile::Start)
}
fn find_tile_in_matrix(tiles: &Vec<Vec<Tile>>,target:Tile)->Position<IntType>{
    for (y,row) in tiles.iter().enumerate(){
        for (x,tile) in row.iter().enumerate(){
            if *tile == target{
                return Position::new(x as i32, y as i32);
            }
        }
    }
    unreachable!("There is always at least one end in a valid matrix");
}
impl From<String> for Matrix<Tile>{
    fn from(value: String) -> Self {
        let tiles = value.lines().map(|line| line.chars().filter_map(|c| {
            match c{
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Space),
                'S' => Some(Tile::Start),
                'E' => Some(Tile::End),
                _ => None
            }
        }).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();
        let goal = find_end_in_matrix(&tiles);
        let start = find_start_in_matrix(&tiles);
        Self { data: tiles, goal, start }
    }
}
impl From<&Matrix<Tile>> for Matrix<Option<i32>>{
    fn from(matrix: &Matrix<Tile>) -> Self {
        let data = matrix.get_data().iter().map(|row| row.iter().map(|tile|{
            match tile{
                Tile::Wall => None,
                Tile::End => Some(0),
               _ => Some(i32::MAX/2),
            }
        }).collect()).collect();
        Matrix{data,goal:matrix.goal,start:matrix.start}
    }
}
