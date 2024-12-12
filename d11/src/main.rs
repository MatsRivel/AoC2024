use std::collections::HashMap;

use stone::Stone;


const DATA:&str = "92 0 286041 8034 34394 795 8 2051489";


/*
Rules:
- if id == 0 => id = 1
- else if id.len() %2 == 0 => (id[0..id.len()/2],id[id.len()/2..id.len()]) 
- else: => id*2024
*/

mod stone{
    use std::fmt::Display;
    use super::*;
    pub type IdNum = u64;
    #[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
    pub struct Stone{
        id: IdNum
    }
    impl Display for Stone{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let n = self.get_id();
            write!(f,"{n}")
        }
    }
    impl Stone{
        pub fn new(id:IdNum)->Self{
                Self{id}
        }

        pub fn get_id(&self)->IdNum{
            self.id
        }
        pub fn rule1(&self)->bool{
            self.get_id() == 0
        }
        fn digit_count(&self)->IdNum{
            (self.id as f64).log10() as IdNum +1
        }
        pub fn rule2(&self)->bool{
            self.digit_count()%2 == 0
        }
        fn split_num(&self)->[IdNum;2]{
            let pow = (10 as IdNum).pow(self.digit_count() as u32/2);
            let left = self.id / pow;
            let left_big = left * pow;
            let right = self.id-left_big;
            [left,right]
        }

        pub fn apply_rules(self)->Vec<Self>{
            if self.rule1(){
                vec![Self::new(1)]
            }else if self.rule2(){
                let [left,right] = self.split_num();
                vec![Self::new(left),Self::new(right)]
            }
            else{
                let id = self.get_id() * 2024;
                vec![Self::new(id)]
            }
        }
    }

}
use stone::IdNum;
fn get_data(s:&str)->Vec<Stone>{
    s.split(' ').map(|number| {
        let id = number.parse::<IdNum>().unwrap();
        Stone::new(id)
    } ).collect()
}


fn solve1(data: Vec<Stone>, cache: &mut Cache)->CacheNum{
    // let output = solve(data, 25);
    let mut count = 0;
    for stone in data.into_iter(){
        count += single_stone_custom_cache(stone, 75-25, 75, cache);
    }
    count

}
type CacheNum = u64;
struct Cache{
    cache: HashMap::<IdNum,[Option<CacheNum>;75]>
}
impl Cache{
    fn new()->Self{
        let cache = HashMap::new();
        Self{cache}
    }
    fn get(&self,key:&IdNum, iteration: &usize)->Option<CacheNum>{
        if let Some(arr) = self.cache.get(key){
            arr[*iteration]
        }else{
            None
        }
    }
    fn insert(&mut self, key:&IdNum, iteration: &usize, value: CacheNum)->Option<CacheNum>{
        if let Some(arr) = self.cache.get_mut(key){
            if let Some(previously_existing) = arr[*iteration]{
                Some(previously_existing)
            }else{
                arr[*iteration] = Some(value);
                None
            }
        }else{
            let mut new_arr = [None;75];
            new_arr[*iteration] = Some(value);
            self.cache.insert(*key, new_arr);
            None
        }
    }
}
fn single_stone_custom_cache(stone: Stone, current: usize, n: usize, cache: &mut Cache)->CacheNum{
    if current == n{
        return 1;
    }
    let id = stone.get_id();
    match cache.get(&id,&current){
        Some(count) => count,
        None => {
            let stones = stone.apply_rules();
            let mut count = 0;
            for inner_stone in stones.into_iter(){
                let inner_count = single_stone_custom_cache(inner_stone, current+1, n, cache);
                count += inner_count;
            }
            cache.insert(&id, &current, count);
            count
        },
    }
}
fn solve2(data: Vec<Stone>, mut cache: Cache)->CacheNum{
    // Note: assuming the data is the same.
    // Done!
    let mut count = 0;
    for stone in data.into_iter(){
        count += single_stone_custom_cache(stone, 0, 75, &mut cache);
    }
    count
    

}
fn main() {
    let start = std::time::Instant::now();
    let s = DATA;
    let data = get_data(s);
    let data2 = data.clone();
    let file_end = std::time::Instant::now();
    let mut cache = Cache::new();
    let solution1 = solve1(data, &mut cache);
    let s1_end = std::time::Instant::now();
    assert_eq!(solution1, 239714);
    let solution2 = solve2(data2, cache);
    assert_eq!(solution2,284973560658514);
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
    const TESTDATA1: &str = "0 1 10 99 999";
    const TESTDATA2: &str = "125 17";
    use super::*;
    #[test]
    fn solve_test1_2(){
        let s = TESTDATA2;
        let expected = 55312;
        let data = get_data(&s);
        let mut cache = Cache::new();
        let solution1 = solve1(data,&mut cache);
        assert_eq!(solution1,expected)
    }

    // mod stone_rules{
    //     use super::*;
    //     #[test]
    //     fn rule0(){
    //         let expected = Stone::new(vec![0]);
    //         let actual = Stone::new(vec![0,0,0,0]);
    //         assert_eq!(actual,expected)
    //     }
    //     #[test]
    //     fn rule1(){
    //         let expected = vec![Stone::new(vec![1])];
    //         let actual = Stone::new(vec![0]).apply_rules();
    //         assert_eq!(actual,expected)
    //     }
    //     #[test]
    //     fn rule2(){
    //         let expected = vec![Stone::new(vec![1]), Stone::new(vec![1])];
    //         let actual = Stone::new(vec![1,1]).apply_rules();
    //         assert_eq!(actual,expected)
    //     }
    //     #[test]
    //     fn rule3(){
    //         let expected = vec![Stone::new(vec![2,0,2,4])];
    //         let actual = Stone::new(vec![1]).apply_rules();
    //         assert_eq!(actual,expected)
    //     }
    // }
}