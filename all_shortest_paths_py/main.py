max_int = 1 << 31


"""
Had a really hard time getting a solution in rust to pass, 
so we did this one in python 

The solution is a direct application of the floyd-warshall 
algorithm. Here is a wikipedia article that does a fairly 
good job of explaining the algorithm: . The reason it is so
efficient for this problem is that algorithm runs in theta(n ^ 3) 
time and the problem guarantees that n <= 100. We can then answer 
all queries in constant time by performing table lookups. 

This gives a overall runtime of theta(n ^ 3 + q)
"""
def solve():
    case = 0

    while True:
        (n, m, q) = map(int, input().split(" "))

        if n == 0:
            break

        opts = [[max_int] * n for _ in range(n)]

        if case > 0:
            print()

        case += 1

        for i in range(n):
            opts[i][i] = 0

        # - nodes with edges to themselves cost 0
        for _ in range(m):
            (u, v, w)  = map(int, input().split(" "))
            opts[u][v] = min(opts[u][v], w)

        # - construct shortest paths table
        for k in range(n):
            for i in range(n):
                for j in range(n):
                    if opts[i][k] != max_int and opts[k][j] != max_int:
                        opts[i][j] = min(opts[i][j], opts[i][k] + opts[k][j])

        # - label any paths that are arbitrarily short with negative max_int
        for k in range(n):
            for i in range(n):
                for j in range(n):
                    if opts[k][k] < 0 and opts[i][k] != max_int and opts[k][j] != max_int:
                        opts[i][j] = -max_int

        # - answer all queries in constant time
        for _ in range(q):
            (src, dest) = map(int, input().split(" "))

            if opts[src][dest] == max_int:
                print("Impossible")
            elif opts[src][dest] == -max_int:
                print("-Infinity")
            else:
                print(opts[src][dest])

                
def main():
    solve()
    return


if __name__ == "__main__":
    main()
