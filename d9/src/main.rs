use std::{collections::VecDeque, fs::read_to_string};

use block_balancer::BlockBalancer;
type Solution = usize;
type Data = Vec<BlockType>;
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord,Debug)]
struct Point{
    count: usize,
    id: usize,
}
impl Point{
    fn new(c:char, idx:usize)->Option<Self>{
        let count = c.to_digit(10)? as usize;
        Some(Self{ count,id: idx})
    }
}

#[derive(Clone,PartialEq, Eq, PartialOrd, Ord,Debug)]
enum PointType{
    Space(Point),
    Real(Point)
}
impl PointType{
    fn is_space(&self)->bool{
        match self{
            Self::Space(_) => true,
            Self::Real(_) => false,
        }
    }
    fn count(&self)->usize{
        let point = match self{
            Self::Space(point) => point,
            Self::Real(point) => point,
        };
        point.count
    }
}
#[derive(Clone,PartialEq, Eq, PartialOrd, Ord,Debug)]
struct Block{
    id:usize,
    data: Vec<Point>
}
impl Block{
    fn new(id: usize, data: Vec<Point>)->Self{
        Self{id, data}
    }
    fn len(&self)->usize{
        self.data.len()
    }
}
#[derive(Clone,PartialEq, Eq, PartialOrd, Ord,Debug)]
enum BlockType{
    Space(Block),
    Real(Block)
}
impl BlockType{
    fn new(data: Vec<PointType>)->Self{
        match data.first().unwrap(){
            PointType::Space(point) => {
                let points = data.iter().map(|v| {
                    match v{
                        PointType::Space(point) => *point,
                        PointType::Real(point) => *point,
                    }
                }).collect();
                Self::Space(Block::new(point.id, points))
            },
            PointType::Real(point) => {
                let points = data.iter().map(|v| {
                    match v{
                        PointType::Space(point) => *point,
                        PointType::Real(point) => *point,
                    }
                }).collect();
                Self::Real(Block::new(point.id, points))
            },
        }
    }
    fn len(&self)->usize{
        match self{
            BlockType::Space(block) => block.data.len(),
            BlockType::Real(block) => block.data.len(),
        }
    }
}

fn get_data(s:&str)->Vec<PointType>{
    s.lines().flat_map(|line| line.chars().enumerate().filter_map(|(idx,c)|  {
        let is_space = idx%2 == 1;
        if let Some(block) = Point::new(c,idx/2){
            if is_space{
                Some(PointType::Space(block))
            }else{
                Some(PointType::Real(block))
            }
        }else{
            None
        }
    })).collect()

}

fn solve1(data:&Vec<PointType>)->Solution{
    let mut queue = VecDeque::new();
    for point in data.iter(){
        for _ in 0..point.count(){
            if let PointType::Real(b) = point{
                queue.push_back(Some(b.id));
            }else{
                queue.push_back(None);
            }
        }
    }
    let mut group = VecDeque::new();
    'outer: while let Some(element) = queue.pop_front(){
        if let Some(point) = element{
            #[cfg(debug_assertions)]
            print!("{point}");
            group.push_back(point);
        }else{
            'inner: loop{
                match queue.pop_back(){
                    None => break 'outer,
                    Some(None) => continue,
                    Some(Some(point)) =>  {
                        #[cfg(debug_assertions)]
                        print!("{point}");
                        group.push_back(point);
                        break 'inner;
                    }
                }
            }
        }
    }
    let output = group.iter().enumerate().map(|(idx,v)| idx*v).sum::<Solution>();
    output
}

