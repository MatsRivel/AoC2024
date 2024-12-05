use std::{fmt::Display, fs::read_to_string};
impl From<Letter> for char{
    fn from(value: Letter) -> Self {
        value.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Letter{
    X,
    M,
    A,
    S,
    None
}

impl From<char> for Letter{
    fn from(value: char) -> Self {
        match value{
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S, 
            _ =>Self::None 
        }
    }
}
impl Display for Letter{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Letter::X => write!(f,"X"),
            Letter::M => write!(f,"M"),
            Letter::A => write!(f,"A"),
            Letter::S => write!(f,"S"),
            Letter::None => write!(f,"_"),
        }
    }
}

fn get_matrix_from_string(s: &str)->Vec<Vec<Letter>>{
    s.lines()
    .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Letter>>())
    .collect::<Vec<Vec<Letter>>>()
}



fn find_horizontals(matrix:&Vec<Vec<Letter>>)->i32{
    let mut counter = 0;
    use Letter::*;
    let target = [X,M,A,S]; 
    let reverse_target = [S,A,M,X];
    for i in 0..matrix.len(){
        for j in 0..=(matrix.len()-target.len()){
            let actual = [matrix[i][j],matrix[i][j+1],matrix[i][j+2],matrix[i][j+3]];
            // println!("{actual:?}");
            if actual == target || actual == reverse_target{
                counter += 1;
            }
        }
    }    
    counter
}
fn find_vertical(matrix:&Vec<Vec<Letter>>)->i32{
    let mut counter = 0;
    use Letter::*;
    let target = [X,M,A,S]; 
    let reverse_target = [S,A,M,X];
    for i in 0..=(matrix.len()-target.len()){
        for j in 0..matrix.len(){
            let actual = [matrix[i][j],matrix[i+1][j],matrix[i+2][j],matrix[i+3][j]];
            if actual == target || actual == reverse_target{
                counter += 1;
            }
        }
    }    
    counter
}
fn find_diagonal_left(matrix:&Vec<Vec<Letter>>)->i32{
    let mut counter = 0;
    use Letter::*;
    let target = [X,M,A,S]; 
    let reverse_target = [S,A,M,X];
    // println!("{target:?} <---> {reverse_target:?}\n");
    for i in 0..=(matrix.len()-target.len()){
        for j in 0..=(matrix.len()-target.len()){
            let actual = [matrix[i][j],matrix[i+1][j+1],matrix[i+2][j+2],matrix[i+3][j+3]];
            // println!("{actual:?}");
            if actual == target || actual == reverse_target{
                counter += 1;
            }
        }
    }    
    counter
}
fn find_diagonal_right(matrix:&Vec<Vec<Letter>>)->i32{
    let mut counter = 0;
    use Letter::*;
    let target = [X,M,A,S]; 
    let reverse_target = [S,A,M,X];
    for i in 0..=(matrix.len()-target.len()){
        for j in ((target.len()-1)..matrix.len()).rev(){
            let actual = [matrix[i][j],matrix[i+1][j-1],matrix[i+2][j-2],matrix[i+3][j-3]];
            if actual == target || actual == reverse_target{
                counter += 1;
            }
        }
    }    
    counter
}
fn solve1(matrix:&Vec<Vec<Letter>>)->i32{
    let horizontal = find_horizontals(matrix);
    let vertical = find_vertical(matrix);
    let diagonal_left = find_diagonal_left(matrix);
    let diagonal_right = find_diagonal_right(matrix);
    horizontal + vertical + diagonal_left + diagonal_right
}
fn solve2(matrix:&Vec<Vec<Letter>>)->i32{
    use Letter::*;
    let y_range = 1..(matrix.len()-1);
    let x_range = 1..(matrix[0].len()-1);
    let a_list = y_range.flat_map(|y| x_range.clone().filter_map(move |x| {
        match matrix[y][x]{
            Letter::A => Some((y,x)),
            _ => Option::None
        }
    }));
    let mut counter = 0;
    for (y,x) in a_list{
        let words = [[
            M,  S,
              A,
            M,  S],[
            S,  S,
              A,
            M,  M],[
            M,  M,
              A,
            S,  S],[
            S,  M,
              A,
            S,  M],
            ];
        let found = [
            matrix[y-1][x-1],
            matrix[y-1][x+1],
            matrix[y][x],
            matrix[y+1][x-1],
            matrix[y+1][x+1]
            ];
        if words.contains(&found){
            counter += 1;
        }
    }
    counter
}

fn main() {
    let start = std::time::Instant::now();
    let s = read_to_string("Data.txt").unwrap();
    let matrix = get_matrix_from_string(&s);
    let solution1 = solve1(&matrix);
    assert_eq!(solution1,2639);
    let solution2 = solve2(&matrix);
    assert_eq!(solution2,2005);
    let end = std::time::Instant::now();
    println!("Part 1: {solution1}");
    println!("Part 2: {solution2}");
    println!("Total time: {:?}",end-start);
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn diagonal_left_test(){
        let expected = 1;
        let s = "XXXX\nMMMM\nAAAA\nSSSS";
        let matrix = get_matrix_from_string(s);
        println!("--- Test Input ---");
        for row in matrix.iter(){
            println!("{row:?}");
        }
        println!("------------------");
        let actual = find_diagonal_left(&matrix);
        assert_eq!(expected,actual,"Found {actual}, expected {expected}");
    }
    #[test]
    fn diagonal_right_test(){
        let expected = 1;
        let s = "XXXX\nMMMM\nAAAA\nSSSS";
        let matrix = get_matrix_from_string(s);
        println!("--- Test Input ---");
        for row in matrix.iter(){
            println!("{row:?}");
        }
        println!("------------------");
        let actual = find_diagonal_right(&matrix);
        assert_eq!(expected,actual,"Found {actual}, expected {expected}");
    }
    #[test]
    fn vertical_test(){
        let expected = 3;
        let s = "XXXX\nMMMM\nAAAA\nSXSS";
        let matrix = get_matrix_from_string(s);
        println!("--- Test Input ---");
        for row in matrix.iter(){
            println!("{row:?}");
        }
        println!("------------------");
        let actual = find_vertical(&matrix);
        assert_eq!(expected,actual,"Found {actual}, expected {expected}");
    }
    #[test]
    fn horizontal_test(){
        let expected = 2;
        let s = "XMAS\nXMAS\nAAAA\nSSSS";
        let matrix = get_matrix_from_string(s);
        println!("--- Test Input ---");
        for row in matrix.iter(){
            println!("{row:?}");
        }
        println!("------------------");
        let actual = find_horizontals(&matrix);
        assert_eq!(expected,actual,"Found {actual}, expected {expected}");
    }
    #[test]
    fn solves_test_1(){
        let file_name = "TestData1.txt";
        let expected = 18;
        let s = read_to_string(file_name).unwrap();
        let matrix = get_matrix_from_string(&s);
        let actual = solve1(&matrix);
        assert_eq!(expected,actual);
    }
    #[test]
    fn solves_test_2(){
        let file_name = "TestData1.txt";
        let expected = 9;
        let s = read_to_string(file_name).unwrap();
        let matrix = get_matrix_from_string(&s);
        let actual = solve2(&matrix);
        assert_eq!(expected,actual);
    }
}