use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context, Result};
use chrono::Utc;
use tokio_cron_scheduler::{JobScheduler, Job};
use tokio::process::Command;
use flate2::write::GzEncoder;
use flate2::Compression;

fn build_backup_path(dir: &str, prefix: &str, ext: &str) -> PathBuf {
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%S");
    let filename = format!("{}_{}.{}", prefix, timestamp, ext);
    PathBuf::from(dir).join(filename)
}

async fn backup_postgres(dir: &str) -> Result<()> {
    let output_path = build_backup_path(dir, "pg_backup", "sql");
    let file = File::create(&output_path).context("create pg dump file")?;
    let mut cmd = Command::new("pg_dump");
    cmd.stdout(Stdio::from(file));
    let status = cmd.status().await.context("running pg_dump")?;
    if !status.success() {
        anyhow::bail!("pg_dump exited with status {:?}", status.code());
    }
    // compress with gzip
    let gz_path = output_path.with_extension("sql.gz");
    let mut input = File::open(&output_path)?;
    let mut gz = GzEncoder::new(File::create(&gz_path)?, Compression::default());
    std::io::copy(&mut input, &mut gz)?;
    gz.finish()?;
    std::fs::remove_file(output_path)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let schedule = env::var("BACKUP_SCHEDULE").unwrap_or_else(|_| "0 0 2 * * *".into());
    let backup_dir = env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".into());
    std::fs::create_dir_all(&backup_dir).context("create backup directory")?;
    let sched = JobScheduler::new().await?;
    let job = Job::new_async(schedule.as_str(), move |_, _| {
        let dir = backup_dir.clone();
        Box::pin(async move {
            if let Err(e) = backup_postgres(&dir).await {
                eprintln!("backup failed: {}", e);
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;
    tokio::signal::ctrl_c().await?;
    Ok(())
}
