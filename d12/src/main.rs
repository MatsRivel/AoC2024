use std::collections::HashMap;

use plant::Plant;
use position::Position;

mod position{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Position{
        x: usize,
        y:usize
    }
    impl Position{
        pub fn new(x:usize,y:usize)->Self{
            Self{x,y}
        }
        pub fn adjusted(&self, dy:i32,dx:i32)->Option<Self>{
            match (self.y > 0 || dy != -1) && (self.x > 0 || dx != 1){
                true => Some(Self::new((self.x as i32 +dx) as usize, (self.y as i32 +dy) as usize)),
                false => None,
            }
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
    use crate::{edge::Edge, position::Position};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Plant{
        pos: Position,
        id: char,
        edges: [Edge;4]
    }
    impl Plant{
        pub fn new(pos: Position, id: char, edges:[Edge;4])->Self{
            Self { pos, id, edges}
        }
        pub fn id(&self)->char{
            self.id
        }
        pub fn get_edges(&self)->&[Edge;4]{
            &self.edges
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlantBuilder{
        pos: Position,
        id: char,
    }
    impl PlantBuilder{
        pub fn new(pos: Position, id: char)->Self{
            Self{pos,id}
        }
        pub fn build(self, edges: [Edge;4])->Plant{
            Plant{ pos: self.pos, id: self.id, edges}
        }
        pub fn id(&self)->char{
            self.id
        }
    }
}
mod data{
    use std::collections::{HashMap, HashSet};

    use crate::{edge::Edge, plant::{Plant, PlantBuilder}, position::Position};
    pub struct Data<T>{
        data: Vec<Vec<T>>
    }
    impl <T:Clone>Data<T>{
        pub fn get(&self, pos: &Position)->Option<&T>{
            self.data.get(pos.y())?.get(pos.x())
        }
        pub fn shape(&self)->[usize;2]{
            [self.data.len(),self.data[0].len()]
        }
    }
    impl Data<PlantBuilder>{
        fn new(s:&str)->Self{
            let builders: Vec<Vec<PlantBuilder>> = s.lines()
                .enumerate()
                .map(|(y,line)| {
                    line.char_indices()
                        .map(|(x,c)| {
                            let pos = Position::new(x, y);
                            PlantBuilder::new(pos,c)
                        }).collect()
                }).collect();
            Self { data: builders }
        }
    }
    impl Data<Plant>{
        pub fn new(s:&str)->Self{
            let builders= Data::<PlantBuilder>::new(s);
            let mut data = vec![Vec::with_capacity(builders.shape()[1]);builders.shape()[0]];
            println!("{:?}", builders.shape());
            println!("{:?},{:?}", data.len(),data[0].len());

            for y in 0..builders.shape()[0]{
                for x in 0..builders.shape()[1]{
                    let current_pos = Position::new(x, y);
                    let current_plant = builders.get(&current_pos).unwrap();
                    let mods = [[-1,0],[1,0],[0,-1],[0,1]];
                    let pre_edges = mods.into_iter()
                        .filter_map(|[dy,dx]|Position::new(x, y).adjusted(dy, dx))
                        .map(|p|{
                            if let Some(neighbour) = builders.get(&p){
                                if neighbour.id() == current_plant.id(){
                                    Edge::Blocked
                                }else{
                                    Edge::Open
                                }
                            }else{
                                Edge::Open
                            }
                        })
                        .collect::<Vec<Edge>>();
                    let mut edges = [Edge::Blocked;4];
                    for (i,e) in pre_edges.into_iter().enumerate(){
                        edges[i] = e;
                    }
                    data[current_pos.y()].push(current_plant.build(edges));
                }
            }
            Self{data}
        }
        pub fn segregate(self)->HashMap<char,Vec<Plant>>{
            let set: HashSet<char> = self.data.iter().flat_map(|row| row.iter().map(|plant| plant.id()).collect::<Vec<char>>()).collect();
            let mut map: HashMap<char,Vec<Plant>> = set.into_iter().map(|c| (c,vec![])).collect();
            self.data.into_iter().flat_map(|row| row).for_each(|plant| {
                if let Some(v) = map.get_mut(&plant.id()){
                    v.push(plant);
                }else{
                    panic!("Should never have any not yet seen chars here")
                }
            });
            map
        }
    }
}

fn solve1(map: &HashMap<char,Vec<Plant>>)->usize{
    let mut output = 0;
    for row in map.values(){
        let area = row.len();
        let circumference = row.iter().map(|plant| plant.get_edges().iter().filter(|edge| edge.is_open()).count()).sum::<usize>();
        output += area*circumference
   }
    output
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests{
    use std::fs::read_to_string;

    use data::Data;

    use super::*;
    #[test]
    fn solve_test1_1(){
        let file_name = "TestData1.txt";
        let expected = 772;
        let s = read_to_string(file_name).unwrap();
        let map = Data::<Plant>::new(&s).segregate();
        let solution1 = solve1(&map);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test1_2(){
        let file_name = "TestData2.txt";
        let expected = 1930;
        let s = read_to_string(file_name).unwrap();
        let map = Data::<Plant>::new(&s).segregate();
        let solution1 = solve1(&map);
        assert_eq!(solution1,expected)
    }
}