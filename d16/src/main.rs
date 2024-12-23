use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fmt::Display, fs::read_to_string};

use support::{astar::AStartTraversible, direction::Direction, matrix::{GetSet, Matrix}, position::{self, Position}};
type IntType = i32;
type DataType = Tile;
type IndexType = Position<IntType>;
type MyMatrix = Matrix<DataType, IntType, Vec<DataType>, IndexType>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile{
    Wall,
    Space,
    Start,
    End,
    Visited
}
impl Tile{
    pub fn new(c: char)->Option<Self>{
        match c{
            '#' => Some(Self::Wall),
            '.' => Some(Self::Space),
            'S' => Some(Self::Start),
            'E' => Some(Self::End),
            _ => None

        }
    }
    pub fn is_wall(&self)->bool{
        match self{
            Tile::Wall => true,
            Tile::Space => false,
            Tile::Start => false,
            Tile::End => false,
            Tile::Visited => false,
        }
    }
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self{
            Tile::Wall => '#',
            Tile::Space => ' ',
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Visited => '.',
        };
        write!(f,"{c}")
    }
}

fn reconstruct_path(came_from: HashMap<PosRot,PosRot>, current: PosRot)->Vec<PosRot>{
    let mut output = vec![current];
    let mut p = *came_from.get(&current).unwrap();
    loop{
        output.push(p);
        match came_from.get(&p){
            Some(v) => p = *v,
            None => break,
        }
    }
    output
}

