use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use std::process::{Command, Stdio};
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "custom_cli_tool", about = "Hosting platform helper CLI")] 
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and start all services
    Deploy,
    /// Tail logs for a service or all services
    Logs { service: Option<String> },
    /// Trigger a manual database backup
    Backup,
    /// Show stack status from healthcheck dashboard
    Status,
    /// Get or set entries in the .env file
    Env {
        action: String,
        key: Option<String>,
        value: Option<String>,
    },
}

fn run_command(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().context("failed to spawn command")?;
    if !status.success() {
        anyhow::bail!("command exited with status {:?}", status.code());
    }
    Ok(())
}

fn deploy() -> Result<()> {
    println!("{}", "Deploying stack...".green());
    run_command(Command::new("docker-compose").args(["up", "-d", "--build"]))
}

fn logs(service: Option<String>) -> Result<()> {
    let mut args = vec!["logs".to_string(), "-f".to_string()];
    if let Some(s) = service {
        args.push(s);
    }
    run_command(Command::new("docker-compose").args(&args))
}

fn backup() -> Result<()> {
    let file = PathBuf::from("backups").join("manual_backup.sql");
    fs::create_dir_all("backups").ok();
    println!("{} {:?}", "Creating backup at".green(), file);
    let output = Command::new("docker-compose")
        .args(["exec", "-T", "db", "pg_dump", "-U", "postgres"])
        .output()
        .context("pg_dump failed")?;
    fs::write(&file, output.stdout)?;
    Ok(())
}

async fn status() -> Result<()> {
    let url = std::env::var("STATUS_URL").unwrap_or_else(|_| "http://localhost:9100/health".into());
    let resp = reqwest::get(&url).await.context("request failed")?;
    let text = resp.text().await?;
    println!("{}", text);
    Ok(())
}

fn env_get(key: &str) -> Result<()> {
    let data = fs::read_to_string(".env").unwrap_or_default();
    for line in data.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
            println!("{}", rest);
            return Ok(());
        }
    }
    println!("Key not found");
    Ok(())
}

fn env_set(key: &str, value: &str) -> Result<()> {
    let mut lines: Vec<String> = Vec::new();
    let mut found = false;
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if line.starts_with(&format!("{}=", key)) {
                lines.push(format!("{}={}", key, value));
                found = true;
            } else {
                lines.push(line.to_string());
            }
        }
    }
    if !found {
        lines.push(format!("{}={}", key, value));
    }
    fs::write(".env", lines.join("\n"))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Deploy => deploy()?,
        Commands::Logs { service } => logs(service)?,
        Commands::Backup => backup()?,
        Commands::Status => status().await?,
        Commands::Env { action, key, value } => {
            match action.as_str() {
                "get" => {
                    if let Some(k) = key { env_get(&k)?; }
                    else { println!("Please specify key"); }
                },
                "set" => {
                    if let (Some(k), Some(v)) = (key, value) { env_set(&k, &v)?; }
                    else { println!("Please specify key and value"); }
                },
                _ => println!("Unknown env action"),
            }
        }
    }
    Ok(())
}
