fn main() {
    let depths = read_depths();
    println!("a: {}", a(depths.clone()));
    println!("b: {}", b(depths));
}

fn read_depths() -> Vec<usize> {
    include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn a(depths: Vec<usize>) -> usize {
    depths.iter().zip(depths.iter().skip(1)).fold(
        0,
        |acc, (prev, curr)| {
            if curr > prev {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn b(depths: Vec<usize>) -> usize {
    let windows: Vec<usize> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    windows
        .iter()
        .zip(windows.iter().skip(1))
        .fold(0, |acc, (prev, curr)| acc + (curr > prev) as usize)
}
