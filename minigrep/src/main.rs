use std::{env, process};

use minigrep::{run, Config};

// case shell
// 1. cargo run
// 2. cargo run to poem.txt
// 3. cargo run to poem.txt > output.txt
// 4. CASE_INSENSITIVE=1 cargo run to poem.txt > output.txt

// 多项目
// 1. cargo run -p [project_name]
// 2. cargo run -p [project_name] to ./[project_name]/poem.txt
// 3. cargo run -p [project_name] to ./[project_name]/poem.txt > ./[project_name]/output.txt
// 4. CASE_INSENSITIVE=1 cargo run -p [project_name] to ./[project_name]/poem.txt > ./[project_name]/output.txt
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
