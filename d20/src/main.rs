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

pub fn shortcut_counting(a:usize,b:usize, distance_matrix: &Matrix<Option<i32>>, neighbours: &Vec<Position<IntType>>, savings_threshold:i32 )->i32{

    let score_a = distance_matrix.get(neighbours[a]).unwrap().unwrap();
    let score_b = distance_matrix.get(neighbours[b]).unwrap().unwrap();
    let shorcut_value = (score_a-score_b).abs();
    // +1 is added because we need to take a step during the shortcut.
    if shorcut_value >= savings_threshold+1{
        return 1;
    }else{
        return 0;
    }
}


pub fn get_distance_matrix(tile_matrix:&Matrix<Tile>)->Matrix<Option<i32>>{
    let mut distance_matrix = Matrix::from(tile_matrix);
    let mut candidates: HashSet<Position<IntType>> = HashSet::new();
    candidates.insert(tile_matrix.get_end());
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
    return distance_matrix;
}

pub fn get_shortcuts(tile_matrix:Matrix<Tile>,savings_threshold:i32, max_shortcut_length:i32)->i32{
    let distance_matrix = get_distance_matrix(&tile_matrix);

    #[cfg(debug_assertions)]
    let mut shortcut_candidates = Vec::new();
    #[cfg(debug_assertions)]
    let mut shortcut_log: HashMap<i32, i32> = HashMap::new();
    #[cfg(debug_assertions)]
    let mut map = tile_matrix.get_map();
    #[cfg(not(debug_assertions))]
    let map = tile_matrix.get_map();

    let mut shortcut_count = 0;
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
                shortcut_count += shortcut_counting(a, b, &distance_matrix, &neighbours, savings_threshold)
            }
        }
    }
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
    return shortcut_count;
}

fn main() {

    #[cfg(debug_assertions)]
    let path = "TestData.txt";
    // let path = "HomeBrew3.txt";
    #[cfg(not(debug_assertions))]
    let path = "Data.txt";

    let tile_matrix = Matrix::new(path);
    let max_shortcut_length = 2;
    #[cfg(not(debug_assertions))]
    let shortcut_value_threshold = 100;
    
    #[cfg(debug_assertions)]
    let shortcut_value_threshold = 1;
    let shortcut_count = get_shortcuts(tile_matrix,shortcut_value_threshold,max_shortcut_length);
    println!("{shortcut_count} shortcuts >= {shortcut_value_threshold}");

}
#[cfg(test)]
mod tests{
    use super::*;
    use test_case::test_case;
    #[test_case("TestData.txt",1, 77)]
    #[test_case("Data.txt",100, 1375)]
    fn solves_p1(path:&str, shortcut_value_threshold: i32, expected:i32){
        let tile_matrix = Matrix::new(path);
        let shortcut_count = get_shortcuts(tile_matrix,shortcut_value_threshold, 2);
        assert_eq!(shortcut_count,expected)
    }

}