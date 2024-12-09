use std::{collections::VecDeque, fs::read_to_string};
type Solution = usize;
type Data = Vec<BlockType>;
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord,Debug)]
struct Block{
    count: usize,
    id: usize,
}
impl Block{
    fn new(c:char, idx:usize)->Option<Self>{
        let count = c.to_digit(10)? as usize;
        Some(Self{ count,id: idx})
    }
}
#[derive(Clone,Copy,PartialEq, Eq, PartialOrd, Ord,Debug)]
enum BlockType{
    SpaceBlock(Block),
    RealBlock(Block)
}
impl BlockType{
    fn is_space(&self)->bool{
        match self{
            BlockType::SpaceBlock(_) => true,
            BlockType::RealBlock(_) => false,
        }
    }
    fn count(&self)->usize{
        let block = match self{
            BlockType::SpaceBlock(block) => block,
            BlockType::RealBlock(block) => block,
        };
        block.count
    }
}
fn get_data(s:&str)->Data{
    s.lines().flat_map(|line| line.chars().enumerate().filter_map(|(idx,c)|  {
        let is_space = idx%2 == 1;
        if let Some(block) = Block::new(c,idx/2){
            if is_space{
                Some(BlockType::SpaceBlock(block))
            }else{
                Some(BlockType::RealBlock(block))
            }
        }else{
            None
        }
    })).collect()

}

fn solve1(data:&Data)->Solution{
    let mut queue = VecDeque::new();
    for block in data.iter(){
        for _ in 0..block.count(){
            if let BlockType::RealBlock(b) = block{
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
    use std::process::id;

    use super::*;
    pub struct BlockBalancer{
        main: VecDeque<BlockType>,
        left: VecDeque<BlockType>,
        right: VecDeque<BlockType>
    }
    impl BlockBalancer{
        pub fn new(data:&Data)->Self{
            let main = VecDeque::new();
            let left = data.iter().map(|block| *block).collect(); // Block is clonable.
            let right = VecDeque::new();
            Self { main, left, right }
        }
        pub fn move_front_to_main(&mut self){
            if let Some(v) = self.left.pop_front(){
                self.main.push_back(v);
            }
        }
        pub fn move_back_to_main(&mut self){
            if let Some(v) = self.left.pop_back(){
                self.main.push_back(v);
            }
        }
        pub fn shift_left(&mut self)->bool{
            if let Some(v) = self.right.pop_front(){
                    self.right.push_back(v); 
                    true
            }else{
                false
            }

        }
        pub fn shift_right(&mut self)->bool{
            if let Some(v) = self.left.pop_back(){
                self.right.push_front(v);
                true
            }else{
                false
            }
        }
        pub fn peek_left(&self)->Option<&BlockType>{
            self.left.get(0)
        }
        pub fn peek_right(&self)->Option<&BlockType>{
            if self.left.len() == 0{
                None
            }else{
                self.left.get(self.left.len()-1)
            }
            
        }
        pub fn reset_buffers(&mut self){
            while self.shift_left(){};
        }
        pub fn get_score(self)->Solution{
            self.main.iter().filter_map(|v|{
                match v{
                    BlockType::SpaceBlock(block) => None,
                    BlockType::RealBlock(block) => Some(block.id),
                }
            }).enumerate().map(|(idx,id)| {idx*id}).sum::<Solution>()
        }
        fn string_from_queue(&self, q: &VecDeque<BlockType>)->String{
            q.iter().map(|block| {
                (0..block.count()).map(|_|{
                    match block{
                        BlockType::SpaceBlock(_block) => ".".to_string(),
                        BlockType::RealBlock(block) => block.id.to_string(),
                    }
                }).collect::<String>()
            }).collect::<String>()
        }
        pub fn left_as_string(&self)->String{
            self.string_from_queue(&self.left)
        }
        pub fn main_as_string(&self)->String{
            self.string_from_queue(&self.main)
        }
        pub fn clean_first_n_from_left(&mut self, n: usize){
            for _ in 0..n{
                self.left.pop_front();
            }
        }
    }

}
fn solve2(data:&Data)->Solution{
    let mut balancer = block_balancer::BlockBalancer::new(data);
    println!("{}",balancer.left_as_string());
    while let Some(peeked) = balancer.peek_left(){
        match peeked{
            BlockType::RealBlock(_) => balancer.move_front_to_main(),
            BlockType::SpaceBlock(_) => {
                let space_size = peeked.count();
                let mut found_fit = false;
                while let Some(candidate) = balancer.peek_right(){
                    let count = candidate.count();
                    if count <= space_size{
                        for _ in 0..space_size{
                            balancer.move_back_to_main();
                        }
                        balancer.clean_first_n_from_left(count);
                        found_fit = true;
                    }else{
                        for _ in 0..candidate.count(){
                            balancer.shift_right();
                        }
                    }
                }
                if !found_fit{
                    // No more available data in the left buffer.
                    // Time to accept the empty spaces.
                     // Moving all the stuff back, then moving the front to main.
                    balancer.reset_buffers(); //Note: Doing this here, and at the end of the loop, has no additional cost, as at the end of the loop the queue will be empty.
                    for _ in 0..space_size{
                        balancer.move_front_to_main();
                    }
                }
            }
        }

        balancer.reset_buffers();
    }
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
    fn test2_parsing(){
        let file_name = "TestData1.txt";
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let balancer = block_balancer::BlockBalancer::new(&data);
        assert_eq!(&balancer.left_as_string(), expected);
    }
}