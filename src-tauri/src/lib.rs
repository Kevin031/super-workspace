use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    name: String,
    path: String,
    is_project: bool,
    has_readme: bool,
    description: Option<String>,
    version: Option<String>,
    children: Option<Vec<ProjectInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitInfo {
    branch: Option<String>,
    last_commit: Option<String>,
    last_commit_author: Option<String>,
    last_commit_time: Option<String>,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadmeContent {
    content: String,
    exists: bool,
}

#[tauri::command]
fn scan_directory(path: String) -> Result<Vec<ProjectInfo>, String> {
    let scan_path = Path::new(&path);
    if !scan_path.exists() {
        return Err(format!("路径不存在: {}", path));
    }

    let mut projects = Vec::new();

    let entries = fs::read_dir(scan_path)
        .map_err(|e| format!("无法读取目录: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("无法读取目录项: {}", e))?;
        let file_type = entry.file_type().map_err(|e| format!("无法获取文件类型: {}", e))?;

        // 只处理目录
        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let item_path = entry.path();

        // 检查是否是项目（有 package.json）
        let package_json_path = item_path.join("package.json");
        let is_project = package_json_path.exists();

        // 检查是否有 README
        let has_readme = check_readme(&item_path);

        let mut project_info = ProjectInfo {
            name: name.clone(),
            path: item_path.to_string_lossy().to_string(),
            is_project,
            has_readme,
            description: None,
            version: None,
            children: None,
        };

        // 如果是项目，尝试读取 package.json
        if is_project {
            if let Ok(package_content) = fs::read_to_string(&package_json_path) {
                if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&package_content) {
                    project_info.description = package_json
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    project_info.version = package_json
                        .get("version")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        } else {
            // 如果是分组，扫描子目录
            let mut children = Vec::new();
            if let Ok(sub_entries) = fs::read_dir(&item_path) {
                for sub_entry in sub_entries {
                    if let Ok(sub_entry) = sub_entry {
                        if let Ok(sub_file_type) = sub_entry.file_type() {
                            if !sub_file_type.is_dir() {
                                continue;
                            }
                        }

                        let sub_name = sub_entry.file_name().to_string_lossy().to_string();
                        let sub_path = sub_entry.path();
                        let sub_package_json = sub_path.join("package.json");
                        let sub_is_project = sub_package_json.exists();
                        let sub_has_readme = check_readme(&sub_path);

                        let mut sub_project = ProjectInfo {
                            name: sub_name,
                            path: sub_path.to_string_lossy().to_string(),
                            is_project: sub_is_project,
                            has_readme: sub_has_readme,
                            description: None,
                            version: None,
                            children: None,
                        };

                        if sub_is_project {
                            if let Ok(package_content) = fs::read_to_string(&sub_package_json) {
                                if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&package_content) {
                                    sub_project.description = package_json
                                        .get("description")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());

                                    sub_project.version = package_json
                                        .get("version")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                }
                            }
                        }

                        children.push(sub_project);
                    }
                }
            }
            project_info.children = Some(children);
        }

        projects.push(project_info);
    }

    Ok(projects)
}

fn check_readme(path: &Path) -> bool {
    let readme_names = ["README.md", "README", "readme.md", "readme"];
    for name in readme_names {
        if path.join(name).exists() {
            return true;
        }
    }
    false
}

#[tauri::command]
fn read_readme(path: String) -> Result<ReadmeContent, String> {
    let project_path = Path::new(&path);
    let readme_names = ["README.md", "README", "readme.md", "readme"];

    for name in readme_names {
        let readme_path = project_path.join(name);
        if readme_path.exists() {
            return Ok(ReadmeContent {
                content: fs::read_to_string(&readme_path)
                    .map_err(|e| format!("无法读取README: {}", e))?,
                exists: true,
            });
        }
    }

    Ok(ReadmeContent {
        content: String::from("未找到 README 文件"),
        exists: false,
    })
}

#[tauri::command]
fn get_git_info(path: String) -> Result<GitInfo, String> {
    let project_path = Path::new(&path);

    // 检查是否是 git 仓库
    if !project_path.join(".git").exists() {
        return Ok(GitInfo {
            branch: None,
            last_commit: None,
            last_commit_author: None,
            last_commit_time: None,
            status: "clean".to_string(),
        });
    }

    // 获取当前分支
    let branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(project_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });

    // 获取最后一次提交
    let last_commit = Command::new("git")
        .args(["log", "-1", "--pretty=format:%s"])
        .current_dir(project_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });

    // 获取最后一次提交作者
    let last_commit_author = Command::new("git")
        .args(["log", "-1", "--pretty=format:%an"])
        .current_dir(project_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });

    // 获取最后一次提交时间
    let last_commit_time = Command::new("git")
        .args(["log", "-1", "--pretty=format:%ci"])
        .current_dir(project_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });

    // 检查工作区状态
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_path)
        .output()
        .ok();

    let status = if let Some(output) = status_output {
        if output.status.success() && !output.stdout.is_empty() {
            "modified".to_string()
        } else {
            "clean".to_string()
        }
    } else {
        "clean".to_string()
    };

    Ok(GitInfo {
        branch,
        last_commit,
        last_commit_author,
        last_commit_time,
        status,
    })
}

#[tauri::command]
fn open_in_finder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件夹: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件夹: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件夹: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn open_in_vscode(path: String) -> Result<(), String> {
    Command::new("code")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("无法用VSCode打开: {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_default_projects_path() -> String {
    #[cfg(target_os = "macos")]
    {
        return format!("{}/Projects", std::env::var("HOME").unwrap_or_else(|_| "~".to_string()));
    }

    #[cfg(target_os = "windows")]
    {
        return format!("{}\\Projects", std::env::var("USERPROFILE").unwrap_or_else(|_| "~".to_string()));
    }

    #[cfg(target_os = "linux")]
    {
        return format!("{}/Projects", std::env::var("HOME").unwrap_or_else(|_| "~".to_string()));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_directory,
            read_readme,
            get_git_info,
            open_in_finder,
            open_in_vscode,
            get_default_projects_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
