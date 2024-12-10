use std::{collections::{HashSet, VecDeque}, fs::read_to_string};
use map::Map;
enum Direction{
    Up,
    Down,
    Left,
    Right
}
mod map{
    use super::*;

    pub fn get_data(s: &str) -> Map {
        let input = s.lines().map(|line| line.chars().filter_map(|c| c.to_digit(10)).map(|i| i as u8).collect() ).collect();
        Map::new(input)
    }   

    pub struct Map{
        width: usize,
        height: usize,
        data: Vec<u8>,
        seen: HashSet<usize>
    }
    impl Map{
        pub fn new(input: Vec<Vec<u8>>)->Self{
            let height = input.len();
            let width = input[0].len();
            let data = input.into_iter().flat_map(|v| v).collect();
            Self{width,height,data, seen: HashSet::new()}
        }
        pub fn purge_seen(&mut self){
            self.seen = HashSet::new();
        }
        pub fn get(&self, y:usize, x:usize)->Option<&u8>{
            let idx = self.coord_to_idx(y, x)?;
            self.data.get(idx)
        }

        fn coord_to_idx(&self, y:usize, x:usize)->Option<usize>{
            if self.height <= y || self.width <= x{
                None
            }else{
                Some(y*self.width + x)
            }
        }

        fn idx_to_coord(&self,idx:usize)->(usize,usize){
            (idx/self.height, idx%self.width)
        }

        pub fn find_starting_positions(&self)->VecDeque<(usize,usize)>{
            self.data.iter().enumerate().filter(|(_,i)| **i == 0).map(|(idx,_)| self.idx_to_coord(idx)).collect()
        }
        
        fn try_step(&self, y: usize, x: usize, direction: Direction)->Option<( usize, usize)>{
            let [dy,dx] = match direction{
                Direction::Up    => [-1, 0],
                Direction::Down  => [ 1, 0],
                Direction::Left  => [ 0,-1],
                Direction::Right => [ 0, 1]
            };
            let new_y = match (y,dy){
                (1.., -1) | (0.., 0..) =>  (y as i64 + dy) as usize,
                (_, _) => return None
            };
            let new_x = match (x,dx){
                (1.., -1) | (0.., 0..) =>  (x as i64 + dx) as usize,
                (_, _) => return None
            };
            Some((new_y, new_x))
        }

        fn get_neighbours(&self,y:usize,x:usize)->Vec<(usize,usize)>{
            let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            directions.into_iter().filter_map(|dir| self.try_step(y, x, dir)).collect()
        }
        pub fn get_filtered_neighbours(&self,y:usize,x:usize)->VecDeque<(usize,usize)>{
            if let Some(current) = self.get(y, x){
                self.get_neighbours(y, x)
                    .into_iter()
                    .filter(|(ny,nx)| self.get(*ny, *nx) == Some(&(current+1)))
                    .filter(|(ny,nx)| {
                        let idx = self.coord_to_idx(*ny, *nx).unwrap();
                        !self.seen.contains(&idx)
                    })
                    .collect()
            }else{
                VecDeque::new()
            }
        }
        pub fn add_to_seen(&mut self, y: usize, x: usize){
            if let Some(idx) = self.coord_to_idx(y, x){
                self.seen.insert(idx);
            }
        }
    }
}

fn solve1(data: &mut Map) -> i32 {
    data.purge_seen(); // Making sure we don't use stored data.
    let mut counter= 0;
    for starting_position in data.find_starting_positions(){
        data.purge_seen(); // Reset stored data when switching head.
        let mut upcoming = VecDeque::new();
        upcoming.push_front(starting_position);
        while let Some(next) = upcoming.pop_front(){
            let (y,x) = next;
            data.add_to_seen(y, x);
            if let Some(&current) = data.get(y, x){
                if current == 9{
                    counter += 1;
                } 
            }
            for neighbour in data.get_filtered_neighbours(y, x){
                upcoming.push_front(neighbour);
            }
        }

    }
    counter
}
fn solve2(data: &mut Map) -> i32{
    data.purge_seen(); // Making sure we don't use stored data.
    let mut counter= 0;
    for starting_position in data.find_starting_positions(){
        // data.purge_seen(); // Reset stored data when switching head.
        let mut upcoming = VecDeque::new();
        upcoming.push_front(starting_position);
        while let Some(next) = upcoming.pop_front(){
            let (y,x) = next;
            // data.add_to_seen(y, x);
            if let Some(&current) = data.get(y, x){
                if current == 9{
                    counter += 1;
                } 
            }
            for neighbour in data.get_filtered_neighbours(y, x){
                upcoming.push_front(neighbour);
            }
        }

    }
    counter
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let mut data = map::get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&mut data);
    assert_eq!(solution1,587);
    let s1_end = std::time::Instant::now();
    let solution2 = solve2(&mut data);
    assert_eq!(solution2,1340);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Parse file time: {:?}",file_end-start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}",s2_end-s1_end);
    println!("Total time: {:?}",s2_end-start);
}




#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn solve_test1_1(){
        let file_name = "TestData1.txt";
        let expected = 1;
        let s = read_to_string(file_name).unwrap();
        let mut data = map::get_data(&s);
        let solution1 = solve1(&mut data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test1_2(){
        let file_name = "TestData2.txt";
        let expected = 36;
        let s = read_to_string(file_name).unwrap();
        let mut data = map::get_data(&s);
        let solution1 = solve1(&mut data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test2_1(){
        let file_name = "TestData1.txt";
        let expected = 16;
        let s = read_to_string(file_name).unwrap();
        let mut data = map::get_data(&s);
        let solution1 = solve2(&mut data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test2_2(){
        let file_name = "TestData2.txt";
        let expected = 81;
        let s = read_to_string(file_name).unwrap();
        let mut data = map::get_data(&s);
        let solution1 = solve2(&mut data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test2_3(){
        let file_name = "TestData3.txt";
        let expected = 227;
        let s = read_to_string(file_name).unwrap();
        let mut data = map::get_data(&s);
        let solution1 = solve2(&mut data);
        assert_eq!(solution1,expected)
    }
}