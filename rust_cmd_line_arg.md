# explore rust handle args arguments

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="01_rust_handle_cli_args.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "clippy prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"

// plain run
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)" arg1
*/
EoF

```

## Saving the Argument Values in Variables

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="02_save_args_to_vars.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "clippy prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"

// example plain run
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)" -- test sample.txt

*/
EoF

```

## Detect expected arguments

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="03_detect_expected_arguments.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
//FROM HERE
// [What is the difference between len() and count()?](https://stackoverflow.com/questions/29500666/what-is-the-difference-between-len-and-count)
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Numbers of args(len) w/o file_name => {}",&args.len() -1);

    if *&args.len() < 2{
        println!("ERROR : To few arguments");
        process::exit(1);
    }

    let file_name = &args[0];
    let argument = &args[1];

    println!("execute path/file name {}", file_name);
    println!("argument {}", argument);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "clippy prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"

// plain run
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)" arg1

*/
EoF

```

## Multiple arguments

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="04_multiple_arguments.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
//FROM HERE
// [What is the difference between len() and count()?](https://stackoverflow.com/questions/29500666/what-is-the-difference-between-len-and-count)
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Numbers of args(len) w/o file_name => {}",&args.len() -1);

    if *&args.len() < 2{
        println!("ERROR : To few arguments");
        process::exit(1);
    }

    if *&args.len() > 2{
        println!("ERROR : To many arguments, we can only handle one argument");
        process::exit(1);
    }

    vec_loop_2(args);
    process::exit(0);

}
    // loop over vec
    // https://stackoverflow.com/questions/66092442/how-to-loop-over-a-vector-of-integers-in-rust



fn vec_loop_2(mut v: Vec<String>) -> Vec<String> {
    for i in v.iter_mut() {
        //*i *= 2;
        println!("arguments => {}",*i);
    }
    v
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "clippy prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"

// plain run
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)" arg1

*/
EoF

```

## template

```rust,no_run
#!/usr/bin/env bash
export EXAMPLE_SCRIPT_FILE="99_template.rs"
export EXAMPLE_SCRIPT_DIR="examples/"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
//FROM HERE
// <link of source>
pub fn main(){

    println!("template");
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
echo "clippy prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo clippy --fix --allow-dirty --allow-staged
echo "build prg => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo build --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "run PRG => \$(echo \$FILE_NAME | cut -d . -f 1)";
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "";
echo "run TEST => \$(echo \$FILE_NAME | cut -d . -f 1)"
cargo test --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
# cargo test --jobs 4 --example "\$(echo \$FILE_NAME | cut -d . -f 1)"
echo "ReturnCode => \$?"

// plain run
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
cargo run --example "\$(echo \$FILE_NAME | cut -d . -f 1)" arg1

*/
EoF

```
