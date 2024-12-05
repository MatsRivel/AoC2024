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
impl Letter{
    fn vec_from_word(s:&str)->Vec<Self>{
        s.chars().map(|c| c.into()).collect()
    }
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
#[derive(Debug,PartialEq,Eq)]
enum Rotation{
    Zero,
    Eighth,
    Quart,
    ThreeEighth
}

#[derive(Debug)]
struct Matrix{
    data: Vec<Letter>,
    current_rotation: Rotation,
    ymax: usize,
    xmax: usize
}

impl Matrix{
    fn new(s: &str)->Self{
        let unpadded = get_matrix_from_string(s);
        let ymax = unpadded.len();
        let xmax = unpadded[0].len();
        let data = unpadded.into_iter().flat_map(|line| line).collect();
        Self{data, current_rotation: Rotation::Zero, ymax, xmax}
    }
    fn rot_eighth(&mut self){
        match self.current_rotation{
            Rotation::Zero => self.current_rotation = Rotation::Eighth,
            Rotation::Eighth => self.current_rotation = Rotation::Quart,
            Rotation::Quart => self.current_rotation = Rotation::ThreeEighth,
            Rotation::ThreeEighth => self.current_rotation = Rotation::Zero,
        }

    }
    fn point_to_idx(&self, y:usize, x:usize)->usize{
        let idx = (y * (self.xmax)) + x;
        idx
    }
    fn rotational_indices(&self)->Vec<(usize,usize)>{
        let mut indices = Vec::new();
        // for i in (0..self.xmax).rev(){
        //     for j in 0..self.ymax{
        //         let y = j;
        //         let x = i;
        //         indices.push((y,x));
        //     }
        // }
        // let horizontal = 0..self.xmax;
        // let vertical = 1..self.ymax;
        // for i in horizontal{
        //     let x0 = self.xmax-1-i;
        //     let y0 = 0;
        //     let mut x = x0;
        //     let mut y = y0;
        //     indices.push((y,x));
        //     if x > 0 && y < self.ymax-1 {
        //         x -= 1;
        //         y += 1;
        //         indices.push((y,x));
        //     }
        // }
        // for i in vertical{
        //     let x0 = 0;
        //     let y0 = i;
        //     let mut x = x0;
        //     let mut y = y0;
        //     indices.push((y,x));
        //     if x < self.xmax-1 && y < self.ymax-1 {
        //         x += 1;
        //         y += 1;
        //         indices.push((y,x));
        //     }
        // }
        let recursive_indices = recursive_indices(self.xmax-1, 0, self.xmax, self.ymax);
        indices
    }

    fn rotate_self(mut self)->Self{
        let indices = self.rotational_indices();

        let rotated_matrix = indices.iter()
            .filter_map(|&(y,x)| {
                println!("({y},{x})"); //Todo: (2,2) and (0,0) before (1,1)
                let idx = self.point_to_idx(y,x);
                self.data.get(idx)
            }).map(|l| *l)
            .collect();

        self.data = rotated_matrix;
        self.rot_eighth();
        self
        
    }
    fn count_word(&self, target_word:&str)->i32{
        let target = Letter::vec_from_word(target_word);
        let mut y = 0;
        let mut x = target.len();
        let mut count = 0;
        todo!()
    }

    
    fn to_word(&self)->Vec<Letter>{
        self.data.clone()


    }
}

fn recursive_indices(x:usize, y:usize, xmax: usize, ymax:usize)->Option<Vec<(usize,usize)>>{
    if x == xmax || y == ymax{
        return None;
    }
    let values: Vec<(usize,usize)> = [
        Some(vec![(x,y)]),
        recursive_indices(x-1, y, xmax, ymax),
        recursive_indices(x, y+1, xmax, ymax)]
            .into_iter()
            .filter_map(|v| v)
            .flat_map(|v|v)
            .collect();
    if values.is_empty(){
        return None;
    }
    Some(values)
}

fn get_matrix_from_string(s: &str)->Vec<Vec<Letter>>{
    s.lines()
    .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Letter>>())
    .collect::<Vec<Vec<Letter>>>()
}

fn get_padded_matrix(matrix: Vec<Vec<Letter>>)->Vec<Vec<Letter>>{
    let mut horizontal_padded_matrix = matrix
        .into_iter()
        .map(|mut line| {
            let mut padding = vec![' '.into();line.len()];
            line.append(&mut padding);
            line
        }).collect::<Vec<Vec<Letter>>>();
    let mut vertical_padding = vec![vec![' '.into();horizontal_padded_matrix[0].len()];horizontal_padded_matrix.len()];
    horizontal_padded_matrix.append(&mut vertical_padding);
    horizontal_padded_matrix
}

