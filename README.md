This code is able to solve the n-queens-problem with 10K queens in less than 5s

# Build & run
`cargo run --release`


| 0 | 1 | 2 | 3 |
|:---:|:---:|:---:|:---:|
| [0;0] | [0;1] | [0;2] | [0;3] |
| [1;0] | [1;1] | [1;2] | [1;3] |
| [2;0] | [2;1] | [2;2] | [2;3] |
| [3;0] | [3;1] | [3;2] | [3;3] |

We need 3 vectors to keep in memory all the conflicts and speed up the execution:
- rows[n]
- forw_diags[2n-1]
- backw_diags[2n-1]


### What I learned

1. By default the rust test harness hides output from test execution to
keep results readable. Test output can be recovered (e.g., for debugging)
by passing `--nocapture` to the test binaries:

    cargo test -- --nocapture

2. How to implement `fmt::Display` trait in custom struct.
