#[derive(Default, Debug, Clone)]
struct Card {
    row_sum: [u32; 5],
    col_sum: [u32; 5],
    values: [[u32; 5]; 5],
    solved: bool,
}

impl Card {
    fn mark(&mut self, input: u32) -> Option<u32> {
        for idx in 0..5 {
            for jdx in 0..5 {
                if self.values[idx][jdx] == input {
                    self.row_sum[idx] -= input;
                    self.col_sum[jdx] -= input;
                    self.values[idx][jdx] = 0;

                    self.solved = self.row_sum[idx] == 0 || self.col_sum[jdx] == 0;
                }
            }
        }

        if self.solved {
            Some(self.row_sum.iter().sum::<u32>())
        } else {
            None
        }
    }

    fn val(&self) -> u32 {
        self.row_sum.iter().sum::<u32>()
    }
}

fn main() {
    let mut lines: Vec<_> = include_str!("../../input/day04/input.txt")
        .lines()
        .into_iter()
        .collect();

    let play = lines
        .remove(0)
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut v: Vec<_> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .chunks(5)
        .into_iter()
        .map(|v| {
            let mut c = Card::default();
            v.iter().enumerate().for_each(|(i, &&n)| {
                n.split_whitespace()
                    .enumerate()
                    .for_each(|(j, nn)| c.values[i][j] = nn.parse::<u32>().unwrap());
                c.row_sum[i] = c.values[i].iter().sum();
            });
            for j in 0..5 {
                for i in 0..5 {
                    c.col_sum[j] += c.values[i][j];
                }
            }
            c
        })
        .collect();

    let sol1 = play1(v.clone(), &play);
    println!("Part one answer: {}", sol1.unwrap_or(0));

    let sol2 = play2(&mut v, &play);
    println!("Part two answer: {}", sol2.unwrap_or(0));
}

fn play1(mut card: Vec<Card>, input: &Vec<u32>) -> Option<u32> {
    for i in input {
        for c in 0..card.len() {
            match card[c].mark(*i) {
                Some(n) => {
                    return Some(n * i);
                }
                None => {}
            }
        }
    }

    None
}

fn play2(card: &mut Vec<Card>, input: &Vec<u32>) -> Option<u32> {
    for i in input {
        for c in &mut *card {
            c.mark(*i);
        }

        if card.len() == 1 && card[0].solved {
            return Some(card[0].val() * i);
        } else {
            card.retain(|c| !c.solved);
        }
    }

    None
}
