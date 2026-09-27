use std::sync::LazyLock;

use nanorand::{Rng, WyRand};

static WORDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    include_str!("buzzwords.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
});

pub fn pick(count: usize) -> Vec<&'static str> {
    let count = count.min(WORDS.len());
    let mut random = WyRand::new();
    let mut chosen: Vec<usize> = Vec::with_capacity(count);

    while chosen.len() < count {
        let index = random.generate_range(0..WORDS.len());
        if !chosen.contains(&index) {
            chosen.push(index);
        }
    }

    chosen.into_iter().map(|index| WORDS[index]).collect()
}
