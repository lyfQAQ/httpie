use clap::{Args, Parser, Subcommand};

/// A naive httpie implementation with Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

// 子命令对应不同的 http 方法
#[derive(Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Args, Debug)]
struct Get {
    /// http 请求的 url
    url: String,
}

// post 子命令需要输入一个 URL，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Args, Debug)]
struct Post {
    /// http 请求的 url
    url: String,
    /// http 请求的 body
    body: Vec<String>,
}
fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
