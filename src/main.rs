use itertools::Itertools::{self};
use starter::Starter;
mod latin_square;
mod starter;
use std::collections::HashMap;

mod speedy_strong;

#[derive(Debug)]
enum STSError {
    RangeError(String),
    CongruenceError(String),
}

#[derive(Clone)]
struct Stiner {
    n: i32,
    blocks: HashMap<(i32, i32, i32), i8>,
    pairs: Vec<(i32, i32)>,
    used_pairs: HashMap<(i32, i32), i8>,
    all_possible_blocks: Vec<(i32, i32, i32)>,
}
impl Stiner {
    fn new(n: i32) -> Result<Self, STSError> {
        if n < 3 {
            return Err(STSError::RangeError("N is out of range".to_string()));
        } else if n % 6 != 1 && n % 6 != 3 {
            return Err(STSError::CongruenceError(
                "n must be congruent to 1 or 3 (mod 6)".to_string(),
            ));
        }
        let pairs = (0..n).combinations(2).collect_vec();
        let blocks = HashMap::new();

        let p = pairs.into_iter().map(|vector| (vector[0], vector[1]));
        let mut pairs = vec![];
        for v in p {
            pairs.push(v);
        }

        let used_pairs = HashMap::new();
        //let all_possible_blocks = Stiner::get_all_possible_blocks(n);
        let all_possible_blocks = Stiner::get_all_possible_blocks(n);

        Ok(Self {
            n,
            blocks,
            pairs,
            used_pairs,
            all_possible_blocks,
        })
    }
    fn get_pairs(triple: &(i32, i32, i32)) -> Vec<(i32, i32)> {
        let triple = triple.clone();
        let (a, b, c) = triple;
        vec![(a, b), (a, c), (b, c)]
    }
    fn can_insert(&self, triple: (i32, i32, i32)) -> bool {
        let (a, b, c) = triple;
        if a > self.n || b > self.n || c > self.n || (a == b || b == c || a == c) {
            return false;
        }
        let covered_pairs = Self::get_pairs(&triple);
        for covered_pair in covered_pairs {
            if self.used_pairs.contains_key(&covered_pair) {
                return false;
            }
        }
        true
    }

    fn insert(&mut self, triple: (i32, i32, i32)) {
        //sort to ensure easy searching
        let mut triple = [triple.0, triple.1, triple.2];
        triple.sort();
        let triple = (triple[0], triple[1], triple[2]);
        if !self.can_insert(triple) {
            let covered_pairs = Self::get_pairs(&triple);
            for pair in covered_pairs {
                if self.used_pairs.contains_key(&pair) {
                    println!("{:?}", pair);
                }
            }
            panic!("Cannot insert {triple:?} because it would violate STS properties");
        }

        let covered_pairs = Self::get_pairs(&triple);
        self.blocks.insert(triple, 1);
        for pair in covered_pairs {
            self.used_pairs.insert(pair, 1);
        }
    }

    fn remove(&mut self, triple: (i32, i32, i32)) {
        //sort to ensure easy searching
        let mut triple = [triple.0, triple.1, triple.2];
        triple.sort();
        let triple = (triple[0], triple[1], triple[2]);

        if !self.blocks.contains_key(&triple) {
            panic!("Block not in STS");
        }

        let covered_pairs = Self::get_pairs(&triple);
        self.blocks.remove(&triple);

        for pair in covered_pairs {
            self.used_pairs.remove(&pair);
        }
    }

    fn get_all_possible_blocks(n: i32) -> Vec<(i32, i32, i32)> {
        let all_potential_blocks = (0..n).combinations(3).collect_vec();
        let b = all_potential_blocks
            .into_iter()
            .map(|vector| (vector[0], vector[1], vector[2]));

        let mut all_blocks: Vec<(i32, i32, i32)> = vec![];
        for v in b {
            all_blocks.push(v);
        }
        all_blocks
    }
    fn next_from_seq(&self, num: i32) -> (i32, i32, i32) {
        let n = self.n;
        for i in 0..n {
            for j in 0..n {
                if self.can_insert((i, j, num)) {
                    return (i, j, num);
                }
            }
        }
        panic!("cannon insert");
    }

