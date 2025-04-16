use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::{actors::{Actor, IntType, Movable}, direction::Direction, matrix::Matrix, position::Position, tile::Tile};


type DefaultValueType= f32;
type DefaultKeyType = Actor;
#[derive(Debug)]
struct DefaultMap{
    default: DefaultValueType,
    data: HashMap<DefaultKeyType,DefaultValueType>
}
impl DefaultMap{
    pub fn new(default_value: DefaultValueType) -> Self{
        Self { default:default_value, data: HashMap::new() }
    }
    pub fn get_or_default(&self, key: DefaultKeyType)->DefaultValueType{
        match self.data.get(&key){
            Some(v) =>*v,
            None => self.default,
        }
    }
    pub fn set(&mut self, key: DefaultKeyType, value: DefaultValueType){
        self.data.insert(key, value);
    }
}


fn d(current_pos:Position<IntType>, neighbour_pos:Position<IntType>)->DefaultValueType{
    // ((current_pos.x() - neighbour_pos.x()).abs() + (current_pos.y() - neighbour_pos.x()).abs()) as u32   
    let xa = current_pos.x() as DefaultValueType;
    let xb = neighbour_pos.x() as DefaultValueType;
    let ya = current_pos.y() as DefaultValueType;
    let yb = neighbour_pos.y() as DefaultValueType;
    ((xa-xb).powf(2.0) + (ya-yb).powf(2.0)).sqrt()
    // (xa-xb).abs() + (ya-yb).abs()
}

fn get_path_length(matrix: &Matrix::<Tile>, came_from: HashMap<Position<IntType>, Position<IntType>>, current: Position<IntType>, start: Position<IntType>)->u32{
    let mut map = matrix.get_map();
    let mut count = 0;
    let mut active = current;
    #[cfg(debug_assertions)]{
    println!("--- Raw Map ---");
    for row in map.iter(){
        for c in row{
            print!("{c}");
        }
        println!();
    }
    println!();
    }
    
    while let Some(&next) = came_from.get(&active){
        // #[cfg(debug_assertions)]
        {
            if count > 100{
                println!("Maximum stepcount reached!");
                break;
            }
            println!("{active}->{next}");
        }

        let x = next.x() as usize;
        let y = next.y() as usize;
        if map[y][x] == '#'{
            map[y][x] = 'X';
        }else{
            map[y][x] = 'o';
        }
        count += 1;
        active = next;
        if next == start{
            break;
        }
    };
    // #[cfg(debug_assertions)]{
    println!("--- Mod Map ---");
    for row in map.iter(){
        for c in row{
            print!("{c}");
        }
        println!();
    }
    println!();
    // }
    count
}

fn explicit_compare(va: f32, vb: f32)->Ordering{
    match va.partial_cmp(&vb){
        Some(v) => v,
        None => std::cmp::Ordering::Equal,
    }
}

pub fn astar(matrix: &Matrix::<Tile>, initial_actor: Actor)->Option<u32>{
    const DIRS: [Direction;4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    let start = initial_actor.get_pos();
    let end = matrix.get_end();
    println!("start: {start}, end: {end}");
    let h = |pos_in: Position<IntType>| (((pos_in.x()-end.x()) as f32).powf(2.0) + ((pos_in.y()-end.y()) as f32).powf(2.0)).sqrt();
    let mut open_set = HashSet::<Actor>::new();
    open_set.insert(initial_actor);
    
    let mut came_from = HashMap::new();
    
    let mut g_score  = DefaultMap::new(f32::MAX/2.0);
    g_score.set(initial_actor, 0.000);

    let mut f_score = DefaultMap::new(f32::MAX/2.0);
    f_score.set(initial_actor, h(start));

    // Priority Queue would improve this lines performance.
    while let Some((&current, _score)) = open_set.iter().map(|pos| (pos, f_score.get_or_default(*pos))).min_by(|(_,va),(_,vb)| explicit_compare(*va, *vb)){
        // println!("Outer: {}",current.get_pos());
        let current_pos = current.get_pos();
        if current_pos == end{
            return Some(get_path_length(matrix, came_from, current_pos, start));
        }
        
        assert!(open_set.remove(&current)); // If we try to remove something that is not here, we've fucked up
        for neighbour in DIRS.iter().filter_map(|dir| current.try_move(*dir, matrix)){
            assert!(neighbour != current, "We can never be our own neighbour.");
            // if neighbour.get_pos() == start{
            //     continue;
            // }
            // assert!(neighbour.get_pos() != start, "We should never return to start once we start moving.");
            // println!("\tInner: {}",neighbour.get_pos());
            let neighbour_pos = neighbour.get_pos();
            let d_score = d(current_pos, neighbour_pos);
            let tentative_score = g_score.get_or_default(current) + d_score;

            if tentative_score < g_score.get_or_default(neighbour){
                came_from.insert(neighbour_pos, current_pos);
                g_score.set(neighbour, tentative_score);
                f_score.set(neighbour,tentative_score + h(neighbour_pos));
                if !open_set.contains(&neighbour){
                    open_set.insert(neighbour);
                }
            } 
        }
    }
    return None; // Failiure
}

