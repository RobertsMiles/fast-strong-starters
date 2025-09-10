use crate::Starter;
use itertools::Itertools::{self};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn skew(starter: &Vec<(i32, i32)>, n: i32) -> bool {
    let mut assert_skew: HashMap<i32, (i32, i32)> = HashMap::new();
    for pair in starter {
        let sum = (pair.0 + pair.1) % n;
        assert_skew.insert(sum, *pair);
    }
    for key in assert_skew.keys() {
        if assert_skew.contains_key(&(n - key)) {
            return false;
        }
    }

    true && starter.len() == ((n - 1) / 2) as usize
}

pub fn create(n: i32, nth: u32) -> Vec<(i32, i32)> {
    let mut nth = nth;
    let t = (n - 1) / 2;
    let pairs = (1..n).combinations(2).collect_vec();

    let mut s: Vec<(i32, i32)> = vec![];
    let mut memory: Vec<usize> = vec![];

    for _i in 0..(t) {
        memory.push(0);
    }

    while s.len() < t as usize || nth > 0 {
        s = vec![];
        let mut differences: HashSet<i32> = HashSet::new();
        let mut used: HashSet<i32> = HashSet::new();
        let mut sums: HashSet<i32> = HashSet::new();

        for i in 0..memory.len() {
            let mut index = i;
            let mut ran = false;
            while memory[index] >= pairs.len() {
                memory[index] = 0;
                index -= 1;
                ran = true;
            }
            if ran {
                memory[index] += 1;
            }
            let data = memory[i];
            let pair = (pairs[data][0], pairs[data][1]);
            if can_add(n, pair, &differences, &sums, &used) {
                s.push(pair);
                insert(n, pair, &mut differences, &mut sums, &mut used);
            } else {
                let mut counter = 0;
                let mut pair = (pairs[data + counter][0], pairs[data + counter][1]);
                while data + counter < pairs.len() && !can_add(n, pair, &differences, &sums, &used)
                {
                    pair = (pairs[data + counter][0], pairs[data + counter][1]);
                    counter += 1;
                }
                if data + counter >= pairs.len() {
                    memory[i] = 0;
                    let mut index = i - 1;
                    while memory[index] + 1 >= pairs.len() {
                        memory[index] = 0;
                        if index > 0 {
                            index -= 1
                        } else {
                            break;
                        }
                    }
                    memory[index] += 1;
                    break;
                } else if can_add(n, pair, &differences, &sums, &used) {
                    memory[i] = data + counter;

                    s.push(pair);
                    insert(n, pair, &mut differences, &mut sums, &mut used);
                }
            }
        }
        if s.len() == t as usize && nth > 0 {
            nth -= 1;
            memory[(t - 1) as usize] += 1;
        } else if s.len() == t as usize && nth == 0 {
            return s;
        }
    }
    s
}
pub fn create_all_delta(n: i32, nth: u32) -> HashSet<Vec<(i32, i32)>> {
    let orig_seed = nth;
    let mut uniques: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let mut nth = nth;
    let t = (n - 1) / 2;
    let pairs = (1..n).combinations(2).collect_vec();

    let mut s: Vec<(i32, i32)> = vec![];
    let mut memory: Vec<usize> = vec![];

    for _i in 0..(t) {
        memory.push(0);
    }

    while s.len() < t as usize || nth > 0 {
        s = vec![];
        let mut differences: HashSet<i32> = HashSet::new();
        let mut used: HashSet<i32> = HashSet::new();
        let mut sums: HashSet<i32> = HashSet::new();

        for i in 0..memory.len() {
            let mut index = i;
            let mut ran = false;
            while memory[index] >= pairs.len() {
                memory[index] = 0;
                index -= 1;
                ran = true;
            }
            if ran {
                memory[index] += 1;
            }
            let data = memory[i];
            let pair = (pairs[data][0], pairs[data][1]);
            if can_add(n, pair, &differences, &sums, &used) && check_delta(n, &s, pair) {
                s.push(pair);
                insert(n, pair, &mut differences, &mut sums, &mut used);
            } else {
                let mut counter = 0;
                let mut pair = (pairs[data + counter][0], pairs[data + counter][1]);
                while data + counter < pairs.len()
                    && !(can_add(n, pair, &differences, &sums, &used) && check_delta(n, &s, pair))
                {
                    pair = (pairs[data + counter][0], pairs[data + counter][1]);
                    counter += 1;
                }
                if data + counter >= pairs.len() {
                    memory[i] = 0;
                    let mut index = i - 1;
                    while memory[index] + 1 >= pairs.len() {
                        memory[index] = 0;
                        if index > 0 {
                            index -= 1
                        } else {
                            break;
                        }
                    }
                    memory[index] += 1;
                    break;
                } else if can_add(n, pair, &differences, &sums, &used) && check_delta(n, &s, pair) {
                    memory[i] = data + counter;

                    s.push(pair);
                    insert(n, pair, &mut differences, &mut sums, &mut used);
                }
            }
        }
        if s.len() == t as usize && nth > 0 {
            let mut copy_s = s.clone();
            copy_s.sort();
            if !uniques.contains(&copy_s) {
                let mut kappas: HashSet<i32> = HashSet::new();
                for k in 0..(2 * n) {
                    if k % 2 == 0 {
                        let x =
                            Starter::hamilton(n, k, &Starter::new(n, s.clone()).unwrap(), false);
                        if x {
                            kappas.insert(k % n);
                        }
                    }
                }
                println!(
                    "n={} {:?} deltas={:?} kappas = {:?}",
                    n,
                    copy_s,
                    get_delta(n, &copy_s),
                    kappas
                );
                println!();
            }
            uniques.insert(copy_s);

            nth -= 1;
            memory[(t - 1) as usize] += 1;
        } else if s.len() == t as usize && nth == 0 {
            return uniques;
        }
    }
    uniques
}

