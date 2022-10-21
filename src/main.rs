use async_recursion::async_recursion;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use tokio::fs::copy;
use tokio::fs::metadata;
use tokio::fs::read_dir;
use tokio::fs::remove_file;

#[derive(StructOpt)]
#[structopt(name = "chord")]
struct Profile {
    #[structopt(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opt = Profile::from_args();
    println!("Using {}", opt.name);
    let config_path = PathBuf::from(
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("profile")
            .join(opt.name),
    );
    return replace_files_recur(config_path).await;
}

#[async_recursion]
async fn replace_files_recur(config_path: PathBuf) -> std::io::Result<()> {
    if !exists(&config_path).await {
        return Ok(());
    }
    let mut config_dir = read_dir(config_path).await?;
    loop {
        let sub_entry_op = config_dir.next_entry().await?;
        if sub_entry_op.is_none() {
            break;
        }
        let sub_entry = sub_entry_op.unwrap();

        if is_dir(sub_entry.path()).await {
            replace_files_recur(sub_entry.path()).await?;
        } else {
            let replaced_path: PathBuf = sub_entry
                .path()
                .iter()
                .enumerate()
                .filter(|(i, _e)| i == &0 || i > &5)
                .map(|(_i, e)| e)
                .collect();
            if exists(&replaced_path).await {
                remove_file(&replaced_path).await?;
            }
            println!("Replace {}", replaced_path.to_str().unwrap());
            copy(sub_entry.path(), &replaced_path).await?;
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
