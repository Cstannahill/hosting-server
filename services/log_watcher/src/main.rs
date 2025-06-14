use std::{env, path::PathBuf, time::Duration};
use anyhow::{Context, Result};
use regex::Regex;
use tokio::{fs::File, io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom}, time};

#[derive(Clone)]
struct Config {
    files: Vec<PathBuf>,
    levels: Vec<String>,
    patterns: Vec<Regex>,
    webhook: Option<String>,
    interval: Duration,
}

impl Config {
    fn from_env() -> Result<Self> {
        let files = env::var("LOG_FILES")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
            .collect();
        let levels = env::var("LOG_LEVELS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        let patterns = env::var("LOG_REGEX")
            .unwrap_or_default()
            .split(',')
            .filter_map(|s| Regex::new(s.trim()).ok())
            .collect();
        let webhook = env::var("WEBHOOK_URL").ok();
        let interval = env::var("POLL_INTERVAL_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_millis(1000));
        Ok(Self { files, levels, patterns, webhook, interval })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env().context("reading configuration")?;
    if config.files.is_empty() {
        eprintln!("No log files specified via LOG_FILES");
        return Ok(());
    }
    let mut handles = Vec::new();
    for path in config.files.clone() {
        let cfg = config.clone();
        handles.push(tokio::spawn(async move {
            if let Err(e) = watch_file(path, cfg).await {
                eprintln!("{}", e);
            }
        }));
    }
    futures::future::join_all(handles).await;
    Ok(())
}

async fn watch_file(path: PathBuf, cfg: Config) -> Result<()> {
    let mut file = File::open(&path)
        .await
        .with_context(|| format!("opening {}", path.display()))?;
    file.seek(SeekFrom::End(0)).await?;
    let mut reader = BufReader::new(file);
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).await?;
        if bytes == 0 {
            time::sleep(cfg.interval).await;
            continue;
        }
        if line_matches(&line, &cfg) {
            print!("{}: {}", path.display(), line);
            if let Some(url) = &cfg.webhook {
                let _ = send_webhook(url, &line).await;
            }
        }
        line.clear();
    }
}

fn line_matches(line: &str, cfg: &Config) -> bool {
    let level_match = if cfg.levels.is_empty() {
        true
    } else {
        cfg.levels.iter().any(|l| line.to_lowercase().contains(l))
    };
    let regex_match = if cfg.patterns.is_empty() {
        true
    } else {
        cfg.patterns.iter().any(|re| re.is_match(line))
    };
    level_match && regex_match
}
// async function
async fn send_webhook(url: &str, line: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({ "message": line });
    client.post(url).json(&body).send().await?;
    Ok(())
}
