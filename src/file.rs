use anyhow::Result as AnyResult;
use std::fs;
use git2::Repository;

const BASE_PATH: &str = "data";
const RESULT_DIR: &str = "result";

pub fn create_path(name: String) -> AnyResult<String> {
    let path = format!("{}/{}/{}", BASE_PATH, name, RESULT_DIR);
    fs::create_dir_all(&path)?;
    fs::File::create(format!("{}/{}", path, ".gitkeep"))?;
    
    Ok(path)
}

pub fn init_git(path: String) -> AnyResult<Repository> {
    let repo = Repository::init(path)?;
    repo.index()?.add("")
}
