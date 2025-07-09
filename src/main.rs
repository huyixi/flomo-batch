use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use std::{fs, thread, time::Duration};

/// 批量发送内容到 Flomo 的 CLI 工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// 输入的 txt 文件路径
    #[arg(value_name = "FILE")]
    file: String,

    /// 仅预览内容，不发送
    #[arg(short = 'p', long = "preview")]
    dry_run: bool,
}

#[derive(Serialize)]
struct FlomoMessage<'a> {
    content: &'a str,
}

/// 判断是否已有标签（内容包含 #）
fn has_tag(line: &str) -> bool {
    line.contains('#')
}

fn main() {
    dotenv().ok();

    let webhook_url = env::var("FLOMO_WEBHOOK_URL").expect("请设置环境变量 FLOMO_WEBHOOK_URL");

    let args = Args::parse();

    let file_content =
        fs::read_to_string(&args.file).unwrap_or_else(|_| panic!("❌ 无法打开文件: {}", args.file));

    let mut lines = file_content.lines();

    // 第一行作为默认标签
    let default_tag = lines
        .next()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "#记录".to_string());

    if default_tag.is_empty() {
        panic!("❌ 默认标签为空，请确保文本第一行是标签，例如 #工作");
    }

    let client = Client::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 如果已有标签，直接发送原内容；否则添加默认标签
        let content = if has_tag(trimmed) {
            trimmed.to_string()
        } else {
            format!("{} {}", trimmed, default_tag)
        };

        if args.dry_run {
            println!("📝 [DRY RUN] {}", content);
        } else {
            let message = FlomoMessage { content: &content };
            let response = client.post(&webhook_url).json(&message).send();

            match response {
                Ok(r) if r.status().is_success() => println!("✅ 已发送: {}", content),
                Ok(r) => eprintln!("❌ 状态码: {}", r.status()),
                Err(e) => eprintln!("❌ 发送失败: {}", e),
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