pub fn create_skew(n: i32, nth: u32) -> Vec<(i32, i32)> {
    let mut nth = nth;
    let t = (n - 1) / 2;
    let pairs = (1..n).combinations(2).collect_vec();

    let mut s: Vec<(i32, i32)> = vec![];
    let mut memory: Vec<usize> = vec![];

    let mut record = 0;

    for _i in 0..(t) {
        memory.push(0);
    }

    let mut num_valid = 0;
    while s.len() < t as usize || nth > 0 || !skew(&s, n) {
        if s.len() >= record {
            //println!("{:?}, len = {}", s, s.len());
            record = s.len()
        }
        s = vec![];
        let mut differences: HashSet<i32> = HashSet::new();
        let mut used: HashSet<i32> = HashSet::new();
        let mut sums: HashSet<i32> = HashSet::new();

        for i in 0..memory.len() {
            let mut index = i;
            let mut ran = false;
            while memory[index] >= pairs.len() {
                memory[index] = 0;
                index -= 1;
                ran = true;
            }
            if ran {
                memory[index] += 1;
            }
            let data = memory[i];
            let pair = (pairs[data][0], pairs[data][1]);
            if can_add(n, pair, &differences, &sums, &used) {
                s.push(pair);
                insert(n, pair, &mut differences, &mut sums, &mut used);
            } else {
                let mut counter = 0;
                let mut pair = (pairs[data + counter][0], pairs[data + counter][1]);
                while data + counter < pairs.len() && !can_add(n, pair, &differences, &sums, &used)
                {
                    pair = (pairs[data + counter][0], pairs[data + counter][1]);
                    counter += 1;
                }
                if data + counter >= pairs.len() {
                    memory[i] = 0;
                    let mut index = i - 1;
                    while memory[index] + 1 >= pairs.len() {
                        memory[index] = 0;
                        if index > 0 {
                            index -= 1
                        } else {
                            break;
                        }
                    }
                    memory[index] += 1;
                    break;
                } else if can_add(n, pair, &differences, &sums, &used) {
                    memory[i] = data + counter;

                    s.push(pair);
                    insert(n, pair, &mut differences, &mut sums, &mut used);
                }
            }
        }
        if (s.len() == t as usize && nth > 0) || !skew(&s, n) {
            if skew(&s, n) {
                nth -= 1;
            }
            if s.len() == t as usize {
                num_valid += 1;
                /*
                println!(
                    "Found: {} valid strong starters that are not skew",
                    num_valid
                );
                */
            }
            memory[(t - 1) as usize] += 1;
        } else if s.len() == t as usize && nth == 0 && skew(&s, n) {
            return s;
        }
    }
    s
}
fn check_delta(n: i32, starter: &Vec<(i32, i32)>, potential_pair: (i32, i32)) -> bool {
    let mut valid_deltas = HashSet::new();
    let mut sums = HashSet::new();
    for pair in starter {
        sums.insert((pair.0 + pair.1) % n);
    }
    sums.insert((potential_pair.0 + potential_pair.1) % n);
    for delta in 2..(n - 1) {
        let mut good = true;
        for s in &sums {
            let ds = (delta * s) % n;
            if sums.contains(&ds) {
                good = false;
            }
        }
        if good {
            valid_deltas.insert(delta);
        }
    }
    valid_deltas.len() > 0
}

