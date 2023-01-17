use std::io::{self, BufRead, Stdin};
use std::collections::{VecDeque};

#[derive(Clone)]
struct Point {
    r: i32,
    c: i32
}

#[derive(Clone)]
struct Pair {
    p1: Point,
    p2: Point
}

#[derive(Clone)]
struct Input {
    rows: usize,
    columns: usize,
    graph: Vec<Vec<char>>,
    queries: Vec<Pair>
}

#[derive(Clone)]
struct Node {
    kind: char,
    component: i32
}

/* adjacent cells to look at */
const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/**
 * Parse the input and store it in the 'Input' struct
 */
fn parse() -> Input {
    let stdin: Stdin = io::stdin();

    let rows: usize;
    let columns: usize;
    let mut graph: Vec<Vec<char>>;
    let mut queries: Vec<Pair>;

    /* read in all lines from stdin */
    let mut lines: VecDeque<String> = stdin.lock().lines()
        .map(|r| r.unwrap()).collect();

    let mut nums: Vec<i32> = lines.pop_front().unwrap().split( " ")
        .map(|r| r.trim().parse().unwrap()).collect();

    rows    = nums[0] as usize;
    columns = nums[1] as usize;

    graph = vec![vec!['0'; columns]; rows];

    /* read in the graph */
    for i in 0..rows {
        for (j, c) in lines.pop_front().unwrap().chars().enumerate() {
            graph[i][j] = c;
        }
    }

    let num_queries: usize = lines.pop_front().unwrap().parse().unwrap();
    queries = vec![Pair {p1: Point{r: 0, c: 0}, p2: Point{r: 0, c: 0}};  num_queries];

    /* read in the number of queries */
    for i in 0..num_queries {
        nums = lines.pop_front().unwrap().split(" ")
            .map(| r| r.trim().parse().unwrap()).collect();

        queries[i] = Pair{p1: Point {r: nums[0], c: nums[1]}, p2: Point {r: nums[2], c: nums[3]}};
    }

    Input{rows, columns, graph, queries}
}

/**
 * helper function to see if a nodes indices are within matrix bounds
 */
fn inbounds(length: usize, width: usize, r: i32, c: i32) -> bool {
    r >= 0 && (r as usize) < length && c >= 0 && (c as usize) < width
}

/**
 * Compute all queries in theta(n + m + q).
 *
 * where,
 *  n = c * r
 *  m = c * r * 4
 * and,
 *  q = # of queries
 *
 * The general idea is to compute all connected components of
 * the input graph, and assign each component a unique ID.
 * We can then perform a quick lookup to see if two nodes are
 * in the same connected component.
 *
 * The graph is stored as a matrix, so we iterate over all (i,j) pairs
 * and run bsf on any undiscovered node. Since we only perform the call
 * on undiscovered nodes, the runtime relation is.
 *
 * T(n, m) = theta(n) + theta(m)
 *         = theta(n + m)
 *
 * Its rather straight forward to see why this is the case.
 *
 * For each newly discovered node we assign it the unique
 * ID of the current traversal, and place the node in
 * a matrix, keeping track of each nodes component ID.
 *
 * For each query we simply perform a constant time look up
 * and see if both nodes are in the same component.
 * This is linear with respect to the number of queries.
 *
 * Therefore,
 * T(n, m, q) = theta(n + m) + theta(q)
 *            = theta(n + m + q)
 *
 * This is a drastic improvement over the more naive O((n + m) * q) solution
 *
 * Quick proof of correctness:
 *
 * bsf is correct, and will traverse all nodes
 * within a connected component. Since we assign
 * a unique ID to every component the algorithm must
 * be right (yep, that's really it ...)
 */
fn solve(input: Input) {
    /* discovered and connected components matrix's */
    let mut discovered: Vec<Vec<i8>> = vec![vec![0; input.columns]; input.rows];
    let mut cc: Vec<Vec<Node>>       = vec![vec![Node{kind: '\0', component: -1}; input.columns]; input.rows];

    /* ID's for each groups connected components */
    let mut cc_zero_id: i32 = 0;
    let mut cc_one_id: i32  = 0;

    for i in 0..input.rows {
        for j in 0..input.columns {

            /* not yet discovered, run bsf from this point */
            if discovered[i][j] == 0 {
                let area_type: char  = input.graph[i][j];
                let mut point: Point = Point{r: i as i32, c: j as i32};

                /* get the current connected component ID */
                let group: i32 = if area_type == '0' { cc_zero_id } else { cc_one_id };
                /* update component ID's */
                if area_type == '0' { cc_zero_id += 1 } else { cc_one_id += 1 }
                /* bsf queue */
                let mut queue: VecDeque<Point> = VecDeque::new();

                /* update the current node */
                cc[i][j]          = Node{ kind: area_type, component: group };
                discovered[i][j]  = 1;

                queue.push_back(point);

                /* run bsf */
                while !queue.is_empty() {
                    point = queue.pop_front().unwrap();

                    for n in NEIGHBORS {
                        let new_point    = Point{r: point.r + n.0, c: point.c + n.1};
                        let new_r: usize = new_point.r as usize;
                        let new_c: usize = new_point.c as usize;

                        /* new node to add */
                        if inbounds(input.graph.len(), input.graph[0].len(), new_point.r, new_point.c)
                            && discovered[new_r][new_c]  == 0
                            && input.graph[new_r][new_c] == area_type {

                            discovered[new_r][new_c] = 1;
                            cc[new_r][new_c] = Node{kind: area_type, component: group};
                            queue.push_back(new_point)
                        }
                    }
                }
            }
        }
    }

    /* answer each query in constant time. theta(q) overall */
    for query in input.queries {
        let start: Node = cc[(query.p1.r - 1) as usize][(query.p1.c - 1) as usize].clone();
        let end: Node   = cc[(query.p2.r - 1) as usize][(query.p2.c - 1) as usize].clone();

        /* we are in the same component */
        if start.component == end.component && start.kind == end.kind {
            println!("{}", if start.kind == '0' { "binary" } else { "decimal" } )
        } else {
            println!("neither")
        }
    }
}


pub fn main() -> io::Result<()> {
    solve(parse());
    Ok(())
}