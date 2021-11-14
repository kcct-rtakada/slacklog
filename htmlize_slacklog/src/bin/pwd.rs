use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("starting dir: {}", path.display());
    // path.display() -> D:\code\sai_slack_log\htmlize_slacklog

    // println!("starting dir: {:?}", path);

    let root = Path::new("/");
    assert!(env::set_current_dir(&root).is_ok());

    let path = env::current_dir()?;
    println!("final dir: {}", path.display());

    Ok(())
}