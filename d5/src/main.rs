use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};


fn parse_file(file_name: &str)->(HashMap<usize,Vec<usize>>,Vec<Vec<usize>>){
    let s = read_to_string(file_name).unwrap();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates = Vec::new();
    s.lines().for_each(|line|{
        if let Some((left,right)) = line.split_once('|'){
            let a = left.parse().unwrap();
            let b = right.parse().unwrap();
            if let Some(less_than_list) = rules.get_mut(&a){
                less_than_list.push(b);
            }else{
                rules.insert(a, vec![b]);
            }
        }
        else{
            let update = line.split(',').filter_map(|num|{
                let num = match num.strip_suffix("\r"){
                    Some(v) => v,
                    None => num,
                };
                num.parse().ok()
            }).collect::<Vec<usize>>();
            updates.push(update);
        }
    });
    (rules,updates)
}
fn is_line_valid(line: &Vec<usize>, rules: &HashMap<usize,Vec<usize>>)->bool{
    let mut seen_numbers = vec![false;100];
    line.iter().all(|point|{
        let mut is_valid = true;
        if let Some(criteria) = rules.get(point){
            for element in criteria{
                if seen_numbers[*element]{
                    is_valid = false;
                    break;
                }
            }
        }
        seen_numbers[*point] = true; // Setting this last as two or more consecutive of the same value is ok.
        is_valid
    })
}
fn solve1(rules: &HashMap<usize,Vec<usize>>, updates: &Vec<Vec<usize>>)->usize{
    updates.into_iter()
        .filter(|line| !line.is_empty())
        .filter(|line| is_line_valid(line, &rules))
        .map(|line|{
            let idx = line.len()/2;
            line[idx]
        }).sum::<usize>()
}
#[derive(PartialEq)]
struct Page<'a>{
    num:usize,
    rules: &'a HashMap<usize,Vec<usize>>
}
impl <'a>Page<'a>{
    fn new(num: usize, rules: &'a HashMap<usize,Vec<usize>>)->Self{
        Self{num,rules}
    }
}
impl <'a>PartialOrd for Page<'a>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(criteria) = self.rules.get(&self.num){
            if criteria.contains(&other.num){
                return Some(std::cmp::Ordering::Less);
            }
        }
        if let Some(criteria) = self.rules.get(&other.num){
            if criteria.contains(&self.num){
                return Some(std::cmp::Ordering::Greater);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}
impl <'a>Eq for Page<'a>{}
impl <'a>Ord for Page<'a>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ord) = self.partial_cmp(other){
            ord
        }else{
            Ordering::Equal
        }
    }
}

fn solve2(rules: HashMap<usize,Vec<usize>>, updates: Vec<Vec<usize>>)->usize{
    updates.into_iter()
        .filter(|line| !is_line_valid(line,&rules))
        .map(|line|{
            line.into_iter().map(|point| Page::new(point, &rules)).collect::<Vec<Page>>()           
        }).map(|mut line|{
            line.sort();
            line
        }).map(|line|{
            let idx = line.len()/2;
            line[idx].num
        }).sum::<usize>()
}
fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    let (rules, updates) = parse_file(file_name);
    let solution1 = solve1(&rules, &updates);
    assert_eq!(solution1,4905);
    let solution2 = solve2(rules, updates);
    let end = std::time::Instant::now();
    assert_eq!(solution2,6204);
    println!("Part1: {solution1}");
    println!("Part2: {solution2}");
    println!("Total time: {:?}",end-start);
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let file_name = "TestData1.txt";
        let (rules, updates) = parse_file(file_name);
        let solution1 = solve1(&rules, &updates);
        assert_eq!(solution1,143);
    }
    #[test]
    fn test2(){
        let file_name = "TestData1.txt";
        let (rules, updates) = parse_file(file_name);
        let solution1 = solve2(rules, updates);
        assert_eq!(solution1,123);
    }
}