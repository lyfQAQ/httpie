use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use reqwest::Url;
use std::str::FromStr;

/// A naive httpie implementation with Rust
#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
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
    #[arg(value_parser = parse_url)]
    url: String,
}

// post 子命令需要输入一个 URL，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Args, Debug)]
struct Post {
    /// http 请求的 url
    #[arg(value_parser = parse_url)]
    url: String,
    /// http 请求的 body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

/// 通过 parse_kv_pair 将key=value 解析成 KvPair 结构
#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

/// 实现 FromStr trait 的类,可以用 parse 方法解析自己
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 将 Some(T)/None 转换为 Ok(T)/Err(E)
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 自定义解析函数
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