    fn double_construction(&self) -> Stiner {
        let mut temp_blocks = vec![];
        for b in &self.blocks {
            let x = b.0;
            temp_blocks.push((x.0, x.1, x.2));
            temp_blocks.push((x.0 + self.n, x.1 + self.n, x.2));
            temp_blocks.push((x.0 + self.n, x.1, x.2 + self.n));
            temp_blocks.push((x.0, x.1 + self.n, x.2 + self.n));
        }

        let inf = 2 * (self.n);
        for i in 0..self.n {
            temp_blocks.push((i, i + self.n, inf));
        }

        let mut new_sts = Stiner::new(2 * self.n + 1).unwrap();
        for block in temp_blocks {
            println!("{:?}", block);
            new_sts.insert(block);
        }
        new_sts
    }
}

#[test]
fn good_speedy_strong() {
    let n = 7;
    let strong_starter = speedy_strong::create(n, 0);
}
#[test]
fn good_starter() {
    let n = 7;
    let starter = vec![(1, 3), (2, 6), (4, 5)];
    let strong_starter = starter::Starter::new(n, starter);
    strong_starter.unwrap();
}
#[test]
fn good_strong_starter() {
    let n = 7;
    let strong_starter = starter::Starter::new_strong(n, 0);
}
#[should_panic]
#[test]
fn bad_strong_starter() {
    let n = 7;
    let patterned_starter = starter::Starter::new_patterned(n, 8);
    let result = patterned_starter.is_strong();
    if !result {
        panic!("yay");
    }
}
#[test]
fn good_patterned() {
    for i in 0..500 {
        if i % 2 == 1 {
            let n = 9;
            let _patterned_starter = starter::Starter::new_patterned(n, n + 1);
        }
    }
}

#[should_panic]
#[test]
fn bad_starter() {
    let n = 7;
    let starter = vec![(6, 1), (5, 2), (4, 3), (1, 2)];
    let strong_starter = starter::Starter::new(n, starter);
    strong_starter.expect("This should fail");
}
#[test]
fn good_square() {
    let square = vec![vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]];
    let latin = latin_square::LatinSquare::new(square);
    latin.expect("failure to create square");
}
#[test]
#[should_panic]
fn bad_square_1() {
    let square = vec![vec![0, 1, 2], vec![1, 2, 1], vec![2, 3, 0]];
    let latin = latin_square::LatinSquare::new(square);
    latin.expect("failure to create square");
}
#[test]
#[should_panic]
fn bad_square_2() {
    let square = vec![vec![0, 1, 2], vec![1, 2, 5], vec![2, 3, 0]];
    let latin = latin_square::LatinSquare::new(square);
    latin.expect("failure to create square");
}

#[test]
fn invalid_sts_sizes() {
    let sts = Stiner::new(0);
    if sts.is_err() {
        dbg!("passes");
    } else {
        panic!("This should have failed");
    }
    let sts = Stiner::new(5);
    if sts.is_err() {
        dbg!("passes");
    } else {
        panic!("This should have failed");
    }
    let sts = Stiner::new(7);
    if sts.is_ok() {
        dbg!("passes");
    } else {
        panic!("This should have failed");
    }
    let sts = Stiner::new(9);
    if sts.is_ok() {
        dbg!("passes");
    } else {
        panic!("This should have failed");
    }
}

#[test]
fn fano_plane() {
    let mut sts = Stiner::new(7).unwrap();
    sts.insert((0, 1, 2));
    sts.insert((0, 5, 4));
    sts.insert((0, 6, 3));
    sts.insert((1, 6, 4));
    sts.insert((1, 5, 3));
    sts.insert((2, 6, 5));
    sts.insert((2, 3, 4));
}

#[test]
fn mini_double() {
    let mut sts = Stiner::new(3).unwrap();
    sts.insert((0, 1, 2));
    sts.double_construction();
}
#[should_panic]
fn remove_test() {
    let mut sts = Stiner::new(7).unwrap();
    sts.insert((0, 1, 2));
    sts.insert((0, 5, 4));
    sts.insert((0, 6, 3));
    sts.insert((1, 6, 4));
    sts.insert((1, 5, 3));
    sts.insert((2, 6, 5));
    sts.insert((2, 3, 4));

    sts.remove((2, 3, 4));
    sts.remove((2, 3, 4));
}

