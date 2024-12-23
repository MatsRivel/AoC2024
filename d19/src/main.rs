use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs::read_to_string};

type Data<'a> = (Vec<&'a str>,Vec<&'a str>);

pub fn get_data(s:&str)->Data{
    let segments = s.lines().filter(|line|  line.contains(',') && !line.is_empty()).flat_map(|s| s.split(", ").collect::<Vec<&str>>()).collect::<Vec<&str>>();
    let targets = s.lines().filter(|line| !line.contains(',') && !line.is_empty()).collect::<Vec<&str>>();
    (segments,targets)
}
fn filter_criteria1(target: &str, segments: &Vec<&str>)->bool{
    if target.is_empty(){
        return true;
    }
    for (idx, &segment) in segments.iter().enumerate(){
        if target == segment{
            return true;
        }
        if target.starts_with(segment){
            // println!("'{target}' started with '{segment}'");
            // let mut new_segments = segments.clone();
            // new_segments.remove(idx);
            let new_target = &target[(segment.len())..];
            // println!("Does '{new_target}' start with any of {new_segments:?}?");
            if filter_criteria1(new_target, &segments){
                return true;
            }
        }
        else{
            // println!();
            // println!("'{target}' DID NOT start with  '{segment}'");   
        }
    }
    // println!("{target} did not start with any of {segments:?}");
    false
}
pub fn solve1(data: &Data)->usize{
    let (segments, targets) = data;
    targets.into_iter().filter(|&&target| {
        if filter_criteria1(target, segments){
            // #[cfg(debug_assertions)]
            // println!("target: {target:?} is valid from {segments:?}");
            true
        }else{
            // #[cfg(debug_assertions)]
            // println!("target: {target:?} is INVALID from {segments:?}");
            false
        }
    }).count()
}

fn filter_criteria2(target: &str, segment_map: &HashMap<char,Vec<&str>>, depth: usize)->usize{
    // TODO: First find how many ways each segment can be written using smaller segments.
    //       Then, if a segment that consists of smaller segments fits, do not use the smaller segments, but multiply the current answer by the number. 
    if target.is_empty(){
        return 1;
    }
    let mut pot = 0;
    if let Some(segments) = segment_map.get(&target.chars().nth(0).unwrap()){
        for &segment in segments.iter(){
            if target == segment{
                pot += 1;
                continue;
            }
            if target.starts_with(segment){
                #[cfg(debug_assertions)]{
                    for _ in 0..depth{
                        print!(".");
                    }
                    println!("{target} starts with {segment}!");
                }
                // let new_segments = segments.clone();
                let new_target = &target[(segment.len())..];
                pot += filter_criteria2(new_target, &segment_map, depth+1);
            }
        }
    }
    pot
}
fn compare_string_length(a: &str, b:&str)->Ordering{
    if a.len() == b.len(){
        Ordering::Equal
    }else if a.len() < b.len(){
        Ordering::Less
    }else{
        Ordering::Greater
    }
}
fn order_segments<'a>(segments:&'a Vec<&str>)->Vec<&'a str>{
    let mut ordered_segments = segments.clone();
    ordered_segments.sort_by(|a,b| compare_string_length(a,b));
    ordered_segments
}
pub fn solve2<'a>(data: &'a Data)->usize{
    let (segments, targets) = data;
    let ordered_segments = order_segments(&segments);
    // println!("{ordered_segments:?}");
    let cleaned_segments = ordered_segments.clone()
        .into_iter()
        .enumerate()
        .filter(|&(idx, target)| !filter_criteria1(target, &ordered_segments[(idx+1)..].to_vec()) )
        .map(|(_,s)| s).collect::<Vec<&str>>();

    // println!("{cleaned_segments:?}\n");
    let segment_map = cleaned_segments.into_iter()
        .map(|seg| (seg.chars().nth(0).unwrap(),seg))
        .fold(HashMap::<char,Vec<&'a str>>::new(), | mut acc,(key,val)|{
            if let Some(vec) = acc.get_mut(&key){
                vec.push(val);
            }else{
                acc.insert(key, vec![val]);
            }
            acc
        } );
        
    targets.into_iter().map(|&target| {
        let count = filter_criteria2(target, &segment_map,0);
        if count > 0{
            // #[cfg(debug_assertions)]
            // println!("{target:?} <-- {segment_map:?}");
        }else{
        }
        count
    }).sum::<usize>()
}

fn main() {
    let start = std::time::Instant::now();
    // let file_name = "Data.txt";
    // // let file_name = "TestData1.txt";
    // let s = read_to_string(file_name).unwrap();
    // let data = get_data(&s);
    // let file_end = std::time::Instant::now();
    // let solution1 = solve1(&data);
    // let s1_end = std::time::Instant::now();
    // assert_eq!(solution1, 228);
    // let solution2 = solve2(&data);
    // // assert_eq!(solution2,284973560658514);
    // let s2_end = std::time::Instant::now();
    // println!("Part1: {solution1}");
    // println!("Part2: {solution2}");
    // println!("Parse file time: {:?}", file_end - start);
    // println!("P1 time: {:?}",s1_end-file_end);
    // println!("P2 time: {:?}", s2_end - s1_end);
    // println!("Total time: {:?}", s2_end - start);

    // let expected = 16;
    let file_name = "TestData2.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let solution = solve2(&data);
    assert!(solution != 0);
    let s2_end = std::time::Instant::now();
    println!("Total time: {:?}", s2_end - start);
}


#[cfg(test)]
mod tests{
    use super::*;

    mod s1{
        use super::*;
        #[test]
        fn getting_expected_data(){
            let expected_segments = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
            let expected_targets = vec![ "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"];

            let file_name = "TestData1.txt";
            let s = read_to_string(file_name).unwrap();
            let data = get_data(&s);
            assert_eq!(data.0,expected_segments);
            assert_eq!(data.1,expected_targets);
        }
        #[test]
        fn solve1_1(){
            let expected = 6;
            let file_name = "TestData1.txt";
            let s = read_to_string(file_name).unwrap();
            let data = get_data(&s);
            let solution = solve1(&data);
            assert_eq!(solution,expected)
        }
        #[test]
        fn solve1_2(){
            let expected = 1;
            let s = "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr".to_string();
            let data = get_data(&s);
            let solution = solve1(&data);
            assert_eq!(solution,expected)
        }
        #[test]
        fn starts_with_test1(){
            let s = "brwrr";
            let subs = &s[1..];
            assert!(subs.starts_with('r'));
        }
        #[test]
        fn starts_with_test2(){
            let s = "brwrr";
            let subs = &s[3..];
            assert!(subs.starts_with('r'));
        }
    }
    mod s2{
        use super::*;
        #[test]
        fn solve2_1(){
            let expected = 16;
            let file_name = "TestData1.txt";
            let s = read_to_string(file_name).unwrap();
            let data = get_data(&s);
            let solution = solve2(&data);
            assert_eq!(solution,expected)
        }
        #[test]
        fn solve2_2(){
            let expected = 2;
            let s = "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr".to_string();
            let data = get_data(&s);
            let solution = solve2(&data);
            assert_eq!(solution,expected)
        }
        #[test]
        fn solve2_3(){
            // let expected = 16;
            let file_name = "TestData2.txt";
            let s = read_to_string(file_name).unwrap();
            let data = get_data(&s);
            let solution = solve2(&data);
            assert!(solution != 0)
        }
    }
}