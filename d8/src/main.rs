use std::{collections::HashSet, fs::read_to_string};

type Solution = i32;
type PointData = i64;
type Data = Vec<Antenna>;
trait DistanceTo{
    fn distance_to(&self,other: &Self)-> PointData;
    fn vector_to(&self,other: &Self)-> [PointData;2];
}
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PointBuilder{
    ymax:PointData,
    xmax:PointData,
}
impl PointBuilder{
    fn new(ymax:usize, xmax:usize)->Self{
        Self { xmax: xmax as PointData, ymax: ymax as PointData }
    }
    fn new_point(&self, y:PointData, x:PointData)->Option<Point>{
        if 0 <= y && y <= self.ymax && 0 <=x && x <= self.xmax {
            Some(Point { x, y })
        }else{
            None
        }
    }
    fn new_with_vector(&self, point: &Point, [dy,dx]:&[PointData;2])->Option<Point>{
        let y = point.y + dy;
        let x = point.x + dx;
        self.new_point(point.y + dy, point.x+dx)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point{
    y:PointData,
    x:PointData,
}
impl DistanceTo for Point{
    fn distance_to(&self, other:&Self)->PointData{
        (self.y-other.y).abs() + (self.x-other.x).abs()
    }

    fn vector_to(&self,other: &Self)-> [PointData;2] {
        [self.y-other.y, self.x-other.x]
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Frequency{
    None,
    Freq(char)
}
impl Frequency{
    fn new(c:char)->Option<Self>{
        let f: Frequency = c.into();
        match f{
            Frequency::None => None,
            Frequency::Freq(_) => Some(f),
        }
    }
}
impl From<Frequency> for char{
    fn from(value: Frequency) -> Self {
        match value{
            Frequency::None => '.',
            Frequency::Freq(c) => c,
        }
    }
}
impl From<char> for Frequency{
    fn from(value: char) -> Self {
        match value{
            ' ' | '.' => Self::None,
            c=>Self::Freq(c)

        }
    }
}
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Antenna{
    point: Point,
    frequency: Frequency
}
impl Antenna{
    fn new(pos: Option<Point>, frequency: Option<Frequency>)->Option<Self>{
        match (pos,frequency){
            (Some(p), Some(f)) => Some(Self { point: p, frequency:f }),
            _ => None
        }
        
    }
    fn is_same_frequency_as(&self,other:&Self)->bool{
        self.frequency == other.frequency
    }
}
impl DistanceTo for Antenna{
    fn distance_to(&self, other:&Self)->PointData{
        self.point.distance_to(&other.point)
    }

    fn vector_to(&self,other: &Self)-> [PointData;2] {
        self.point.vector_to(&other.point)
    }
}
fn get_data(s:&str)->(Data, PointBuilder){
    let chars: Vec<Vec<char>> = s.lines().map(|line| line.chars().map(|c|c  ).collect()).collect();
    let builder = PointBuilder::new(chars.len()-1, chars[0].len()-1);
    let data: Data = chars.into_iter().enumerate().flat_map(|(y,row)| row.into_iter().enumerate().filter_map(move |(x, c)| {
        let point = builder.new_point(y as PointData ,x as PointData);
        let freqency = Frequency::new(c);
        Antenna::new(point, freqency)
    } )).collect();
    #[cfg(debug_assertions)]
    for d in data.iter(){
        println!("{d:?}");
    }
    (data, builder)
}

fn possible_anti_nodes(antenna_a: &Antenna, antenna_b: &Antenna, builder: &PointBuilder)->[Option<Point>;4]{
    let a = antenna_a.point;
    let b = antenna_b.point;
    let [dy,dx] = a.vector_to(&b);
    // a-b = d
    // a = d+b
    // a - d = b
    let potentials = [
        builder.new_with_vector(&a, &[ dy, dx]),
        builder.new_with_vector(&b, &[-dy,-dx]),
        builder.new_with_vector(&a, &[ -dy/3, -dx/3]),
        builder.new_with_vector(&a, &[ -2*dy/3, -2*dx/3]),
    ];
    #[cfg(debug_assertions)]
    println!("{potentials:?}\n");
    potentials
}
fn possible_anti_nodes2(antenna_a: &Antenna, antenna_b: &Antenna, builder: &PointBuilder)->Vec<Point>{
    let a = antenna_a.point;
    let b = antenna_b.point;
    let [dy,dx] = a.vector_to(&b);
    // a-b = d
    // a = d+b
    // a - d = b
    let potentials = (0..=builder.ymax).flat_map(|y| {
        (0..=builder.xmax).filter(move |x|{
            x%dx == 0 && y%dy == 0
        }).filter_map(move |x| builder.new_point(y, x))
    }).collect();
    // #[cfg(debug_assertions)]
    // println!("{potentials:?}\n");
    potentials
}
fn makes_anti_node(antenna_a: &Antenna, antenna_b: &Antenna, node: &Point )->bool{
    let da = antenna_a.point.vector_to(node);
    let db = antenna_b.point.vector_to(node);
    let c1 = da[0] == db[0]*2 && da[1] == db[1]*2;
    let c2 = db[0] == da[0]*2 && db[1] == da[1]*2;
    let c3 = da != db;
    c3 && (c1 || c2)
}
fn solve1(data: &Data, builder: &PointBuilder)->Solution{
    let mut antis = HashSet::new();
    for (a_idx, a) in data.iter().enumerate(){
        for b in data.iter().skip(a_idx+1){
            if !a.is_same_frequency_as(b){
                continue;
            }
            let possible_anti_nodes = possible_anti_nodes(&a, &b, builder);
            possible_anti_nodes.into_iter()
            .filter_map(|n| n)
                .filter(|node| makes_anti_node(a, b, node))
                .for_each(|node| {
                    // #[cfg(debug_assertions)]
                    // println!("Antinode: {node:?}"); 
                    antis.insert(node);
                });
        }
    }
    #[cfg(debug_assertions)]
    {
        let mut grid = vec![vec!['.';(builder.xmax+1)  as usize];(builder.ymax+1) as usize];
        for anti in antis.iter(){
            grid[anti.y as usize ][anti.x as usize ] = '#';
        }
        for ant in data.iter(){
            grid[ant.point.y as usize ][ant.point.x as usize ] = ant.frequency.into();
        }
        for row in grid{
            for char in row{
                print!("{char}");
            }
            println!()
        }
        println!("___")
    }
    antis.len() as i32
}
fn solve2(data: &Data, builder: &PointBuilder)->Solution{
    let mut antis = HashSet::new();
    for (a_idx, a) in data.iter().enumerate(){
        for b in data.iter().skip(a_idx+1){
            if !a.is_same_frequency_as(b){
                continue;
            }
            let possible_anti_nodes = possible_anti_nodes2(&a, &b, builder);
            possible_anti_nodes.into_iter()
                // .filter(|node| makes_anti_node(a, b, node))
                .for_each(|node| {
                    // #[cfg(debug_assertions)]
                    // println!("Antinode: {node:?}"); 
                    antis.insert(node);
                });
        }
    }
    #[cfg(debug_assertions)]
    {
        let mut grid = vec![vec!['.';(builder.xmax+1)  as usize];(builder.ymax+1) as usize];
        for anti in antis.iter(){
            grid[anti.y as usize ][anti.x as usize ] = '#';
        }
        for ant in data.iter(){
            grid[ant.point.y as usize ][ant.point.x as usize ] = ant.frequency.into();
        }
        for row in grid{
            for char in row{
                print!("{char}");
            }
            println!()
        }
    }
    antis.len() as i32
}

fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let (data, builder) = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data, &builder);
    assert_ne!(solution1,14);
    assert_eq!(solution1,249);
    let s1_end = std::time::Instant::now();
    let solution2 = solve2(&data, &builder);
    // assert_eq!(solution2,0);
    assert!(solution2<2500);
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
    fn solve_test1(){
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let (data, builder) = get_data(&s);
        let answer = solve1(&data, &builder);
        let expected = 14;
        assert_eq!(answer,expected)
    }
    #[test]
    fn solve_test2(){
        let file_name = "TestData1.txt";
        let s = read_to_string(file_name).unwrap();
        let (data, builder) = get_data(&s);
        let answer = solve2(&data, &builder);
        let expected = 34;
        assert_eq!(answer,expected)
    }
    #[test]
    fn can_detect_antis1(){
        let s = "...\n.A.\n..A"; // Only valid spot for an anti is (0,0);
        let (data, builder) = get_data(&s);
        let x = solve1(&data, &builder);
        assert_eq!(x,1);
    }
    #[test]
    fn can_detect_antis2(){
        let s =".......\n...0...\n0......"; // Only valid spot for an anti is (0,6);
        let (data, builder) = get_data(&s);
        let x = solve1(&data, &builder);
        assert_eq!(x,1);
    }

    mod comparisons{
        use super::*;
        #[test]
        fn detect_same_antenna(){
            let a = Antenna::new(Some(Point{y:2,x:2}), Some(Frequency::Freq('a'))).unwrap();
            let b = Antenna::new(Some(Point{y:2,x:2}), Some(Frequency::Freq('a'))).unwrap();
            assert_eq!(a,b)
        }
        
        #[test]
        fn detect_different_antenna(){
            let a = Antenna::new(Some(Point{y:2,x:2}), Some(Frequency::Freq('a'))).unwrap();
            let b = Antenna::new(Some(Point{y:2,x:2}), Some(Frequency::Freq('b'))).unwrap();
            let c = Antenna::new(Some(Point{y:1,x:2}), Some(Frequency::Freq('a'))).unwrap();
            assert_ne!(a,b);
            assert_ne!(a,c);
            assert_ne!(b,c);
        }
        #[test]
        fn detect_same_freq(){
            let a = Frequency::Freq('a');
            let b = Frequency::Freq('a');
            assert_eq!(a,b)
        }
        #[test]
        fn detect_different_freq(){
            let a = Frequency::Freq('a');
            let b = Frequency::Freq('b');
            assert_ne!(a,b)
        }
    }

}