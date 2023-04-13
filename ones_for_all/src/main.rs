use std::io::{self, Stdin, BufRead};

const MAX_INT: u32 = 1 << 31;

fn parse() -> u32 {
    let stdin: Stdin = io::stdin();

    stdin.lock().lines()
        .map(|r | r.unwrap()).collect::<Vec<String>>()[0].parse().unwrap()
}

fn solve(n: u32) -> u32 {
    /* set all base cases, i.e. primitive complexities.
     * A primitive complexity is a number who's integer complexity
     * is itself
     */
    let mut opt: Vec<u32> = vec![0; (n+1) as usize];
    opt[0] = 0;
    opt[1] = 1;
    opt[2] = 2;
    opt[3] = 3;
    opt[4] = 4;
    opt[5] = 5;

    for i in 6..(n+1) as usize {
        let mut smallest= MAX_INT;

        for p in 2 .. i/2 as usize {
            let contender = opt[i / p] + opt[p] + opt[i % p];

            if smallest == MAX_INT || contender < smallest {
                smallest = contender
            }
        }

        opt[i] = smallest;

        // need to check concat complexity
        let digits = i.to_string();
        let chars = digits.chars().collect::<Vec<char>>();

        for j in 1..digits.len()  {
            let left: usize  = digits[0..j].parse().unwrap();
            let right: usize = digits[j..digits.len()].parse().unwrap();

            if (opt[left] + opt[right]) < opt[i] && *chars.get(j).unwrap() != '0' {
                opt[i] = opt[left] + opt[right]
            }
        }
    }

    opt[n as usize]
}

fn main() -> io::Result<()> {
    println!("{}", solve( parse()));
    Ok(())
}