fn get_matrix(file_name: &str)->Vec<Vec<Letter>>{
    let s = read_to_string(file_name).unwrap();
    let matrix = get_matrix_from_string(&s);
    matrix
}

fn solve1(file_name:&str)->i32{
    let s = read_to_string(file_name).unwrap();
    let matrix = Matrix::new(&s);

    todo!()
}

fn main() {
    let s = "XM\nAS".to_string();
    let expected = Letter::vec_from_word("XMAS");
    let matrix = Matrix::new(&s);
    let actual = matrix.to_word();
    assert_eq!(actual, expected)
}

#[cfg(test)]
mod tests{
    use super::*;
    mod points{
        use super::*;
        const POINT_STR: &str = "AAA\nMMM\nSSS";
        #[test]
        fn point_to_idx1(){
            let matrix = Matrix::new(POINT_STR);
            let point = (0,2);
            let expected = 2;
            let actual = matrix.point_to_idx(point.0, point.1);
            assert_eq!(expected,actual);
        }
        #[test]
        fn point_to_idx2(){
            let matrix = Matrix::new(POINT_STR);
            let (y,x) = (1,1);
            let expected = 4;
            let actual = matrix.point_to_idx(y, x);
            assert_eq!(expected,actual);
        }
        #[test]
        fn point_to_idx3(){
            let matrix = Matrix::new(POINT_STR);
            let (y,x) = (1,0);
            let expected = 3;
                    let actual = matrix.point_to_idx(y, x);
            assert_eq!(expected,actual);
        }
        #[test]
        fn point_to_idx4(){
            let matrix = Matrix::new(POINT_STR);
            let (y,x) = (1,2);
            let expected = 5;
            let actual = matrix.point_to_idx(y, x);
            assert_eq!(expected,actual);
        }
        #[test]
        fn point_to_idx5(){
            let matrix = Matrix::new(POINT_STR);
            let (y,x) = (2,2);
            let expected = 8;
                    let actual = matrix.point_to_idx(y, x);
            assert_eq!(expected,actual);
        }
        #[test]
        fn point_to_idx6(){
            let matrix = Matrix::new(POINT_STR);
            let (y,x) = (2,1);
            let expected = 7;
            let actual = matrix.point_to_idx(y, x);
            assert_eq!(expected,actual);
        }
    }
    mod rots{
        const ROT_STR: &str = "XM\nAS";
        use super::*;
        #[test]
        fn rotational_indices1(){
            let matrix = Matrix::new(ROT_STR);
            println!("ymax:{}\nxmax{}",matrix.ymax, matrix.xmax);
            let indices = matrix.rotational_indices();
            let expected = vec![(0,1),(1,1),(0,0),(1,0)];
            assert_eq!(expected,indices);
        }
        const ROT_STR2: &str = "XMA\nAMX";
        use super::*;
        #[test]
        fn rotational_indices2(){
            let matrix = Matrix::new(ROT_STR2);
            println!("ymax:{}\nxmax{}",matrix.ymax, matrix.xmax);
            let indices = matrix.rotational_indices();

            let expected = vec![
                    (0,2),
                (0,1),  (1,2),
            (0,0),  (1,1),  (2,2),
                (1,0), (2,1),
                    (2,0)];

            assert_eq!(expected,indices);
        }

        #[test]
        fn rotation_works(){
            let mut matrix = Matrix::new(ROT_STR);
            let rotations = [Rotation::Zero, Rotation::Eighth, Rotation::Quart, Rotation::ThreeEighth];
            for r in rotations{
                assert_eq!(matrix.current_rotation, r);
                matrix.rot_eighth();
            }
        }

        #[test]
        fn matrix_rot_zero(){
            let matrix = Matrix::new(ROT_STR);
            let expected = Letter::vec_from_word(ROT_STR);
            let actual = matrix.to_word();
            assert_eq!(actual, expected)
    
        }
        
        #[test]
        fn matrix_rot_quart(){
            let matrix = Matrix::new(ROT_STR).rotate_self().rotate_self();
            let expected = Letter::vec_from_word("MSXA");
            let actual = matrix.to_word();
            assert_eq!(actual, expected)
    
        }

        #[test]
        fn matrix_rot_eighth(){
            let matrix = Matrix::new(ROT_STR).rotate_self();
            assert_eq!(matrix.current_rotation,Rotation::Eighth);
            let expected = Letter::vec_from_word("MXSA");
            let actual = matrix.to_word();
            assert_eq!(actual, expected)
    
        }
    }
    
    #[test]
    fn solves_test_1(){
        let file_name = "TestData1.txt";
        let expected = 18;
        let actual = solve1(file_name);
        assert_eq!(expected,actual);
    }
}