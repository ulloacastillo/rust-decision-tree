# Rust decision tree

To exec

1) Leave your `.csv file` without header just data and labels in last column.
2) Modify `src/main.rs` line 18 (` File::open("./file.csv").expect("No se pudo abrir el arcivo");` and change "file" with your file name.
3) Compile with: `cargo rustc --release -- -C opt-level=3 -C target-cpu=native -C debuginfo=0`
4) On terminal run: `./target/release/decision-tree {samples} {features}`
5) Output: Time of fit method