fn homework_helper(code: &str) -> i32 {
    let mut map = HashMap::new();
    for i in 0..10 {
        map.insert(format!("{i}"), i);
    }
    map.insert("a".to_string(), 10);
    map.insert("b".to_string(), 11);
    map.insert("c".to_string(), 12);
    let x = map.get(code);
    *x.unwrap()
}

#[test]
fn encoded_test() {
    let n = 13;
    let count = (n * (n - 1)) / 6;
    let mut sts = Stiner::new(n).expect("INVALID STS");

    let seq = "cb6a983579b67ac8cba9cabcbc";
    for i in 0..seq.len() {
        sts.insert(sts.next_from_seq(homework_helper(&seq[i..i + 1])));
    }
    for block in &sts.blocks {
        if block.0 .0 == 0
            || block.0 .1 == 0
            || block.0 .2 == 0
            || block.0 .0 == 1
            || block.0 .1 == 1
            || block.0 .2 == 1
        {
            let a = match block.0 .0 {
                10 => "a".to_string(),
                11 => "b".to_string(),
                12 => "c".to_string(),
                x => x.to_string(),
            };
            let b = match block.0 .0 {
                10 => "a".to_string(),
                11 => "b".to_string(),
                12 => "c".to_string(),
                x => x.to_string(),
            };
            println!("{:?}", block.0);
        }
    }
    let nums = sts.double_construction();
    let based = nums.blocks.keys();
    let based = based
        .sorted_by_key(|block| 100000 * block.0 + 100 * block.1 + block.2)
        .collect_vec();
    println!("{:?}", based);
    println!("Labeled blocks:");
    for block in &based {
        let x = block;
        let a = match x.0 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "a",
            11 => "b",
            12 => "c",

            13 => "\x1b[31m0\x1b[0m",
            14 => "\x1b[31m1\x1b[0m",
            15 => "\x1b[32m2\x1b[0m",
            16 => "\x1b[32m3\x1b[0m",
            17 => "\x1b[32m4\x1b[0m",
            18 => "\x1b[32m5\x1b[0m",
            19 => "\x1b[32m6\x1b[0m",
            20 => "\x1b[32m7\x1b[0m",
            21 => "\x1b[32m8\x1b[0m",
            22 => "\x1b[32m9\x1b[0m",
            23 => "\x1b[32ma\x1b[0m",
            24 => "\x1b[32mb\x1b[0m",
            25 => "\x1b[32mc\x1b[0m",
            _ => "\x1b[31m ∞ \x1b[0m",
        };

        let b = match x.1 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "a",
            11 => "b",
            12 => "c",

            13 => "\x1b[32m0\x1b[0m",
            14 => "\x1b[32m1\x1b[0m",
            15 => "\x1b[32m2\x1b[0m",
            16 => "\x1b[32m3\x1b[0m",
            17 => "\x1b[32m4\x1b[0m",
            18 => "\x1b[32m5\x1b[0m",
            19 => "\x1b[32m6\x1b[0m",
            20 => "\x1b[32m7\x1b[0m",
            21 => "\x1b[32m8\x1b[0m",
            22 => "\x1b[32m9\x1b[0m",
            23 => "\x1b[32ma\x1b[0m",
            24 => "\x1b[32mb\x1b[0m",
            25 => "\x1b[32mc\x1b[0m",
            _ => "\x1b[32m ∞ \x1b[0m",
        };
        let c = match x.2 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "a",
            11 => "b",
            12 => "c",

            13 => "\x1b[32m)\x1b[0m",
            14 => "\x1b[32m!\x1b[0m",
            15 => "\x1b[32m@\x1b[0m",
            16 => "\x1b[32m#\x1b[0m",
            17 => "\x1b[32m$\x1b[0m",
            18 => "\x1b[32m%\x1b[0m",
            19 => "\x1b[32m^\x1b[0m",
            20 => "\x1b[32m&\x1b[0m",
            21 => "\x1b[32m*\x1b[0m",
            22 => "\x1b[32m(\x1b[0m",
            23 => "\x1b[32mA\x1b[0m",
            24 => "\x1b[32mB\x1b[0m",
            25 => "\x1b[32mC\x1b[0m",
            _ => "\x1b[32m∞\x1b[0m",
        };

        print!("{}", c);
    }
    println!("\n\nThere are {} blocks", based.len());
}

