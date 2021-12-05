pub fn main() {
    let input = include_str!("../../input/day03/input.txt");

    let sol1 = solve_part1(&input);
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&input);
    println!("Part two answer: {}", sol2);
}

fn vec_to_num(v: &Vec<i32>, cmp: i32) -> i32 {
    v.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| if v == cmp { 0 } else { 1 << i })
        .sum()
}

fn solve_part1(input: &str) -> i32 {
    let input_size: i32 = input.lines().count() as i32;
    let line_len = input.lines().next().unwrap().len();

    let res: Vec<i32> = input
        .lines()
        .map(|v| {
            v.chars()
                .into_iter()
                .filter_map(|n| match n {
                    '0' => Some(0),
                    '1' => Some(1),
                    _ => None,
                })
                .collect()
        })
        .fold(vec![0; line_len], |acc, v: Vec<i32>| {
            acc.iter().zip(&v).map(|(a, b)| a + b).collect()
        })
        .iter()
        .map(|&n| if n > (input_size / 2) { 1 } else { 0 })
        .collect();

    let i = vec_to_num(&res, 0);
    let j = vec_to_num(&res, 1);

    println!("{}", i * j);
    i * j
}

fn reduce<F>(vv: Vec<Vec<i32>>, idx: usize, cmp: F) -> Vec<Vec<i32>>
where
    F: Fn(usize, usize) -> bool,
{
    let sum = vv.iter().fold(0, |acc, x| acc + x[idx]) as usize;

    let keep = if cmp(sum, (vv.len() + 1) / 2) { 1 } else { 0 };

    let vv: Vec<Vec<i32>> = vv.into_iter().filter(|x| x[idx] == keep).collect();

    if idx != 11 && vv.len() != 1 {
        reduce(vv, idx + 1, cmp)
    } else {
        vv
    }
}

fn solve_part2(input: &str) -> i32 {
    let vv: Vec<Vec<i32>> = input
        .lines()
        .map(|v| {
            v.chars()
                .into_iter()
                .filter_map(|n| match n {
                    '0' => Some(0),
                    '1' => Some(1),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let i = reduce(vv.clone(), 0, |i, j| i >= j);
    let j = reduce(vv.clone(), 0, |i, j| i < j);

    let i = vec_to_num(&i[0], 0);
    let j = vec_to_num(&j[0], 0);

    i * j
}