mod block_balancer{
    use super::*;
    pub struct BlockBalancer{
        main: VecDeque<PointType>,
        left: VecDeque<PointType>,
        right: VecDeque<BlockType>
    }
    impl BlockBalancer{
        pub fn new(data:&Vec<PointType>)->Self{
            let main = VecDeque::new();
            let left = data.iter().map(|point| block.clone() ).collect(); // Block is clonable.
            let right = VecDeque::new();
            Self { main, left, right }
        }
        pub fn peek_left(&self)->Option<&BlockType>{
            self.left.get(0)
        }
        pub fn left_to_main(&mut self){
            self.main.push_back(self.left.pop_front().unwrap());
        }
        pub fn find_fitting_block(&mut self, size: usize)->Option<BlockType>{
            let mut output = None;
            while let Some(block_type) = self.left.pop_back(){
                match block_type{
                    BlockType::Space(_) => self.right.push_front(block_type),
                    BlockType::Real(block) => {
                        let block_len = block.len();
                        if block_len <= size{
                            output = Some(BlockType::Real(block));
                            break;
                        }else{
                            self.right.push_front(BlockType::Real(block));
                        }
                    }
                }
            }
            self.reset_right(); // Put all points from right back on to left.
            output
        }
        pub fn main_push_back(&mut self, value: BlockType){
            self.main.push_back(value);
        }
        pub fn get_score(self)->usize{
            self.main.iter().flat_map(|block|{
                match block{
                    BlockType::Space(block) => block.data.clone(),
                    BlockType::Real(block) => block.data.clone(),
                }
            }).enumerate().map(|(idx, point)|{
                idx * point.id
            }).sum::<usize>()
        }
        pub fn left_len(&self)->usize{
            self.left.len()
        }
        pub fn reset_right(&mut self){
            while let Some(v) = self.right.pop_front(){
                self.left.push_back(v);
            }
        }
        pub fn main_as_string(&self)->String{
            blocks_to_string(self.main.iter())
        }

    }

}
fn blocks_to_string<'a,I>(blocks: I)->String
where
    I: IntoIterator<Item = &'a BlockType>,
{
    blocks.into_iter().map(|b| {
        match b{
            BlockType::Space(block) => block.data.iter().map(|_| '.'.to_string()).collect::<String>(),
            BlockType::Real(block) => block.data.iter().map(|p| format!("{}",p.id)).collect::<String>(),
        }
    }).collect::<String>()
}
fn points_to_blocks(data:&Vec<PointType>)->Vec<BlockType>{
    data.iter().fold(vec![vec![]], |mut acc: Vec<Vec<PointType>>,v|{
        let point = v.clone();
        let acc_len = acc.len();
        let current = acc.get_mut(acc_len-1).unwrap();
        if current.is_empty(){
            current.push(point);
        }else{
            if current.last().unwrap() == v{
                current.push(point);
            }else{
                acc.push(vec![point]);
            }
        }
        acc
    }).into_iter().map(|vec|{
        BlockType::new(vec)
    }).collect()
}
fn solve2(point_data:&Vec<PointType>)->Solution{
    let data = points_to_blocks(point_data);
    let mut balancer = BlockBalancer::new(&data);
    while let Some(peek_type) = balancer.peek_left(){
        match peek_type{
            BlockType::Real(_) => balancer.left_to_main(),
            BlockType::Space(block) => {
                let size = block.len();
                if let Some(fit) = balancer.find_fitting_block(size){
                    balancer.main_push_back(fit);
                }else{
                    balancer.left_to_main();
                }
            },
        }
    }
    #[cfg(debug_assertions)]
    println!("{}",balancer.main_as_string());
    balancer.get_score()
}

fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data);
    assert_eq!(solution1,6353658451014);
    let s1_end = std::time::Instant::now();
    // let solution2 = solve2(&data);
    // assert_eq!(solution2,0);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    // println!("Part2: {solution2}");
    println!("Parse file time: {:?}",file_end-start);
    println!("P1 time: {:?}",s1_end-file_end);
    // println!("P2 time: {:?}",s2_end-s1_end);
    println!("Total time: {:?}",s2_end-start);
}   

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn solve_test1(){
        let file_name = "TestData1.txt";
        let expected = 1928;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve1(&data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn solve_test2(){
        let file_name = "TestData1.txt";
        let expected = 2858;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve2(&data);
        assert_eq!(solution1,expected)
    }
    #[test]
    fn peek_left_should_not_consume(){
        let s = "1";
        let data = points_to_blocks(&get_data(&s));
        let mut balancer = BlockBalancer::new(&data);
        assert_eq!(balancer.left_len(), 1);
        let _ = balancer.peek_left();
        assert_eq!(balancer.left_len(), 1);
        balancer.left_to_main();
        assert_eq!(balancer.left_len(), 0);
    }
    #[test]
    fn main_as_string_test_short(){
        let s = "11";
        let data = points_to_blocks(&get_data(&s));
        let mut balancer = BlockBalancer::new(&data);
        assert_eq!(balancer.left_len(),2);
        balancer.left_to_main();
        assert_eq!(balancer.left_len(),1);
        balancer.left_to_main();
        assert_eq!(balancer.left_len(),0);
        let actual = balancer.main_as_string();
        let expected = "0.".to_string();
        assert_eq!(actual,expected)
    }
    #[test]
    fn main_as_string_test(){
        let s = "32";
        let data = points_to_blocks(&get_data(&s));
        let mut balancer = BlockBalancer::new(&data);
        assert_eq!(balancer.left_len(),2);
        balancer.left_to_main();
        assert_eq!(balancer.left_len(),1);
        balancer.left_to_main();
        assert_eq!(balancer.left_len(),0);
        let actual = balancer.main_as_string();
        let expected = "000..".to_string();
        assert_eq!(actual,expected)
    }

    #[test]
    fn points_to_blocks_test(){
        let points = vec![
            PointType::Real(Point { count: 2, id: 0 }),
            PointType::Real(Point { count: 2, id: 0 }),
            PointType::Space(Point { count: 2, id: 1 }),
            PointType::Space(Point { count: 2, id: 1 }),
            PointType::Real(Point { count: 3, id: 2 }),
            PointType::Real(Point { count: 3, id: 2 }),
            PointType::Real(Point { count: 3, id: 2 })
        ];
        let real_points = get_data("223");
        assert_eq!(real_points,points);
        let expected = points.iter().map(|p|{
            match p{
                PointType::Space(_) => ".".to_string(),
                PointType::Real(point) => format!("{}",point.id),
            }
        }).collect::<String>();
        let blocks = points_to_blocks(&points);
        let actual = blocks_to_string(blocks.iter());
        assert_eq!(actual,expected)
    }

}