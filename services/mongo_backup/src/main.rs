use std::{env, fs::create_dir_all};
use chrono::Utc;
use tokio::{process::Command, signal};
use tokio_cron_scheduler::{JobScheduler, Job};
use anyhow::{Result, Context};

async fn backup_mongo(uri: &str, dir: &str) -> Result<()> {
    let ts = Utc::now().format("%Y%m%d%H%M%S");
    let path = format!("{}/{}_dump.gz", dir, ts);
    let status = Command::new("mongodump")
        .arg("--uri").arg(uri)
        .arg("--archive").arg(&path)
        .arg("--gzip")
        .status()
        .await
        .context("failed to spawn mongodump")?;
    if !status.success() {
        anyhow::bail!("mongodump exited with status {}", status);
    }
    println!("Mongo backup created at {}", path);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let schedule = env::var("BACKUP_SCHEDULE").unwrap_or_else(|_| "0 3 * * *".into());
    let backup_dir = env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups/mongo".into());
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://mongo:27017".into());
    create_dir_all(&backup_dir).context("create backup directory")?;

    let sched = JobScheduler::new().await?;
    let job = Job::new_async(schedule.as_str(), move |_, _| {
        let uri = mongo_uri.clone();
        let dir = backup_dir.clone();
        Box::pin(async move {
            if let Err(e) = backup_mongo(&uri, &dir).await {
                eprintln!("backup error: {e}");
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;
    signal::ctrl_c().await?;
    Ok(())
}
