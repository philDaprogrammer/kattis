use std::io::{self, BufRead, Stdin};

const REM: u32 = 1000000007;

fn parse() -> Vec<char> {
    let stdin: Stdin = io::stdin();

    stdin.lock().lines().map(|l| l.unwrap())
        .collect::<Vec<String>>()[0].chars().collect()
}

/**
 * Solution for 0-1 sequences on kattis.
 * Please see the brief paper explaining
 * the solution
 */
fn solve(bits: Vec<char>) {
    let mut k:    u32 = 1; // number of possible outcomes at iteration i
    let mut inv:  u32 = 0; // number of inversions at iteration i
    let mut ones: u32 = 0; // number of total ones at iteration i

    for b in bits {
        match  b {
            '1' => {
                ones = (ones + k) % REM;
            }
            '0' => {
                inv   = (inv + ones) % REM;
            }
            '?' => {
                inv   = ((inv << 1) + ones) % REM;
                ones  = ((ones << 1) + k) % REM;
                k     = (k << 1) % REM;
            }
            _ => {}
        }
    }

    println!("{}", inv);
}

fn main() -> io::Result<()> {
    solve(parse());
    Ok(())
}