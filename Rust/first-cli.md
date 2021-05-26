# 学習内容

[Command line apps in Rust](https://rust-cli.github.io/book/index.html#command-line-apps-in-rust)をやった

# 結果

```rust
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

```

# 感想

structoptめっちゃ使いやすい
