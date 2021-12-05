pub fn main() {
    let x = include_str!("../../input/day02/input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(h, d), input| match input {
            ("forward", n) => (h + n.parse::<i32>().unwrap(), d),
            ("down", n) => (h, d + n.parse::<i32>().unwrap()),
            ("up", n) => (h, d - n.parse::<i32>().unwrap()),
            _ => unreachable!(),
        });
    println!("Part one answer: {}", x.0 * x.1);

    let x = include_str!("../../input/day02/input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(h, d, a), input| match input {
            ("down", n) => (h, d, a + n.parse::<i32>().unwrap()),
            ("up", n) => (h, d, a - n.parse::<i32>().unwrap()),
            ("forward", n) => (
                h + n.parse::<i32>().unwrap(),
                d + a * n.parse::<i32>().unwrap(),
                a,
            ),
            _ => unreachable!(),
        });
    println!("Part two answer: {}", x.0 * x.1);
}
