# What I learned

1. By default the rust test harness hides output from test execution to
keep results readable. Test output can be recovered (e.g., for debugging)
by passing `--nocapture` to the test binaries:

    cargo test -- --nocapture

2. How to implement `fmt::Display` trait in custom struct.
