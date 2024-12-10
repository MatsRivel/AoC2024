use std::{collections::HashSet, fmt::Display, fs::read_to_string};

#[derive(Debug,Clone,Copy,PartialEq,Eq, PartialOrd, Ord, Hash)]
enum Tile{
    Player,
    Wall,
    CustomWall,
    Space,
    Visited
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self{
            Tile::Player => 'O',
            Tile::Wall => '#',
            Tile::CustomWall => 'X',
            Tile::Space => ' ',
            Tile::Visited => '.'
        };
        write!(f,"{c}")
    }
}
impl From<char> for Tile{
    fn from(value: char) -> Self {
        match value{
            '.' => Self::Space,
            '^' => Self::Player,
            '#' => Self::Wall,
            _ => panic!("Invalid character \"{value}\"")
        }
    }
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Clone, Copy, Hash)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}
impl Direction{
    fn new()->Self{
        Self::Up
    }
    fn rotate(self)->Self{
        match self{
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn as_vector(&self)->(i8,i8){
        match self{
            Direction::Up    => (-1, 0),
            Direction::Down  => ( 1, 0),
            Direction::Left  => ( 0,-1),
            Direction::Right => ( 0, 1),
        }
    }
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position{
    y: usize,
    x: usize
}
#[derive(Debug,Clone,Copy)]
struct PositionConstructor{
    xmax: usize,
    ymax: usize
}
impl PositionConstructor{
    fn step(self, pos: Position, direction: Direction)->Option<Position>{
        let (dy,dx) = direction.as_vector();
        if (pos.y == 0 && dy == -1) || (pos.x == 0 && dx == -1){
            None
        }else if  (pos.y+1 == self.ymax && dy == 1) || (pos.x+1 == self.xmax && dx == 1){
            None
        }else{
            let x = (pos.x as i64 + dx as i64) as usize;
            let y= (pos.y as i64 + dy as i64) as usize;
            self.new_pos(y, x)
        }
    }
    fn new_pos(&self, y:usize, x:usize)->Option<Position>{
        if y >= self.ymax || x >= self.xmax{
            None
        }else{
            Some(Position{y,x})
        }
    }
}
enum LoopOrNot{
    Loop(HashSet<(Position,Direction)>),
    NotLoop(HashSet<(Position,Direction)>)
}
impl LoopOrNot{
    fn is_loop(&self)->bool{
        match self{
            LoopOrNot::Loop(_) => true,
            LoopOrNot::NotLoop(_) => false,
        }
    }
}

#[derive(Clone,Debug)]
struct Matrix{
    position_constructor: PositionConstructor,
    initial_player_pos: Position,
    player_pos: Position,
    direction: Direction,
    data: Vec<Tile>
}
impl Matrix{
    pub fn from_string(s:&str)->Self{
        let direction = Direction::new();
        let mut xlen = 0;
        let data: Vec<Tile> = s
            .lines()
            .map(|l| {
                match l.strip_suffix('\r'){
                    Some(v) => v,
                    None => l,
                }
            })
            .flat_map(|l|{
                xlen = l.len(); // Todo: Find better solution than re-setting this each time?
                l.chars().map(move |c| {
                    let tile: Tile = c.into();
                    tile
                })
            }).collect();
        let ylen = data.len() / xlen;
        let (player_idx,_) = data.iter().enumerate().find(|&(_, tile)| *tile == Tile::Player).unwrap();
        let data = data.into_iter().map(|tile| {
            match tile{
                Tile::Player => Tile::Space,
                _ => tile
            }
        }).collect();
        let (y,x) = (player_idx / xlen, player_idx % xlen);
        let position_constructor = PositionConstructor{ymax:ylen,xmax:xlen};
        let player_pos = position_constructor.new_pos(y, x).expect("We know the initial state has exactly one player.");
        Self{position_constructor, initial_player_pos: player_pos, player_pos, direction, data}
    }
    pub fn from_file(file_path: &str)->Self{
        let s = read_to_string(file_path).unwrap();
        Self::from_string(&s)
    }
    fn idx_to_pos(&self,idx:usize)->Option<Position>{
        let xlen = self.position_constructor.xmax;
        let (y,x) = (idx / xlen, idx % xlen);
        self.new_pos(y, x)
    }
    fn pos_to_idx(&self, p:&Position)->usize{
        let xlen = self.position_constructor.xmax;
        let (y,x) = (p.y,p.x);
        xlen*y+x
    }
    fn get_pos(&self, position: &Position)->Option<&Tile>{
        let idx = self.pos_to_idx(position);
        self.data.get(idx)
    }
    fn set_pos(&mut self, position: &Position, value: Tile){
        let idx = self.pos_to_idx(position);
        self.data[idx] = value;
    }
    fn step(&mut self)->Option<Position>{
        self.position_constructor.step(self.player_pos, self.direction)
    }
    fn rotate(&mut self){
        self.direction = self.direction.rotate();
    }
    fn new_pos(&self,y:usize,x:usize)->Option<Position>{
        self.position_constructor.new_pos(y, x)
    }
    pub fn solve1(mut self)->i32{
        let visited = match self.get_path(){
            LoopOrNot::NotLoop(hash_set) => hash_set,
            LoopOrNot::Loop(_) => panic!("First path should not have loops."),
        };
        let direction_ignored = visited.into_iter().map(|(pos,_)| pos).collect::<HashSet<Position>>();
        direction_ignored.iter().fold(0, |acc,_| acc+1)
    }

    fn get_path(&mut self)->LoopOrNot{
        self.reset_self(); //Todo: Is this a bad idea?
        let mut visited = HashSet::new();
        visited.insert((self.player_pos,self.direction));
        let mut loop_found = false;
        while let Some(new_pos) = self.step(){ // When false: Exited the map on negative edges.

            if let Some(tile) = self.get_pos(&new_pos){
                // #[cfg(debug_assertions)]
                // print!("{new_pos:?} -> {tile:?}");
                match tile{
                    Tile::Wall | Tile::CustomWall => {
                        self.rotate();
                        let previously_seen = !visited.insert((self.player_pos,self.direction));
                        if previously_seen{
                            loop_found = true;
                            break; // Loop found
                        };
                    },
                    _ => {
                        let previously_seen = !visited.insert((new_pos,self.direction));
                        if previously_seen{
                            loop_found = true;
                            break; // Loop found
                        };
                        self.player_pos = new_pos;
                    },
                }
            }else{
                break; // Exicted the map on positive edges
            }
        }
        let output = match loop_found{
            true => {
                self.reset_self();
                // println!("Loop");
                self.print(&Some(visited.clone()));
                LoopOrNot::Loop(visited)
            },
            false => {
                visited.insert((self.player_pos,self.direction));
                self.reset_self();
                // self.print(&Some(visited.clone()));
                LoopOrNot::NotLoop(visited)

            },
        };

        output
    }
    fn reset_self(&mut self){
        self.direction = Direction::new();
        self.player_pos = self.initial_player_pos;
    }

    fn print(&self, visited: &Option<HashSet<(Position,Direction)>>){
        #[cfg(not(debug_assertions))]
        return;
        let mut matrix = self.clone();
        let xlen = self.position_constructor.xmax;
        println!("Current grid:");
        if let Some(set) = visited{
            for (pos,_) in set{
                matrix.set_pos(pos, Tile::Visited);
            }
        }
        let mut grid: Vec<Vec<Tile>> = matrix.data.into_iter().enumerate().fold(vec![], |mut agg,(idx, tile)|{
            if idx % xlen == 0{
                agg.push(vec![]);
            }
            let last_idx = agg.len()-1;
            let last = agg.get_mut(last_idx).unwrap();
            last.push(tile);
            agg
        });
        let (y,x) = (matrix.player_pos.y, matrix.player_pos.x);
        grid[y][x] = Tile::Player;
        for row in grid.into_iter(){
            for point in row.into_iter(){
                print!("{point}");
            }
            println!();
        }
        println!();
    }
    pub fn solve2(&mut self)->i32{
        let visited = match self.clone().get_path(){
            LoopOrNot::NotLoop(hash_set) => hash_set,
            _ => panic!("Initial path can not have loops"),
        };
        // #[cfg(debug_assertions)]
        // self.print(&None);
        let direction_ignored = visited.into_iter().map(|(pos,_)| pos).collect::<HashSet<Position>>();
        let blocking_walls = direction_ignored.into_iter().filter_map(|pos|{
            #[cfg(debug_assertions)]
            println!("\n\npos: {pos:?}");
            #[cfg(debug_assertions)]{
                let custom_wall_count = self.data.iter()
                    .filter(|tile| {
                        match tile{
                            Tile::CustomWall => true,
                            _ => false
                        }
                    }).count();
                debug_assert!(custom_wall_count==0);
            }
            self.set_pos(&pos, Tile::CustomWall);
            let path = self.get_path();
            let result = match path{
                LoopOrNot::Loop(_) => Some(pos),
                LoopOrNot::NotLoop(_) => None, // If this is None, we found a loop.
            };
            self.set_pos(&pos, Tile::Space);
            self.reset_self();
            #[cfg(debug_assertions)]
            println!("Custom Wall result: {pos:?} -> {result:?}");
            result
        }).collect::<Vec<Position>>();

        blocking_walls.len() as i32
    }
}

fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let matrix = Matrix::from_file(file_name);
    let solution1 = matrix.clone().solve1();
    assert_eq!(solution1, 4967);
    let solution2 = matrix.clone().solve2();
    assert!(solution2 < 1901);
    let end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Total time: {:?}", end-start);
}

#[cfg(test)]
mod tests{
    use super::*;
    mod solve_cases{
        use super::*;
        #[test]
        fn solves1(){
            let expected = 41;
            let file_name = "TestData1.txt";
            let matrix = Matrix::from_file(file_name);
            let actual = matrix.solve1();
            assert_eq!(actual,expected);
        }
        #[test]
        fn solves2(){
            let expected = 6;
            let file_name = "TestData1.txt";
            let mut matrix = Matrix::from_file(file_name);
            let actual = matrix.solve2();
            assert_eq!(actual,expected);
        }
        #[test]
        fn solves_ivars_case(){
            let expected = 5331;
            let file_name = "Ivar_input.txt";
            let matrix = Matrix::from_file(file_name);
            let actual = matrix.solve1();
            assert_eq!(actual,expected);
        }
    }
    mod detect_loops{
            use super::*;
        #[test]
        fn can_detect_loops(){

            let file_name = "Looping.txt";
            let mut matrix = Matrix::from_file(file_name);
            let actual = matrix.get_path();
            assert!(actual.is_loop())
        }
        #[test]
        fn can_detect_tiny_loops(){
            let file_name = "Looping_small.txt";
            let mut matrix = Matrix::from_file(file_name);
            let actual = matrix.get_path();
            assert!(actual.is_loop())
        }
        #[test]
        fn can_detect_non_loops(){
            let file_name = "Not_looping.txt";
            let mut matrix = Matrix::from_file(file_name);
            let actual = matrix.get_path();
            assert!(!actual.is_loop())
        }
    }
    mod create_loops{
        use super::*;
        #[test]
        fn can_create_tiny_loops(){
            let expected = 1;
            let file_name = "Almost_looping_small.txt";
            let mut matrix = Matrix::from_file(file_name);
            let initial = matrix.clone().get_path();
            assert!(!initial.is_loop());
            let n_blocking = matrix.solve2();
            // let expected_wall_position = matrix.new_pos(1, 2).unwrap();
            // let expected_wall = matrix.get_pos( &expected_wall_position).unwrap();
            // assert_eq!(&Tile::CustomWall, expected_wall);
            assert_eq!(matrix.player_pos, matrix.initial_player_pos);
            assert_eq!(n_blocking,expected);
        }
        #[test]
        fn can_create_loops(){
            let expected = 1;
            let file_name = "Almost_looping.txt";
            let mut matrix = Matrix::from_file(file_name);
            let expected_pos = matrix.new_pos(1, 1).expect("Known to exist");
            assert_eq!(matrix.player_pos,expected_pos);
            let initial = matrix.clone().get_path();
            assert!(!initial.is_loop());
            let n_blocking = matrix.solve2();

            assert_eq!(n_blocking,expected);
        }
    }
    mod place_walls{
        use super::*;
        #[test]
        fn can_place_custom_walls(){
            let s = "..\n^.";
            let mut matrix = Matrix::from_string(s);
            let p = matrix.new_pos(0, 0).unwrap();
            matrix.set_pos(&p, Tile::CustomWall);
            let expected = Tile::CustomWall;
            let actual = *matrix.get_pos(&p).unwrap();
            assert_eq!(expected,actual)
        }
        #[test]
        fn can_place_custom_walls2(){
            let s = ".\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n.\n^\n.";
            let mut matrix = Matrix::from_string(s);
            let p = matrix.new_pos(11, 0).unwrap();
            matrix.set_pos(&p, Tile::CustomWall);
            let expected = Tile::CustomWall;
            let actual = *matrix.get_pos(&p).unwrap();
            assert_eq!(expected,actual)
        }
        #[test]
        fn can_place_custom_walls3(){
            let s = "............^.";
            let mut matrix = Matrix::from_string(s);
            let p = matrix.new_pos(0, 11).unwrap();
            matrix.set_pos(&p, Tile::CustomWall);
            let expected = Tile::CustomWall;
            let actual = *matrix.get_pos(&p).unwrap();
            assert_eq!(expected,actual)
        }
    }

    #[test]
    fn idx_conversion(){
        let file_name = "TestData1.txt";
        let matrix = Matrix::from_file(file_name);
        for i in 0..matrix.data.len(){
            let pos = matrix.idx_to_pos(i).unwrap();
            let idx = matrix.pos_to_idx(&pos);
            assert_eq!(i,idx);
        }
    }

}
