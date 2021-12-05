fn main() {
    let numbers: Vec<_> = include_str!("../../input/day01/input.txt")
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let sol1 = solve_part1(&numbers);
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&numbers);
    println!("Part two answer: {}", sol2);
}

fn solve_part1(numbers: &[i32]) -> usize {
    numbers.windows(2).filter(|f| f[0] < f[1]).count()
}

fn solve_part2(numbers: &[i32]) -> usize {
    numbers.windows(4).filter(|f| f[0] < f[3]).count()
}
