use aoc::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../../input/day02.txt");

    println!("part 1: {}", part1(input)?);
    println!("part 2: {}", part2(input)?);

    Ok(())
}

fn is_invalid(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let mid = s.len() / 2;
    let (first_half, second_half) = s.split_at(mid);
    first_half == second_half
}

fn part1(input: &str) -> Result<u64, Error> {
    let mut total_sum: u64 = 0;

    for range in input.trim().split(',') {
        let (lower, upper) = range.split_once('-').ok_or("lol")?;

        let lower = lower.trim().parse::<u64>()?;
        let upper = upper.trim().parse::<u64>()?;

        for n in lower..=upper {
            let n_str = n.to_string();

            if is_invalid(&n_str) {
                total_sum += n;
            }
        }
    }

    Ok(total_sum)
}

fn is_periodic(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }

    let repeated = format!("{}{}", s, s);
    let middle = &repeated[1..repeated.len() - 1];

    middle.contains(s)
}

fn part2(input: &str) -> Result<u64, Error> {
    let mut total_sum: u64 = 0;

    for range in input.trim().split(',') {
        let (lower, upper) = range.split_once('-').ok_or("lol")?;

        let lower = lower.trim().parse::<u64>()?;
        let upper = upper.trim().parse::<u64>()?;

        for n in lower..=upper {
            let n_str = n.to_string();

            if is_periodic(&n_str) {
                total_sum += n;
            }
        }
    }

    Ok(total_sum)
}
