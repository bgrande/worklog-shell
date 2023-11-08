use anyhow::Result as AnyResult;
use std::fs;

const BASE_PATH: &str = "data";
const RESULT_DIR: &str = "result";
const GIT_KEEP: &str = ".gitkeep";

pub fn create_path(name: String) -> AnyResult<String> {
    let base_repo_path = format!("{}/{}", BASE_PATH, name);
    let result_path = format!("{}/{}", base_repo_path, RESULT_DIR);
    let github_workflow_path = format!("{}/.github/workflow", base_repo_path);

    let from_workflow_path = format!("{}/github/workflow/ci.yml", BASE_PATH);

    fs::create_dir_all(&result_path)?;
    fs::create_dir_all(&github_workflow_path)?;

    fs::File::create(format!("{}/{}", result_path, GIT_KEEP))?;
    fs::copy(from_workflow_path, format!("{}/ci.yml", github_workflow_path));

    Ok(base_repo_path)
}

pub fn init_git(path: String) -> AnyResult<String> {
    let gitkeep_path = format!("{}/{}/{}", path, RESULT_DIR, GIT_KEEP);
    let workflow_path = format!("{}/.github/workflow/ci.yml", path);

    std::process(format!("cd {} && git init && git add {} {}", path, gitkeep_path, workflow_path));

    Ok(path)
}
