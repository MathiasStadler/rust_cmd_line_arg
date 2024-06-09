// FROM HERE
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
/*
export FILE_NAME=01_rust_handle_cli_args.rs
export FILE_DIR_NAME=examples/
echo "clippy prg => $(echo $FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => $(echo $FILE_NAME | cut -d . -f 1)";
cargo build --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "run PRG => $(echo $FILE_NAME | cut -d . -f 1)";
cargo run --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => $(echo $FILE_NAME | cut -d . -f 1)"
cargo test --example "$(echo $FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "$(echo $FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => $?"

// plain run
export FILE_NAME=01_rust_handle_cli_args.rs
export FILE_DIR_NAME=examples/
cargo run --example "$(echo $FILE_NAME | cut -d . -f 1)" arg1
*/
