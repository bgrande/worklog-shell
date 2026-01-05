use anyhow::Result as AnyResult;
use std::fs;
use std::process::Command;
use fs::File;
use std::io::BufReader;

const BASE_PATH: &str = "data";
const RESULT_DIR: &str = "result";
const GIT_KEEP: &str = ".gitkeep";

fn get_repo_path(name: String) -> AnyResult<String> {
    Ok(format!("{}/{}", BASE_PATH, name))
}

fn create_path(base_repo_path: String) -> AnyResult<()> {

    let result_path = format!("{}/{}", base_repo_path, RESULT_DIR);
    let github_workflow_path = format!("{}/.github/workflow", base_repo_path);

    let from_workflow_path = format!("{}/github/workflow/ci.yml", BASE_PATH);

    fs::create_dir_all(&result_path)?;
    fs::create_dir_all(&github_workflow_path)?;

    let _ = fs::File::create(format!("{}/{}", result_path, GIT_KEEP))?;
    let _ = fs::copy(from_workflow_path, format!("{}/ci.yml", github_workflow_path));

    Ok(())
}

fn init_git(path: String) -> AnyResult<()> {
    let gitkeep_path = format!("{}/{}/{}", path, RESULT_DIR, GIT_KEEP);
    let workflow_path = format!("{}/.github/workflow/ci.yml", path);

    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {}", path))
        .arg("&& git init")
        .arg(format!("&& git add {} {}", gitkeep_path, workflow_path))
        .output()
        .expect("failed to execute process");

    // process(format!("cd {} && git init && git add {} {}", path, gitkeep_path, workflow_path));

    Ok(())
}

pub fn init_repository(name: String) -> AnyResult<()> {
    let path = get_repo_path(name)?;

    create_path(path.clone())?;
    init_git(path.clone())?;

    Ok(())
}

pub fn get_file_buffer(path: String) -> AnyResult<BufReader<File>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    Ok(reader)
}
