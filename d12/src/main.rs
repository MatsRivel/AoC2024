use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

use matrix::Matrix;
use plant::Plant;
use position::Position;

mod position{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position{
        x: usize,
        y: usize
    }
    impl Position{
        pub fn new(x:usize,y:usize)->Self{
            Self{x,y}
        }
        fn adjusted(&self, dy:i32,dx:i32)->Option<Self>{
            match (self.y != 0 || dy != -1) && (self.x != 0 || dx != -1){
                true => {
                    let new_x = self.x as i32 + dx;
                    let new_y = self.y as i32 + dy;
                    debug_assert!(new_x >= 0);
                    debug_assert!(new_y >= 0);
                    Some(Self::new((self.x as i32 + dx) as usize, (self.y as i32 +dy) as usize))},
                false => None,
            }
        }
        pub fn neighbours(&self)->[Option<Position>;4]{
            [[-1,0],[1,0],[0,-1],[0,1]].into_iter().map(|[dy,dx]| self.adjusted(dy, dx)).collect::<Vec<Option<Position>>>().try_into().unwrap()
        }
        pub fn x(&self)->usize{
            self.x
        }
        pub fn y(&self)->usize{
            self.y
        }
    }
}

mod edge{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Edge{
        Open,
        Blocked
    }
    impl Edge{
        pub fn is_open(&self)->bool{
            match self{
                Edge::Open => true,
                Edge::Blocked => false,
            }
        }
    }
}
mod plant{
    use std::fmt::Display;

    use crate::position::Position;
    #[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Plant{
        pos: Position,
        id: char,
        valid_neighbours: [Option<Position>;4]
    }
    impl Plant{
        pub fn new(pos: Position, id:char, valid_neighbours: [Option<Position>; 4])->Self{
            Self { pos, id, valid_neighbours }
        }
        pub fn pos(&self)->Position{
            self.pos
        }
        pub fn id(&self)->char{
            self.id
        }
        pub fn neighbours(&self)->[Option<Position>;4]{
            self.valid_neighbours
        }
    }
    impl Display for Plant{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let y = self.pos.y();
            let x = self.pos.x();
            let id = self.id();
            let neighbour_count = self.neighbours().into_iter().filter_map(|v|v).count();
            write!(f,"({y},{x}) {id} #{neighbour_count}")
        }
    }
}
mod matrix{
    use crate::{plant::Plant, position::Position};

    pub struct Matrix<T>{
        width: usize,
        height: usize,
        data: Vec<T>,
        iter_idx: usize,
    }
    impl <T:Copy>Matrix<T>{
        pub fn new_from_square(data:Vec<Vec<T>>)->Self{
            let height = data.len();
            let width = data[0].len();
            let data = data.into_iter().flat_map(|row| row).collect();
            Self::new_from_flat(width, height, data)
        }
        pub fn new_from_flat(width: usize, height: usize, data:Vec<T>)->Self{
            Self { width, height, data , iter_idx:0}
        }
        fn pos_to_idx(&self,pos:Position)->Option<usize>{
            if pos.x() >= self.width || pos.y() >= self.height{
                return None;
            }
            Some(pos.y()*self.width+pos.x())
        }
        pub fn idx_to_pos(&self,idx:usize)->Option<Position>{
            if idx < self.data.len(){
                let y = idx / self.width;
                let x = idx % self.width;
                Some(Position::new(x, y))
            }else{
                None
            }
        }
        pub fn get(&self, pos: Position)->Option<T>{
            let idx = self.pos_to_idx(pos)?;
            self.data.get(idx).copied()
        }
        pub fn set(&mut self, pos: Position, value: T){
            if let Some(idx) = self.pos_to_idx(pos){
                self.data[idx] = value;
            }
        }
        pub fn width(&self)->usize{
            self.width
        }
        pub fn height(&self)->usize{
            self.height
        }
        pub fn clone_data(&self)->Vec<T>{
            self.data.clone()
        }
    }
    impl Matrix<Plant>{
        pub fn print(&self){
            for y in 0..self.height{
                for x in 0..self.width{
                    let p = Position::new(x, y);
                    print!("{}",self.get(p).unwrap().id())
                }
                println!();
            }
        }
    }
    impl <T:Copy>Iterator for Matrix<T>{
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.iter_idx >= self.data.len(){
                None
            }else{
                let value = self.data[self.iter_idx];
                self.iter_idx+=1;
                Some(value)
            }
        }
    }
}

