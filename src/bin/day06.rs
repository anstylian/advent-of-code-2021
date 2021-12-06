fn main() {
    let input: Vec<_> = include_str!("../../input/day06/input.txt")
        .lines()
        .into_iter()
        .map(|l| l.split(',').map(|n| n.parse::<u64>().unwrap()))
        .flatten()
        .collect();

    let sol1 = solve(&input, 80);
    println!("Part one answer: {}", sol1);

    let sol2 = solve(&input, 256);
    println!("Part two answer: {}", sol2);
}

fn solve(input: &Vec<u64>, days: u64) -> u64 {
    let mut count = [0u64; 9];
    let mut input = input.clone();
    input.sort();

    for i in 0..=8 {
        count[i] = input.iter().filter(|&&n| i == n as usize).count() as u64;
    }

    for _ in 0..days {
        count.rotate_left(1);
        count[6] += count[8];
    }

    count.iter().sum()
}
