use env_config::static_env::{IS_DEBUG, PROJECT_ROOT_PATH};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

/// 检查 link-lib 目录是否存在
fn link_lib_exists(project_root_path: &str) -> bool {
    let link_lib_path = Path::new(project_root_path).join("link-lib");
    link_lib_path.exists() && link_lib_path.is_dir()
}

/// 获取 link-lib 目录下的所有第一层文件夹路径（动态库项目目录）
fn get_link_lib_folders(project_root_path: &str) -> Vec<PathBuf> {
    let project_root_path: PathBuf = PathBuf::from(project_root_path);
    let link_lib_path: PathBuf = project_root_path.join("link-lib");
    let mut link_lib_folders: Vec<PathBuf> = Vec::new();

    // 过滤掉.git或.github

    if link_lib_path.exists() && link_lib_path.is_dir() {
        let read_dir_iter =
            fs::read_dir(&link_lib_path).expect("无法读取 link-lib 目录");

        for entry_result in read_dir_iter {
            if let Ok(entry) = entry_result {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    let folder_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("<未知名称>");

                    if folder_name == ".git" || folder_name == ".github" {
                        continue;
                    }

                    println!(
                        "cargo:warning=动态链接库: {} -> {:?}",
                        folder_name, path
                    );
                    link_lib_folders.push(path);
                }
            }
        }

        println!(
            "cargo:warning=link-lib 目录下的文件夹: {:?}",
            link_lib_folders
        );
    } else {
        println!("cargo:warning=link-lib 目录不存在: {:?}", link_lib_path);
    }

    link_lib_folders
}

/// 构建指定路径下的动态链接库项目
fn build_link_lib(link_lib_path: &PathBuf) {
    let lib_name = link_lib_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("<未知名称>");

    println!(
        "cargo:warning=开始创建动态链接库: {} -> {:?}",
        lib_name, link_lib_path
    );

    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(link_lib_path.join("Cargo.toml"));

    if IS_DEBUG {
        command.arg("--verbose");
    } else {
        command.arg("--release");
    }

    let status = command.status().expect("无法执行 cargo build");

    if status.success() {
        println!("cargo:warning=✅ 构建成功: {}", lib_name);
    } else {
        println!("cargo:warning=❌ 构建失败: {}", lib_name);
    }
}

/// 字符串转换为 snake_case
fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    let mut prev_was_upper = false;

    for ch in s.chars() {
        if ch.is_uppercase() {
            if !snake.is_empty() && !prev_was_upper {
                snake.push('_');
            }
            for low in ch.to_lowercase() {
                snake.push(low);
            }
            prev_was_upper = true;
        } else if ch == '-' {
            snake.push('_');
            prev_was_upper = false;
        } else {
            snake.push(ch);
            prev_was_upper = false;
        }
    }

    snake
}

/// 获取平台相关的动态链接库文件名
fn get_platform_link_lib_name(link_lib_path: &PathBuf) -> String {
    let lib_name = link_lib_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("<未知名称>");

    let lib_name = to_snake_case(lib_name);

    if cfg!(target_os = "windows") {
        format!("{}.dll", lib_name)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", lib_name)
    } else {
        format!("lib{}.so", lib_name)
    }
}

/// 复制构建生成的动态链接库文件到指定目录
fn copy_link_lib_files(link_lib_path: &PathBuf, target_path: &PathBuf) {
    let file_name = get_platform_link_lib_name(link_lib_path);
    let target_file_path = target_path.join(&file_name);
    let link_lib_target_path = link_lib_path
        .join("target")
        .join(if IS_DEBUG { "debug" } else { "release" });

    let source_file_path = link_lib_target_path.join(&file_name);

    match fs::copy(&source_file_path, &target_file_path) {
        Ok(_) => {
            println!(
                "cargo:warning=✅ 成功复制动态链接库: {} -> {:?}",
                &file_name, &target_file_path
            );
        }
        Err(e) => {
            println!("cargo:warning=❌ 复制动态链接库失败: {} -> {:?}, 错误: {}", &file_name, &target_file_path, e);
        }
    }
}

/// 获取当前平台支持的动态库扩展名列表
fn get_dynamic_lib_extensions() -> Vec<&'static str> {
    if cfg!(target_os = "windows") {
        vec!["dll"]
    } else if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    }
}

/// 复制文件或目录到目标位置
fn copy_to_destination(
    source: &Path,
    target: &Path,
    options: &CopyOptions,
) {
    // 确保目标目录存在
    if let Err(e) = fs::create_dir_all(target) {
        println!(
            "cargo:warning=❌ 创建目录失败: {:?}, 错误: {}",
            target, e
        );
        return;
    }

    if source.is_dir() {
        copy_directory(source, target, options);
    } else if source.is_file() {
        // 如果目标是目录，则使用源文件名；否则直接使用目标路径
        let target_path = if target.is_dir() {
            target.join(source.file_name().unwrap_or_default())
        } else {
            target.to_path_buf()
        };
        copy_file(source, &target_path, options);
    } else {
        println!(
            "cargo:warning=❌ 源路径既不是文件也不是目录: {:?}",
            source
        );
    }
}