#[test]
fn hamilton_test() {
    let n = 31;
    let seed = 0;
    //focus: 31,35,17,55
    let strong_starter = speedy_strong::create(n, seed);
    let strong_starter = Starter::new(n, strong_starter).unwrap();

    assert!(strong_starter.is_strong());
    println!("strong: {:?}", strong_starter);
    print!("k=");
    for k in 0..(2 * n) {
        if k % 2 == 0 {
            let x = Starter::hamilton(n, k, &strong_starter, false);
            if x {
                print!("{},", k % n);
            }
        }
    }

    println!("\n∀(a,b)∈f(P) k=a+b such that: S U f(P) is a hamilton starter)");
}
fn hamilton(n: i32, seed: u32) {
    //focus: 31,35,17,55
    let strong_starter = speedy_strong::create(n, seed);
    let strong_starter = Starter::new(n, strong_starter).unwrap();

    assert!(strong_starter.is_strong());
    println!("strong: {:?}", strong_starter);
    print!("k=");
    for k in 0..(2 * n) {
        if k % 2 == 0 {
            let x = Starter::hamilton(n, k, &strong_starter, false);
            if x {
                print!("{},", k % n);
            }
        }
    }

    println!("\n∀(a,b)∈f(P) k=a+b such that: S U f(P) is a hamilton starter)");
    let mut sums = vec![];
    for pair in &strong_starter.get_pairs() {
        sums.push((pair.0 + pair.1) % n);
    }
    for delta in 2..(n - 2) {
        let mut good = true;
        let mut temps: Vec<i32> = vec![];
        for sum in &sums {
            temps.push((delta * sum) % n)
        }
        for temp in &temps {
            if sums.contains(&temp) {
                good = false;
            }
        }
        if good {
            println!("Delta = {}", delta)
        }
    }
}
fn hamilton_skew(n: i32, seed: u32) {
    let strong_starter_v = speedy_strong::create_skew(n, seed);
    let strong_starter = Starter::new(n, strong_starter_v.clone()).unwrap();

    assert!(strong_starter.is_strong());
    println!("strong: {:?}", strong_starter.get_pairs());
    print!("k=");
    for k in 0..(2 * n) {
        if k % 2 == 0 {
            let x = Starter::hamilton(n, k, &strong_starter, false);
            if x {
                print!("{},", k % n);
            }
        }
    }

    println!("\n∀(a,b)∈f(P) k=a+b such that: S U f(P) is a hamilton starter)");
    println!();
    println!("{}", speedy_strong::skew(&strong_starter_v, n));
    println!();

    let mut assert_skew: HashMap<i32, (i32, i32)> = HashMap::new();
    for pair in &strong_starter_v {
        let sum = (pair.0 + pair.1) % n;
        assert_skew.insert(sum, *pair);
    }
    for key in assert_skew.keys() {
        if assert_skew.contains_key(&(n - key)) {
            println!(
                "{n} {key} {} {:?}",
                (n - key),
                assert_skew.get(&(n - key)).unwrap()
            );
            panic!("Invalid skew starter");
        }
    }
    println!("SUMS:\n{:?}", assert_skew.keys().sorted());
}

fn main() {
    //multiples of 3 seem to be an issue
    let valids = speedy_strong::create_all_delta(25, 1);
    for valid in &valids {
        println!("{:?}", valid);
    }
    /*
    let limit = 1000000;
    for i in 1..limit {
        hamilton(25, i);
        println!("{}", i as f64 / limit as f64);
    }
    */
    //for i in 0..100000 {
    // /}
    //hamilton_skew(15, 0);
}
