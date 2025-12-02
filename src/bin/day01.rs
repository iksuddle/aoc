use aoc::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../../input/day01.txt");

    println!("part 1: {}", part1(input)?);
    println!("part 2: {}", part2(input)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32, Error> {
    let mut counter = 0;
    let mut pointer = 50;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num: i32 = num.parse()?;

        pointer = if dir == "L" {
            (pointer - num) % 100
        } else {
            (pointer + num) % 100
        };

        if pointer == 0 {
            counter += 1;
        }
    }

    Ok(counter)
}

fn part2(input: &str) -> Result<i32, Error> {
    let mut counter = 0;
    let mut pointer = 50;

    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num: i32 = num.parse()?;

        if dir == "L" {
            let mag = if pointer == 0 { 100 } else { pointer };
            if num >= mag {
                counter += 1 + (num - mag) / 100;
            }
            pointer = (pointer - num).rem_euclid(100);
        } else {
            counter += (pointer + num) / 100;
            pointer = (pointer + num).rem_euclid(100);
        }
    }

    Ok(counter)
}
