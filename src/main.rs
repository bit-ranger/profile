use async_recursion::async_recursion;
use log::info;
use std::path::{Path, PathBuf};
use tokio::fs::metadata;
use tokio::fs::read_dir;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    return replace_files_recur("/home/bitranger/.config/profile".into()).await;
}

#[async_recursion]
async fn replace_files_recur(config_path: PathBuf) -> std::io::Result<()> {
    let mut config_dir = read_dir(config_path).await?;
    loop {
        let sub_entry_op = config_dir.next_entry().await?;
        if sub_entry_op.is_none() {
            break;
        }
        let sub_entry = sub_entry_op.unwrap();

        if is_dir(sub_entry.path()).await {
            return replace_files_recur(sub_entry.path()).await;
        } else {
            info!("Replaced {}", sub_entry.ino());
        }
    }
    return Ok(());
}

pub async fn exists(path: impl AsRef<Path>) -> bool {
    metadata(path).await.is_ok()
}

pub async fn is_dir(path: impl AsRef<Path>) -> bool {
    metadata(path).await.map(|m| m.is_dir()).unwrap_or(false)
}