fn get_data(s:&str)->Matrix<Plant>{
    let square = s.lines().map(|line| line.chars().collect()).collect();
    let chars: Matrix<char> = Matrix::new_from_square(square);
    let width = chars.width();
    let height = chars.height();
    let plants_data = chars.clone_data().iter().enumerate().map(|(idx, &id)|{
        let pos = chars.idx_to_pos(idx).unwrap();
        let neighbours = pos.neighbours().into_iter().map(|n|{
            match n{
                Some(neighbour) => {
                    println!("{pos:?}->{neighbour:?}");
                    if chars.get(neighbour) == chars.get(pos){
                        Some(neighbour)
                    }else{
                        None
                    }
                },
                None => None,
            }
        }).collect::<Vec<Option<Position>>>().try_into().unwrap();

        Plant::new(pos,id,neighbours)
        }).collect::<Vec<Plant>>();
    let plants = Matrix::new_from_flat(width, height, plants_data);
    plants
}

fn solve1(data:&Matrix<Plant>)->usize{
    data.print();
    let mut total = 0;
    let mut seen = HashSet::<Position>::new();
    let total_length = data.width()*data.height();
    // Iter through all values
    for idx in 0..total_length{
        // Skip any visited values
        let pos = data.idx_to_pos(idx).unwrap();
        if seen.contains(&pos){
            continue;
        }
        println!("____");
        // Go through neighbours of current.
        let mut queue = VecDeque::new();
        queue.push_front(pos);
        let mut area = 0;
        let mut circumference = 0;
        while let Some(next_pos) = queue.pop_front(){
            if seen.contains(&next_pos){
                continue;
            }else{
                seen.insert(next_pos);
            }
            let plant = data.get(next_pos).unwrap();
            println!("{}",plant.id());
            area += 1;
            circumference += plant.neighbours().into_iter().filter(|v| v.is_none()).count();
            plant.neighbours().into_iter().filter_map(|n|n).for_each(|n|queue.push_front(n));

        }
        let current_plant = data.get(pos).unwrap();
        println!("A: {area}, C: {circumference}, Root plant: {current_plant}");
        total += area*circumference;

    }
    total
}
fn main() {
    let file_name = "TestData1.txt";
    let expected = 772;
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let solution1 = solve1(&data);
    assert_eq!(solution1,expected)
}
#[cfg(test)]
mod tests{
    use std::fs::read_to_string;

    use super::*;
    #[test]
    fn edge_test_none(){
        let s = "AAA\nAAA\nAAA";
        let data = get_data(&s);
        let pos = Position::new(1,1);
        let plant = data.get(pos).unwrap();
        let edge_count = plant.neighbours().into_iter().filter(|n| n.is_none()).count();
        assert_eq!(edge_count,0);
    }
    #[test]
    fn edge_test_all(){
        let s = "AAA\nABA\nAAA";
        let data = get_data(&s);
        let pos = Position::new(1,1);
        let plant = data.get(pos).unwrap();
        let edge_count = plant.neighbours().into_iter().filter(|n| n.is_none()).count();
        assert_eq!(edge_count,4);
    }
    #[test]
    fn edge_test_some1(){
        let s = "AAA\nABB\nABA";
        let data = get_data(&s);
        let pos = Position::new(1,1);
        let plant = data.get(pos).unwrap();
        let edge_count = plant.neighbours().into_iter().filter(|n| n.is_none()).count();
        assert_eq!(edge_count,2);
    }
    #[test]
    fn edge_test_some2(){
        let s = "AAA\nCBB\nABA";
        let data = get_data(&s);
        let pos = Position::new(0,0);
        let plant = data.get(pos).unwrap();
        let edge_count = plant.neighbours().into_iter().filter(|n| n.is_none()).count();
        assert_eq!(edge_count,3);
    }
    #[test]
    fn solve_test1_1(){
        let file_name = "TestData1.txt";
        let expected = 772;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve1(&data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test1_2(){
        let file_name = "TestData2.txt";
        let expected = 1930;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve1(&data);
        assert_eq!(solution1,expected)
    }
}