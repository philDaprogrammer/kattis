use std::io::{self, BufRead, Stdin};
use std::collections::{LinkedList};

struct TestCase {
    code: String,
    list: LinkedList<String>,
    is_reversed: bool
}

/**
 * implement 'TestCase' struct to print the list to the desired format
 */
impl TestCase {
   pub fn dump_list(&mut self) {
       if self.list.is_empty() {
           println!("[]");
           return
       }

       let mut to_dump: String = String::with_capacity((self.list.len() * 2) + 2);
       to_dump.push('[');

       while !self.list.is_empty() {
           to_dump.push_str(self.get_list_elem().as_str());
           to_dump.push(',');
       }

       to_dump.pop(); // trailing comma
       to_dump.push(']');

       println!("{}", to_dump);
    }

    fn get_list_elem(&mut self) -> String {
        if self.is_reversed {
            self.list.pop_back().unwrap()
        } else {
            self.list.pop_front().unwrap()
        }
    }
}

/**
 * return the input as a vector of test cases
 */
fn parse() -> Vec<TestCase> {
    let stdin: Stdin = io::stdin();

    // read in the lines
    let mut lines: LinkedList<String> = stdin.lock().lines()
        .map(|l| l.unwrap()).collect::<LinkedList<String>>();

    let num_cases: usize = lines.pop_front().unwrap().parse::<usize>().unwrap();
    // we're better off pre-allocating
    let mut cases: Vec<TestCase> = Vec::with_capacity(num_cases);

    let mut code: String;
    let mut list: LinkedList<String>;
    let mut line: String;
    let is_reversed = false;

    while !lines.is_empty() {
        code = lines.pop_front().unwrap();
        // list size is redundant
        lines.pop_front().unwrap();

        line = lines.pop_front().unwrap();
        line = line[1..line.len()-1].to_string();

        list = if line.is_empty() {
            LinkedList::new()
        } else {
            line.split(",").map(|i| i.to_string())
                .collect::<LinkedList<String>>()
        };

        cases.push(TestCase{code, list, is_reversed})
    }

    cases
}

/**
 * solve the given programs in theta(length(p) * cases)
 *
 * The basic idea is to have a 'pointer' point to either
 * the head or the tail of the list based on the most previous
 * 'R' function. If the state is reversed, then we pop from the
 * back of the list. Otherwise, we pop from the front. We then
 * print the resulting list after all functions are processed.
 *
 */
fn solve(cases: Vec<TestCase>) {
    'outer: for mut case in cases {
        for inst in case.code.chars() {

            if inst == 'R' {
                case.is_reversed = !case.is_reversed;
            } else if inst == 'D' {
                let elem: Option<String> =
                    if case.is_reversed { case.list.pop_back() } else { case.list.pop_front() };

                if elem == None {
                   println!("error");
                   continue 'outer;
                }
            }
        }

        case.dump_list();
    }
}

fn main() {
    solve(parse())
}