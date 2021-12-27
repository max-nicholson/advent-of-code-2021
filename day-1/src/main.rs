fn main() {
    let depths: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let increments =
        depths.iter().zip(depths.iter().skip(1)).fold(
            0,
            |acc, (prev, curr)| {
                if curr > prev {
                    acc + 1
                } else {
                    acc
                }
            },
        );
    println!("{}", increments);
}
