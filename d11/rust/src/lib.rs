use std::path::Path;

use regex::Regex;

use rustc_hash::FxHashMap as HashMap;

pub type StoneT = u64;

pub fn parse_stones<P: AsRef<Path>>(path: P) -> Vec<StoneT> {
    let input = std::fs::read_to_string(path).unwrap();

    let num_re = Regex::new(r"(\d+)").unwrap();

    num_re
        .find_iter(&input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn perform_iteration(stones: &mut Vec<StoneT>) {
    let mut idx = 0;

    while idx < stones.len() {
        let stone = stones[idx];
        if stone == 0 {
            stones[idx] = 1;
        } else {
            let num_digits = stone.ilog10() + 1;

            if num_digits % 2 == 0 {
                let split_pow = StoneT::pow(10, num_digits / 2);
                stones[idx] = stone / split_pow;
                idx += 1;
                stones.insert(idx, stone % split_pow);
            } else {
                stones[idx] *= 2024
            }
        }

        idx += 1;
    }
}

pub fn solve_stupid<const NUM_ITER: u32>(mut stones: Vec<StoneT>) -> u64 {
    for _ in 0..NUM_ITER {
        perform_iteration(&mut stones);
    }
    stones.len() as u64
}

pub fn solve_smart<const NUM_ITER: u32>(stones: Vec<StoneT>) -> u64 {
    let mut count_stones = 0;
    let mut stones: Vec<(StoneT, u32)> = stones.into_iter().map(|stone| (stone, 0)).collect();

    while let Some((stone, iteration)) = stones.pop() {
        if iteration == NUM_ITER {
            count_stones += 1;
            continue;
        }

        if stone == 0 {
            stones.push((1, iteration + 1));
            continue;
        }

        let num_digits = stone.ilog10() + 1;

        if num_digits % 2 == 0 {
            let split_pow = StoneT::pow(10, num_digits / 2);
            stones.push((stone / split_pow, iteration + 1));
            stones.push((stone % split_pow, iteration + 1));
        } else {
            stones.push((stone * 2024, iteration + 1))
        }
    }

    count_stones
}

fn compute_mod<const MOD: StoneT>(val: StoneT) -> StoneT {
    val % MOD
}
fn compute_div<const DIV: StoneT>(val: StoneT) -> StoneT {
    val / DIV
}

pub fn solve_smart_fast<const NUM_ITER: u32>(stones: Vec<StoneT>) -> u64 {
    let mut count_stones = 0;
    let mut stones: Vec<(StoneT, u32)> = stones.into_iter().map(|stone| (stone, 0)).collect();

    let (mut stone, mut iteration) = stones.pop().unwrap();

    loop {
        if iteration == NUM_ITER {
            count_stones += 1;
            if let Some((n_stone, n_iteration)) = stones.pop() {
                stone = n_stone;
                iteration = n_iteration;
                continue;
            }
            break;
        }

        if stone == 0 {
            stone = 1;
            iteration += 1;
            continue;
        }

        let num_digits = stone.ilog10() + 1;

        if num_digits % 2 == 0 {
            let first = match num_digits {
                2 => compute_div::<10>(stone),
                4 => compute_div::<100>(stone),
                6 => compute_div::<1000>(stone),
                8 => compute_div::<10000>(stone),
                10 => compute_div::<100000>(stone),
                12 => compute_div::<1000000>(stone),
                14 => compute_div::<10000000>(stone),
                16 => compute_div::<100000000>(stone),
                18 => compute_div::<1000000000>(stone),
                _ => unreachable!(),
            };
            stones.push((first, iteration + 1));
            stone = match num_digits {
                2 => compute_mod::<10>(stone),
                4 => compute_mod::<100>(stone),
                6 => compute_mod::<1000>(stone),
                8 => compute_mod::<10000>(stone),
                10 => compute_mod::<100000>(stone),
                12 => compute_mod::<1000000>(stone),
                14 => compute_mod::<10000000>(stone),
                16 => compute_mod::<100000000>(stone),
                18 => compute_mod::<1000000000>(stone),
                _ => unreachable!(),
            };
        } else {
            stone *= 2024;
        }

        iteration += 1;
    }

    count_stones
}

pub fn solve_lookup<const LOOKUP_TABLE_SIZE: usize>(
    stones: Vec<StoneT>,
    num_iter: usize,
    lookup_tables: &Vec<[u64; LOOKUP_TABLE_SIZE]>,
) -> u64 {
    let mut count_stones = 0;
    let mut stones: Vec<(StoneT, usize)> =
        stones.into_iter().map(|stone| (stone, num_iter)).collect();

    let (mut stone, mut iter_left) = stones.pop().unwrap();

    loop {
        if iter_left == 0 {
            count_stones += 1;
            if let Some((n_stone, n_iter_left)) = stones.pop() {
                stone = n_stone;
                iter_left = n_iter_left;
                continue;
            }
            break;
        }

        if stone < (LOOKUP_TABLE_SIZE as StoneT) && iter_left <= lookup_tables.len() {
            count_stones += lookup_tables[iter_left - 1][stone as usize];
            if let Some((n_stone, n_iter_left)) = stones.pop() {
                stone = n_stone;
                iter_left = n_iter_left;
                continue;
            }
            break;
        }

        if stone == 0 {
            stone = 1;
            iter_left -= 1;
            continue;
        }

        let num_digits = stone.ilog10() + 1;

        if num_digits % 2 == 0 {
            let first = match num_digits {
                2 => compute_div::<10>(stone),
                4 => compute_div::<100>(stone),
                6 => compute_div::<1000>(stone),
                8 => compute_div::<10000>(stone),
                10 => compute_div::<100000>(stone),
                12 => compute_div::<1000000>(stone),
                14 => compute_div::<10000000>(stone),
                16 => compute_div::<100000000>(stone),
                18 => compute_div::<1000000000>(stone),
                _ => unreachable!(),
            };
            stones.push((first, iter_left - 1));
            stone = match num_digits {
                2 => compute_mod::<10>(stone),
                4 => compute_mod::<100>(stone),
                6 => compute_mod::<1000>(stone),
                8 => compute_mod::<10000>(stone),
                10 => compute_mod::<100000>(stone),
                12 => compute_mod::<1000000>(stone),
                14 => compute_mod::<10000000>(stone),
                16 => compute_mod::<100000000>(stone),
                18 => compute_mod::<1000000000>(stone),
                _ => unreachable!(),
            };
        } else {
            stone *= 2024;
        }

        iter_left -= 1;
    }

    count_stones
}

pub fn calc_lookup_tables<const LOOKUP_TABLE_SIZE: usize>(
    num_iter: usize,
) -> Vec<[u64; LOOKUP_TABLE_SIZE]> {
    let mut lookup_tables = vec![];

    for iter in 1..num_iter {
        let new_lookup_table = std::array::from_fn(|stone| {
            solve_lookup::<LOOKUP_TABLE_SIZE>(vec![stone as StoneT], iter, &lookup_tables)
        });
        lookup_tables.push(new_lookup_table);
    }

    lookup_tables
}

fn solve_memoization_rec(
    stone: StoneT,
    num_iter: u64,
    cache: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    if num_iter == 0 {
        return 1;
    }

    if let Some(prod_stones) = cache.get(&(stone, num_iter)) {
        return *prod_stones;
    }

    let prod_stones = if stone == 0 {
        solve_memoization_rec(1, num_iter - 1, cache)
    } else {
        let num_digits = stone.ilog10() + 1;

        if num_digits % 2 == 0 {
            let split_pow = StoneT::pow(10, num_digits / 2);
            solve_memoization_rec(stone / split_pow, num_iter - 1, cache)
                + solve_memoization_rec(stone % split_pow, num_iter - 1, cache)
        } else {
            solve_memoization_rec(stone * 2024, num_iter - 1, cache)
        }
    };

    cache.insert((stone, num_iter), prod_stones);
    return prod_stones;
}

pub fn solve_memoization(stones: Vec<StoneT>, num_iter: u64) -> u64 {
    let mut cache = HashMap::default();

    let mut count_stones = 0;
    for stone in stones {
        count_stones += solve_memoization_rec(stone, num_iter, &mut cache);
    }
    count_stones
}