fn get_delta(n: i32, starter: &Vec<(i32, i32)>) -> HashSet<i32> {
    let mut valid_deltas = HashSet::new();
    let mut sums = HashSet::new();
    for pair in starter {
        sums.insert((pair.0 + pair.1) % n);
    }
    for delta in 2..(n - 1) {
        let mut good = true;
        for s in &sums {
            let ds = (delta * s) % n;
            if sums.contains(&ds) {
                good = false;
            }
        }
        if good {
            valid_deltas.insert(delta);
        }
    }
    valid_deltas
}

fn can_add(
    n: i32,
    pair: (i32, i32),
    differences: &HashSet<i32>,
    sums: &HashSet<i32>,
    used: &HashSet<i32>,
) -> bool {
    let a = pair.0;
    let b = pair.1;
    if a == 0 || b == 0 {
        return false;
    }
    let sum = (a + b) % n;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };
    let diff = if diff > n / 2 { n - diff } else { diff };
    /*
    let deltas = get_delta(n, sums);
    let mut good = false;
    for delta in &deltas {
        good = good || !sums.contains(&((delta * sum) % n));
    }
    if !good {
        return false;
    }
    */
    if sums.contains(&sum) || sum % n == 0 {
        return false;
    }

    if differences.contains(&diff) {
        return false;
    }

    if used.contains(&a) || used.contains(&b) {
        return false;
    }

    true
}

fn insert(
    n: i32,
    pair: (i32, i32),
    differences: &mut HashSet<i32>,
    sums: &mut HashSet<i32>,
    used: &mut HashSet<i32>,
) {
    let a = pair.0;
    let b = pair.1;
    let sum = (a + b) % n;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };

    let diff = if diff > n / 2 { n - diff } else { diff };
    if !can_add(n, pair, differences, sums, used) {
        panic!("cannot insert");
    }
    sums.insert(sum);
    sums.insert(n - sum);
    differences.insert(diff);

    used.insert(a);
    used.insert(b);
}

fn remove(
    n: i32,
    pair: (i32, i32),
    differences: &mut HashSet<i32>,
    sums: &mut HashSet<i32>,
    used: &mut HashSet<i32>,
) {
    let a = pair.0;
    let b = pair.1;
    let sum = a + b;
    let diff = {
        let mut x = a - b;
        let mut y = b - a;
        if x < 0 {
            x *= -1;
        }
        if y < 0 {
            y *= -1;
        }
        if x < y {
            x
        } else {
            y
        }
    };

    let diff = if diff > n / 2 { n - diff } else { diff };
    if can_add(n, pair, differences, sums, used) {
        panic!("cannot remove");
    }
    sums.remove(&sum);
    differences.remove(&diff);

    used.remove(&a);
    used.remove(&b);
}
