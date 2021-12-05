fn main() {
    let lines: Vec<Vec<(_, _)>> = include_str!("../../input/day05/input.txt")
        .lines()
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .into_iter()
                .filter(|&i| i != "->")
                .map(|p| {
                    let mut split = p.split(',');
                    (
                        split.next().unwrap().parse::<u32>().unwrap(),
                        split.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let lines: Vec<_> = lines.into_iter().map(|l| (l[0], l[1])).collect();

    let sol1 = solve_part1(&lines);
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&lines);
    println!("Part two answer: {}", sol2);
}

fn draw_horr(array: &mut Vec<Vec<usize>>, l: &((u32, u32), (u32, u32))) {
    let p0 = l.0;
    let p1 = l.1;

    if p0.1 == p1.1 {
        let range = if p0.0 < p1.0 {
            p0.0..=p1.0
        } else {
            p1.0..=p0.0
        };

        for i in range {
            array[i as usize][p0.1 as usize] += 1;
        }
    }
}

fn draw_vert(array: &mut Vec<Vec<usize>>, l: &((u32, u32), (u32, u32))) {
    let p0 = l.0;
    let p1 = l.1;

    if p0.0 == p1.0 {
        let range = if p0.1 < p1.1 {
            p0.1..=p1.1
        } else {
            p1.1..=p0.1
        };

        for i in range {
            array[p0.0 as usize][i as usize] += 1;
        }
    }
}

fn draw_diag(array: &mut Vec<Vec<usize>>, l: &((u32, u32), (u32, u32))) {
    let p0 = l.0;
    let p1 = l.1;

    if !(p0.1 == p1.1 || p0.0 == p1.0) {
        let rows: Vec<u32> = if p0.0 < p1.0 {
            (p0.0..=p1.0).collect()
        } else {
            (p1.0..=p0.0).rev().collect()
        };

        let cols: Vec<u32> = if p0.1 < p1.1 {
            (p0.1..=p1.1).collect()
        } else {
            (p1.1..=p0.1).rev().collect()
        };

        rows.iter().zip(cols.iter()).for_each(|i: (&u32, &u32)| {
            array[*i.0 as usize][*i.1 as usize] += 1;
        });
    }
}

fn solve_part1(lines: &Vec<((u32, u32), (u32, u32))>) -> usize {
    let mut array = vec![vec![0usize; 1000]; 1000];

    for l in lines {
        draw_vert(&mut array, &l);
        draw_horr(&mut array, &l);
    }

    array
        .iter()
        .map(|r| r.iter().filter(|&&v| v > 1).count())
        .sum()
}

fn solve_part2(lines: &Vec<((u32, u32), (u32, u32))>) -> usize {
    let mut array = vec![vec![0usize; 1000]; 1000];

    for l in lines {
        draw_vert(&mut array, &l);
        draw_horr(&mut array, &l);
        draw_diag(&mut array, &l);
    }

    array
        .iter()
        .map(|r| r.iter().filter(|&&v| v > 1).count())
        .sum()
}
