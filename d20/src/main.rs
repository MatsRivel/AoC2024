pub mod position;
pub mod direction;
mod tile;
mod matrix;
use std::{collections::{HashMap, HashSet}, num, ops::{Add, Div, Mul, Sub}, thread::current, u32};
use direction::Direction;
use matrix::Matrix;
use position::Position;
use tile::Tile;

pub type IntType = i32;






fn main() {
    #[cfg(debug_assertions)]
    let path = "TestData.txt";
    // let path = "HomeBrew3.txt";
    #[cfg(not(debug_assertions))]
    let path = "Data.txt";

    let tile_matrix = Matrix::new(path);
    let end = tile_matrix.get_end();

    let mut distance_matrix = Matrix::from(&tile_matrix);

    let mut candidates: HashSet<Position<IntType>> = HashSet::new();
    candidates.insert(end);
    // let mut counter = 0;
    while let Some(&candidate) = candidates.iter().next(){
        let path_cost = distance_matrix.get(candidate).unwrap().unwrap()+1;
        
        let neighbours: Vec<(Position<IntType>,i32)> = candidate.neighbours().iter().filter_map(|&p|p)
            .map(|neighbour| (neighbour,distance_matrix.get(neighbour)))
            .filter(|(_,d)| d.is_some())
            .filter(|(_,d)| d.unwrap().is_some())
            .map(|(p,d)| (p,d.unwrap().unwrap())).collect();
        
        candidates.remove(&candidate);
        for &(neighbour, neighbour_cost) in neighbours.iter(){
            if neighbour_cost > path_cost{
                distance_matrix.set(neighbour, Some(path_cost));
                candidates.insert(neighbour);
            }
        }

    }
    
    #[cfg(not(debug_assertions))]
    let shortcut_value_threshold = 100;
    
    #[cfg(debug_assertions)]
    let shortcut_value_threshold = 1;
    #[cfg(debug_assertions)]
    let mut shortcut_candidates = Vec::new();
    #[cfg(debug_assertions)]
    let mut shortcut_log: HashMap<i32, i32> = HashMap::new();
    
    let mut shortcut_count = 0;
    let mut map = tile_matrix.get_map();
    for y in 1..(map.len()-1){
        for x in  1..(map[y].len()-1){
            let pos = Position::new(x as i32, y as i32);
            // We only look at walls
            if tile_matrix.get(pos) != Some(Tile::Wall){
                continue;
            }
            // Get number of adjacent non-wall tiles.
            let neighbours = pos.neighbours().iter().filter_map(|&p|p)
                .filter_map(|p| {
                    match tile_matrix.get(p){
                        None | Some(Tile::Wall) => None,
                        _ => Some(p)
                    }
                })
                .collect::<Vec<Position<IntType>>>();
            let neighbour_count = neighbours.len();
            // If there are less than 2 non-wall tiles next to this wall, it can't be a shortcut!
            if neighbour_count < 2{
                continue;
            }
            #[cfg(debug_assertions)]
            shortcut_candidates.push(pos);
            let neighbour_matrix: Vec<(usize,usize)> = match neighbour_count{
                2 =>    [ (0,1) ].into(),
                3 =>    [ (0,1),(0,2),(1,2) ].into(),
                4 =>    [ (0,1),(0,2),(0,3),
                          (1,2),(0,3),
                          (2,3) ].into(),
                _ => unreachable!("Pre-filtered already")
            };


            for (a,b) in neighbour_matrix{
                let score_a = distance_matrix.get(neighbours[a]).unwrap().unwrap();
                let score_b = distance_matrix.get(neighbours[b]).unwrap().unwrap();
                let shorcut_value = (score_a-score_b).abs();
                // +1 is added because we need to take a step during the shortcut.
                if shorcut_value >= shortcut_value_threshold+1{
                    shortcut_count += 1;

                    #[cfg(debug_assertions)]
                    {
                        if let Some(log) = shortcut_log.get_mut(&shorcut_value){
                            *log +=1;
                        }else{
                            shortcut_log.insert(shorcut_value, 1);
                        }
                    }
                }
            }
        }
    }
    println!("{shortcut_count} shortcuts >= {shortcut_value_threshold}");

    #[cfg(debug_assertions)]
    {
        println!();
        for pos in shortcut_candidates{
            map[pos.y() as usize][pos.x() as usize] = 'O';
        }
        for row in map.iter(){
            for point in row.iter(){
                print!("{point}");
            }
            println!();
        }
        println!("{shortcut_log:#?}");
    }

}
#[cfg(test)]
mod tests{


    use super::*;
    use test_case::test_case;

}