/// 复制单个文件
fn copy_file(source: &Path, target: &Path, options: &CopyOptions) {
    // 检查是否需要跳过复制
    if options.skip_existing && target.exists() {
        println!("cargo:warning=⏩ 跳过已存在文件: {:?}", target);
        return;
    }

    // 确保目标目录存在
    if let Some(parent) = target.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            println!(
                "cargo:warning=❌ 创建目录失败: {:?}, 错误: {}",
                parent, e
            );
            return;
        }
    }

    match fs::copy(source, target) {
        Ok(_) => {
            println!(
                "cargo:warning=✅ 已复制文件: {:?} -> {:?}",
                source, target
            );
        }
        Err(e) => {
            println!(
                "cargo:warning=❌ 复制失败: {:?} -> {:?}, 错误: {}",
                source, target, e
            );
        }
    }
}

/// 复制目录到目标位置
fn copy_directory(
    source_dir: &Path,
    target_dir: &Path,
    options: &CopyOptions,
) {
    let valid_extensions = if options.filter_by_extension {
        get_dynamic_lib_extensions()
    } else {
        Vec::new()
    };

    let walker = WalkDir::new(source_dir)
        .min_depth(1)
        .max_depth(if options.recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        let source_path = entry.path();

        // 跳过目录
        if source_path.is_dir() && !options.include_directories {
            continue;
        }

        // 处理文件
        if source_path.is_file() {
            // 检查文件扩展名过滤
            if options.filter_by_extension {
                if let Some(ext) =
                    source_path.extension().and_then(|e| e.to_str())
                {
                    if !valid_extensions
                        .iter()
                        .any(|e| ext.eq_ignore_ascii_case(e))
                    {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // 计算相对路径
            let relative_path =
                source_path.strip_prefix(source_dir).unwrap();
            let target_path = target_dir.join(relative_path);

            // 复制文件
            copy_file(source_path, &target_path, options);
        }
    }
}

/// 复制选项配置
struct CopyOptions {
    /// 是否包含子目录
    recursive: bool,
    /// 是否包含目录本身
    include_directories: bool,
    /// 是否按扩展名过滤
    filter_by_extension: bool,
    /// 是否跳过已存在的文件
    skip_existing: bool,
}

impl Default for CopyOptions {
    fn default() -> Self {
        CopyOptions {
            recursive: false,
            include_directories: false,
            filter_by_extension: true,
            skip_existing: false,
        }
    }
}

/// SDK 根目录路径
const SDK_PATH: &str = "sdk";

/// 动态链接库相对路径
const LINK_LIB_RELATIVE_PATH: &str = "link-lib";

/// libraries 相对路径
const LIBRARIES_RELATIVE_PATH: &str = "sdk/libraries";

/// always 文件路径（用于触发构建）
const ALWAYS_FILE: &str = "always";

fn main() {
    // 指定当 build.rs 文件或者 always 文件有改动时，重新运行 build 脚本
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", ALWAYS_FILE);
    println!("cargo:rerun-if-changed={}", LIBRARIES_RELATIVE_PATH);

    // 打印项目根目录路径
    println!("cargo:warning=项目根目录: {:?}", PROJECT_ROOT_PATH);

    // 构造 link-lib 路径
    let link_lib_path =
        Path::new(PROJECT_ROOT_PATH).join(LINK_LIB_RELATIVE_PATH);

    // 检查 link-lib 目录是否存在
    let has_link_lib = link_lib_path.exists() && link_lib_path.is_dir();

    if has_link_lib {
        // 获取 link-lib 目录下所有动态库项目文件夹路径
        let folders = get_link_lib_folders(PROJECT_ROOT_PATH);

        // 并行构建所有动态库项目
        folders.par_iter().for_each(|folder| {
            build_link_lib(folder);
        });

        // 构造 libraries 路径
        let libraries_dir =
            Path::new(PROJECT_ROOT_PATH).join(LIBRARIES_RELATIVE_PATH);

        // 并行复制所有构建成功的动态库文件到 libraries 目录
        folders.par_iter().for_each(|folder| {
            copy_link_lib_files(folder, &libraries_dir);
        });
    } else {
        println!("cargo:warning=link-lib 目录不存在，跳过构建过程");
    }

    // 构造最终构建输出目录路径
    let target_dir = Path::new(PROJECT_ROOT_PATH)
        .join(SDK_PATH)
        .join("target")
        .join(if IS_DEBUG { "debug" } else { "release" });

    // 构造 libraries 路径
    let libraries_dir =
        Path::new(PROJECT_ROOT_PATH).join(LIBRARIES_RELATIVE_PATH);

    // 确保 libraries 目录存在
    if !libraries_dir.exists() {
        println!("cargo:warning=创建 libraries 目录: {:?}", libraries_dir);
        if let Err(e) = fs::create_dir_all(&libraries_dir) {
            println!("cargo:warning=❌ 创建 libraries 目录失败: {}", e);
        }
    }

    // 使用新的复制函数
    let copy_options = CopyOptions {
        recursive: false,
        include_directories: false,
        filter_by_extension: true,
        skip_existing: false,
    };

    copy_to_destination(&libraries_dir, &target_dir, &copy_options);
}
