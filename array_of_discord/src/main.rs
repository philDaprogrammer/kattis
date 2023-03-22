use std::io::{self, Stdin, BufRead};

fn parse() -> Vec<String> {
    let stdin: Stdin       = io::stdin();
    let lines: Vec<String> = stdin.lock()
        .lines().map(|s| s.unwrap()).collect();

    lines[1].split_whitespace().map(|s| s.to_string())
        .collect()
}

fn dump_list(nums: &Vec<String>) {
    for num in nums {
        print!("{} ", num);
    }

    println!()
}


/** Solve in O(n) time
 *
 * Let a and b both be consecutive numbers
 * in the sorted list.
 *
 * If len(a) != len(b) then it is impossible
 * to invalidate the sorting as we cannot
 * remove digits.
 *
 * Otherwise, we only need to compare the most
 * significant digit of a and b. If we cannot
 * invalidate the sorting by changing one of the
 * most significant digits, then it is impossible
 * to invalidate the sorting with that pair.
 *
 * It is only impossible to invalidate in two cases
 * when the strings are of equal length. If len(a) and
 * len(b) = 1, then we cannot invalidate when
 * a[0] = 0, and b[0] = 9. If their lengths
 * are greater than one, we cannot invalidate
 * when a[0] = 1 and b[0] = 9. The logic is
 * fairly straight forward for why this is true
 *
 */
fn solve(mut nums: Vec<String>) {
    let mut invalidated: bool = false;

    for (i, pair) in nums.windows(2).enumerate() {
        let mut num1: Vec<char> = pair[0].chars().collect();
        let mut num2: Vec<char> = pair[1].chars().collect();

        /* Cannot invalidate the sorting if num of
           digits is not equal */
        if num1.len() != num2.len() {
            continue;
        }

        let leading_digit1 = num1[0].to_digit(10).unwrap();
        let leading_digit2 = num2[0].to_digit(10).unwrap();

        if num1.len() == 1 {
            if leading_digit1 == 0 && leading_digit2 == 9 {
                continue;
            } else if leading_digit1 == 0 {
                num1[0] = char::from_digit(leading_digit2 + 1, 10).unwrap()
            } else {
                num2[0] = char::from_digit(leading_digit1 - 1, 10).unwrap()
            }
        } else {
            if leading_digit1 == 1 && leading_digit2 == 9 {
                continue;
            } else if leading_digit1 == 1 {
                num1[0] = char::from_digit(leading_digit2 + 1, 10).unwrap()
            } else {
                num2[0] = char::from_digit(leading_digit1 - 1, 10).unwrap()
            }
        }

        nums[i]     = num1.iter().collect::<String>();
        nums[i+1]   = num2.iter().collect::<String>();
        invalidated = true;
        break;
    }

    if invalidated {
        dump_list(&nums)
    } else {
        println!("impossible")
    }
}


fn main() -> io::Result<()>{
    solve(parse());
    Ok(())
}