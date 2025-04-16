use std::fs::read_to_string;

type Data = (Vec<Key>, Vec<Lock>);
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Hash,Clone, Copy)]
struct Key{
    data: [i32;5]
}
impl Key{
    pub fn new(segments: Vec<&str>)->Self{
        debug_assert!(segments.len() == 7, ">{:?}<",segments);
        debug_assert!(segments.first().unwrap().len() == 5, "{:?}", segments.first().unwrap());
        let mut rows = vec![vec![' ';7];5];
        for (i,s) in segments.iter().enumerate(){
            for (j,c) in s.char_indices(){
                rows[j][i] = c;
            }
        }
        let count = rows.iter().map(|row| row.iter().filter(|point| **point == '#').count() as i32).collect::<Vec<i32>>();
        let mut data = [0i32;5];
        count.into_iter().enumerate().for_each(|(idx,c)| data[idx] = c);
        Self{data}
    }
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Hash,Clone, Copy)]
struct Lock{
    data: [i32;5]
}
impl Lock{
    pub fn new(segments: Vec<&str>)->Self{
        debug_assert!(segments.len() == 7, ">{:?}<",segments);
        debug_assert!(segments.first().unwrap().len() == 5, "{:?}", segments.first().unwrap());
        let mut rows = vec![vec![' ';7];5];
        for (i,s) in segments.iter().enumerate(){
            for (j,c) in s.char_indices(){
                rows[j][i] = c;
            }
        }
        let count = rows.iter().map(|row| row.iter().filter(|point| **point == '#').count() as i32).collect::<Vec<i32>>();
        let mut data = [0i32;5];
        count.into_iter().enumerate().for_each(|(idx,c)| data[idx] = c);
        Self{data}
    }
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord,Hash,Clone, Copy)]
enum KeyOrLock{
    Key(Key),
    Lock(Lock)
}

fn key_fits_in_lock(key: &Key, lock: &Lock)->bool{
    #[cfg(debug_assertions)]
    println!("key: {:?}, lock: {:?}", key.data, lock.data);
    lock.data.iter().zip(key.data.iter()).all(|(l,k)| k+l <= 7)
}

fn main() {
    let start = std::time::Instant::now();
    let file_name = "Data.txt";
    // let file_name = "TestData1.txt";
    let s = read_to_string(file_name).unwrap();
    let data = get_data(&s);
    let file_end = std::time::Instant::now();
    let solution1 = solve1(&data);
    let s1_end = std::time::Instant::now();
    assert!(solution1 < 4291);
    // let solution2 = solve2(&data);
    // assert_eq!(solution2,284973560658514);
    let s2_end = std::time::Instant::now();
    println!("Part1: {solution1}");
    // println!("Part2: {solution2}");
    println!("Parse file time: {:?}", file_end - start);
    println!("P1 time: {:?}",s1_end-file_end);
    println!("P2 time: {:?}", s2_end - s1_end);
    println!("Total time: {:?}", s2_end - start);
}
fn get_data(s: &str) -> Data{
    let segments = s.lines().fold(vec![vec![]], |mut acc, line|{
        let line = line.strip_suffix("\r\n").unwrap_or(line);
        if line.is_empty(){
            acc.push(vec![]);
        }else{
            acc.last_mut().unwrap().push(line);
        }
        acc
    });
    let points = segments.into_iter().map(|segment|{
        if segment.first().unwrap().starts_with("#####"){
            KeyOrLock::Lock(Lock::new(segment))
        }else{
            KeyOrLock::Key(Key::new(segment))
        }
    });
    let data = points.fold((Vec::<Key>::new(),Vec::<Lock>::new()), |(mut key_acc, mut lock_acc),val: KeyOrLock|{
        match val{
            KeyOrLock::Key(key) => key_acc.push(key),
            KeyOrLock::Lock(lock) => lock_acc.push(lock),
        }
        (key_acc,lock_acc)
    });
    data

}

fn solve1(data: &Data) -> i32 {
    let (keys, locks) = data;
    let mut count = 0;
    // println!("key count: {}, locks count: {}", keys.len(), locks.len());
    for key in keys{
        for lock in locks{
            if key_fits_in_lock(key,lock){
                count += 1;
            }
        }
    }
    count
}
fn solve2(data: &Data) -> i32 {
todo!()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        // let file_name = "Data.txt";
        let file_name = "TestData1.txt";
        let expected = 3;
        let s = read_to_string(file_name).unwrap();
        let data = get_data(&s);
        let solution1 = solve1(&data);
        assert_eq!(solution1, expected)
    }
}