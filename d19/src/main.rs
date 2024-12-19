use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Display,
    ops::{Index, Range, RangeFrom},
    time::Instant,
};

use radix_trie::{NibbleVec, Trie, TrieCommon, TrieKey};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for Color {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Self::White),
            'u' => Ok(Self::Blue),
            'b' => Ok(Self::Black),
            'r' => Ok(Self::Red),
            'g' => Ok(Self::Green),
            _ => Err(value),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Blue => write!(f, "u"),
            Color::Black => write!(f, "b"),
            Color::Red => write!(f, "r"),
            Color::Green => write!(f, "g"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct StripePattern(Vec<Color>);

impl TrieKey for StripePattern {
    fn encode_bytes(&self) -> Vec<u8> {
        self.0.iter().map(|&color| color as u8).collect()
    }

    fn encode(&self) -> NibbleVec<[u8; 64]> {
        NibbleVec::<[u8; 64]>::from_byte_vec(self.encode_bytes())
    }
}

impl Index<usize> for StripePattern {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for StripePattern {
    type Output = [Color];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}
impl Index<RangeFrom<usize>> for StripePattern {
    type Output = [Color];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Borrow<[Color]> for StripePattern {
    fn borrow(&self) -> &[Color] {
        &self.0
    }
}

impl Display for StripePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for color in &self.0 {
            write!(f, "{color}")?;
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut input_parts = input.split("\n\n");

    let towels: Trie<StripePattern, ()> = input_parts
        .next()
        .unwrap()
        .split(", ")
        .map(|towel| {
            (
                StripePattern(
                    towel
                        .chars()
                        .map(Color::try_from)
                        .map(Result::unwrap)
                        .collect(),
                ),
                (),
            )
        })
        .collect();

    let patterns: Vec<StripePattern> = input_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            StripePattern(
                line.chars()
                    .map(Color::try_from)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect();

    assert!(input_parts.next().is_none());

    let start_a = Instant::now();
    let mut possible_patterns = 0u64;
    for pattern in &patterns {
        let mut queue = vec![0usize];

        while let Some(unconvered_pos) = queue.pop() {
            if unconvered_pos == pattern.0.len() {
                possible_patterns += 1;
                break;
            }

            let sub = towels.get_ancestor(&StripePattern(pattern[unconvered_pos..].to_vec()));
            if let Some(sub) = sub {
                let matched_len = sub.key().unwrap().0.len();

                for k in 1..matched_len {
                    if let Some(sub) = towels.get_ancestor(&StripePattern(
                        pattern[unconvered_pos..(unconvered_pos + k)].to_vec(),
                    )) {
                        if sub.value().is_some() {
                            queue.push(unconvered_pos + k);
                        }
                    }
                }

                if sub.value().is_some() {
                    queue.push(unconvered_pos + matched_len);
                }
            }
        }
    }
    let elapsed_a = start_a.elapsed();

    let start_b = Instant::now();
    let mut output_b = 0;
    let mut cache = HashMap::new();
    for pattern in patterns {
        output_b += find_num_ways(&towels, &pattern.0, &mut cache);
    }
    let elapsed_b = start_b.elapsed();

    println!("Task1: {possible_patterns}");
    println!("Task2: {output_b}");

    println!("Task1 took {}ms!", elapsed_a.as_millis());
    println!("Task2 took {}ms!", elapsed_b.as_millis());
}

fn find_num_ways(
    towels: &Trie<StripePattern, ()>,
    pattern: &[Color],
    cache: &mut HashMap<Vec<Color>, u64>,
) -> u64 {
    if pattern.len() == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(pattern) {
        return *cached;
    }

    let mut num_ways = 0;
    let mut prev_len = pattern.len();
    while prev_len > 0 {
        if let Some(sub) = towels.get_ancestor(&StripePattern(pattern[..prev_len].to_vec())) {
            let prefix_len = sub.key().unwrap().0.len();
            assert!(prefix_len > 0);
            if sub.value().is_some() {
                num_ways += find_num_ways(towels, &pattern[prefix_len..], cache);
            }
            prev_len = sub.key().unwrap().0.len();
        }
        prev_len -= 1;
    }
    cache.insert(pattern.to_vec(), num_ways);
    return num_ways;
}