struct DefaultHashMap{
    data: HashMap<PosRot, IntType>,
    default: IntType
}
impl DefaultHashMap{
    pub fn new(default_value: IntType)->Self{
        Self { data: HashMap::new(), default: default_value }
    }
    pub fn get(&self, key: PosRot)->Option<IntType>{
        match self.data.get(&key){
            Some(v) => Some(*v),
            None => Some(self.default),
        }
    }
    pub fn set(&mut self, key: PosRot, value: IntType){
        self.data.insert(key, value);
    }
}
struct DefaultHeap{
    data: BinaryHeap<(Reverse<IntType>, PosRot)>,
    default: IntType
}
impl DefaultHeap{
    pub fn new(default_value: IntType)->Self{
        Self { data: BinaryHeap::new(), default: default_value }
    }
    pub fn push(&mut self, item: (IntType,PosRot)){
        self.data.push((Reverse(item.0),item.1));
    }
    pub fn pop(&mut self)->Option<(IntType,PosRot)>{
        if let Some((i,pos) ) =self.data.pop(){
            let unreversed = i.0;
            return Some((unreversed,pos));
        }
        None
    }
    pub fn peek(&self)->Option<(IntType,PosRot)>{
        if let Some((i,pos) ) =self.data.peek(){
            let unreversed = i.0;
            return Some((unreversed,*pos));
        }
        None
    }
}
fn step_cost(from: PosRot, to: PosRot)->IntType{
    let base_distance = ((from.pos.x() - to.pos.x()).abs() + (from.pos.y() - to.pos.y()).abs()) * MOVE_COST;
    let mut modifier = 0;
    if from.rot == to.rot.flip(){
        modifier += 2;
    }else if from.rot == to.rot.rot_left() || from.rot == to.rot.rot_right(){
        modifier += 1;
    }
    base_distance + modifier*ROTATION_COST
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PosRot{
    pos: Position<IntType>,
    rot: Direction

}
impl PosRot{
    pub fn new(pos: Position<IntType>, rot: Direction)->Self{
        Self{pos,rot}
    }
    pub fn neighbours(&self)->[Self;3]{
        [   Self::new(self.pos, self.rot.rot_left()),
            Self::new(self.pos,self.rot.rot_right()),
            Self::new(self.pos + self.rot.as_vector(), self.rot)
        ]
    }
}
impl PartialEq<Position<IntType>> for PosRot{
    fn eq(&self, other: &Position<IntType>) -> bool {
        self.pos == *other
    }
}
struct ScoreTracker{
    map: HashMap<PosRot,IntType>,
    heap: BinaryHeap<(Reverse<IntType>, PosRot)>,
    default: IntType
}
impl ScoreTracker{
    fn new(default_value: IntType)->Self{
        Self{map: HashMap::new(), heap: BinaryHeap::new(), default: default_value}
    }

    fn peek_min(&self)->Option<(IntType, PosRot)>{
        if let Some((i,pos) ) =self.heap.peek(){
            let unreversed = i.0;
            return Some((unreversed,*pos));
        }
        None
    }
    fn pop_min(&mut self)->Option<(IntType, PosRot)>{
        if let Some((i,pos) ) =self.heap.pop(){
            let unreversed = i.0;
            return Some((unreversed, pos));
        }
        None
    }

    fn get(&self, key: PosRot)->IntType{
        if let Some(v) = self.map.get(&key){
            *v
        }else{
            self.default
        }
    }
    fn set(&mut self, key: PosRot, value: IntType){
        self.map.insert(key, value);
    }
    fn push_to_heap(&mut self, key: PosRot, value: IntType){
        self.heap.push((Reverse(value),key));
    }
}

fn best_in_open_set(open_set: &HashSet<PosRot>, fscore: &DefaultHashMap)->(PosRot,IntType){
    let mut best = (PosRot::new(Position::new(-1, -1), Direction::Right), IntType::MAX);
    for point in open_set.iter(){
        let score = match fscore.get(*point){
            Some(v) => v,
            None => IntType::MAX,
        };
        if score <= best.1{
            best = (*point,score);
        }
    }
    return best;

}

fn heuristic(pos: PosRot, goal: Position<IntType>)->IntType{
    let base_distance = ((pos.pos.x() - goal.x()).abs() + (pos.pos.y() - goal.y()).abs()) * MOVE_COST;
    let mut modifier = 0;
    // Compensate for rotations to reach X.
    if pos.pos.x() < goal.x(){
        let horizontal_modifier = match pos.rot{
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::None => 0,
        };
        modifier += horizontal_modifier;
    }else if pos.pos.x() > goal.x(){
        let horizontal_modifier = match pos.rot{
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::None => 0,
        };
        modifier += horizontal_modifier;
    }
    // Compensate for rotations to reach Y.
    if pos.pos.y() < goal.y(){
        let vertical_modifier = match pos.rot{
            Direction::Up => 2,
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 1,
            Direction::None => 0,
        };
        modifier += vertical_modifier;
    }else if pos.pos.y() > goal.y(){
        let vertical_modifier = match pos.rot{
            Direction::Up => 0,
            Direction::Down => 2,
            Direction::Left => 1,
            Direction::Right => 1,
            Direction::None => 0,
        };
        modifier += vertical_modifier;
    }
    base_distance + modifier*ROTATION_COST

}

fn get_data(s: &str) -> MyMatrix {
    let data = s.lines().map(|line| line.chars().filter_map(|c| Tile::new(c)).collect() ).collect();
    MyMatrix::new_from_square(data)
}

fn solve1(data: &MyMatrix) -> IntType {
    let start_pos = data.find(&Tile::Start).expect("We know this exists in our data-set");
    let start = PosRot::new(start_pos, Direction::Right);
    let goal = data.find(&Tile::End).expect("We know this exists in our data-set");
    let path = a_star(start, goal, heuristic, data).expect("There is at least one valid path from start to goal.");
    #[cfg(debug_assertions)]
    {   
        let mut data_clone = data.clone();
        path.iter().for_each(|p| data_clone.set(p.pos, Tile::Visited));
        data_clone.set(start_pos, Tile::Start);
        data_clone.set(goal, Tile::End);
        println!("{data_clone}")
    }
    let total_cost = path.iter().zip(path.iter().skip(1)).map(|(from,to)| step_cost(*from, *to)).sum::<IntType>();
    total_cost
}

fn solve2(data: MyMatrix) -> IntType {
    0
}
fn a_star(start: PosRot, goal:Position<IntType>, heuristic: fn(PosRot, Position<IntType>)->IntType, data: & MyMatrix)->Option<Vec<PosRot>>{
    let mut open_set = HashSet::<PosRot>::new();
    open_set.insert(start);
    let mut came_from = HashMap::<PosRot,PosRot>::new();
    
    let mut gscore = DefaultHashMap::new(IntType::MAX);
    gscore.set(start,0);

    let mut fscore = DefaultHashMap::new(IntType::MAX);
    fscore.set(start,heuristic(start, goal) );

    while !open_set.is_empty(){
        let (current_pos, _current_fscore) = best_in_open_set(&open_set, &fscore);
        if current_pos.pos == goal{
            return Some(reconstruct_path(came_from, current_pos));
        }
        open_set.remove(&current_pos);
        let neighbours = current_pos
            .neighbours()
            .into_iter()
            .filter(|n| {
                match data.get(n.pos){
                    None => false,
                    Some(tile) => !tile.is_wall()
                }});
        for neighbour in neighbours{
            let tentative = gscore.get(current_pos).unwrap() + step_cost(current_pos,neighbour);
            if tentative < gscore.get(neighbour).unwrap(){
                came_from.insert(neighbour, current_pos);
                gscore.set(neighbour, tentative);
                fscore.set(neighbour,tentative + heuristic(neighbour, goal));
                if !open_set.contains(&neighbour){
                    open_set.insert(neighbour);
                }
            }
        }
    }
    None
}

impl AStartTraversible<PosRot,Position<IntType>, IntType, MyMatrix, Tile> for MyMatrix
where {
    fn a_start(&self, start: PosRot, goal: Position<IntType>)->Option<Vec<Position<i32>>> {
        todo!()
    }

    fn heuristic(&self, pos: PosRot, goal: Position<IntType>)->IntType {
        let base_distance = ((pos.pos.x() - goal.x()).abs() + (pos.pos.y() - goal.y()).abs()) * MOVE_COST;
        let mut modifier = 0;
        // Compensate for rotations to reach X.
        if pos.pos.x() < goal.x(){
            let horizontal_modifier = match pos.rot{
                Direction::Up => 1,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Right => 0,
                Direction::None => 0,
            };
            modifier += horizontal_modifier;
        }else if pos.pos.x() > goal.x(){
            let horizontal_modifier = match pos.rot{
                Direction::Up => 1,
                Direction::Down => 1,
                Direction::Left => 0,
                Direction::Right => 1,
                Direction::None => 0,
            };
            modifier += horizontal_modifier;
        }
        // Compensate for rotations to reach Y.
        if pos.pos.y() < goal.y(){
            let vertical_modifier = match pos.rot{
                Direction::Up => 2,
                Direction::Down => 0,
                Direction::Left => 1,
                Direction::Right => 1,
                Direction::None => 0,
            };
            modifier += vertical_modifier;
        }else if pos.pos.y() > goal.y(){
            let vertical_modifier = match pos.rot{
                Direction::Up => 0,
                Direction::Down => 2,
                Direction::Left => 1,
                Direction::Right => 1,
                Direction::None => 0,
            };
            modifier += vertical_modifier;
        }
        base_distance + modifier*ROTATION_COST
    }

    fn reconstruct_path(&self, came_from: HashMap<PosRot, PosRot>, current: PosRot)->Vec<PosRot> {
        todo!()
    }
}


const ROTATION_COST: IntType = 1000;
const MOVE_COST: IntType = 1;
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data);
    let s1_end = std::time::Instant::now();
    // assert_eq!(solution1, 37128);
    let solution2 = solve2(data);
    // assert_eq!(solution2,74914228471331);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Parse file time: {:?}", file_end - start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}", s2_end - s1_end);
    println!("Total time: {:?}", s2_end - start);
}



#[cfg(test)]
mod tests{
    use std::fs::read_to_string;

    use crate::{get_data,solve1};

    #[test]
    fn solve_test1_1(){
        let expected = 7036;
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)
    }

    #[test]
    fn solve_test1_2(){
        let expected = 11048;
        let file_name = "TestData2.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)
    }
    #[test]
    fn solve_test1_minimal(){
        let expected = 1;
        let file_name = "TestMinimal.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)
    }
    #[test]
    fn solve_test1_small(){
        let expected = 3004;
        let file_name = "TestSmall.txt";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution = solve1(&data);
        assert_eq!(solution,expected)
    }
}