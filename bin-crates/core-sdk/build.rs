use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // 监控自身变化
    println!("cargo:rerun-if-changed=always");   // 强制每次运行

    let is_release = env::var("PROFILE").unwrap() == "release";
    println!("[构建信息] 当前模式: {}", if is_release { "Release" } else { "Debug" });

    let mut base = std::env::current_dir().expect("无法获取当前目录");
    base.pop(); // 退出到 `bin-crates/test-client` 的父目录
    base.pop(); // 退出到 `sdk` 目录
    base.pop(); // 退出到 `quick-sync` 根目录

    println!("[构建信息] 项目根目录: {:?}", base);

    // 构建 diff 项目
    let diff_path = base.join("diff");
    println!("[构建信息] 开始构建 diff 项目: {:?}", diff_path);

    let mut diff_cmd = Command::new("cargo");
    diff_cmd.arg("build").current_dir(&diff_path);
    if is_release {
        diff_cmd.arg("--release");
    }

    let status = diff_cmd.status().expect("执行 diff 构建命令失败");
    assert!(status.success(), "构建 diff 项目失败");

    println!("[构建完成] diff 项目构建成功");

    // 构建 task-scheduler 项目
    let ts_path = base.join("task-scheduler");
    println!("[构建信息] 开始构建 task-scheduler 项目: {:?}", ts_path);

    let mut ts_cmd = Command::new("cargo");
    ts_cmd.arg("build").current_dir(&ts_path);
    if is_release {
        ts_cmd.arg("--release");
    }

    let status = ts_cmd.status().expect("执行 task-scheduler 构建命令失败");
    assert!(status.success(), "构建 task-scheduler 项目失败");

    println!("[构建完成] task-scheduler 项目构建成功");

    // 目标目录名称（debug/release）
    let target_dir = if is_release { "release" } else { "debug" };

    // 准备两个目标目录
    let libraries_dir = base.join("sdk").join("libraries");
    let output_target_dir = {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        path.pop(); path.pop(); path.pop(); // 退出到 target/debug/ 或 target/release/
        path
    };

    // 确保目录存在
    fs::create_dir_all(&libraries_dir).expect("无法创建 libraries 目录");
    fs::create_dir_all(&output_target_dir).expect("无法创建 target 目录");

    // 复制 diff.dll
    let diff_dll_name = if cfg!(target_os = "windows") {
        "diff.dll"
    } else if cfg!(target_os = "linux") {
        "libdiff.so"
    } else if cfg!(target_os = "macos") {
        "libdiff.dylib"
    } else {
        panic!("不支持的操作系统");
    };

    let diff_dll_src = base.join("diff").join("target").join(target_dir).join(diff_dll_name);
    copy_dll(&diff_dll_src, &libraries_dir, &output_target_dir, diff_dll_name);

    // 复制 task-scheduler.dll
    let ts_dll_name = if cfg!(target_os = "windows") {
        "task_scheduler.dll"
    } else if cfg!(target_os = "linux") {
        "libtask_scheduler.so"
    } else if cfg!(target_os = "macos") {
        "libtask_scheduler.dylib"
    } else {
        panic!("不支持的操作系统");
    };

    let ts_dll_src = base.join("task-scheduler").join("target").join(target_dir).join(ts_dll_name);
    copy_dll(&ts_dll_src, &libraries_dir, &output_target_dir, ts_dll_name);

    println!("[构建完成] 所有依赖项目构建完毕，DLL 已复制到 sdk/libraries/ 和 target/{}/", target_dir);
}

/// 复制 DLL 到两个目标目录
fn copy_dll(
    dll_src: &Path,
    libraries_dir: &Path,
    output_target_dir: &Path,
    dll_name: &str,
) {
    let dest_lib = libraries_dir.join(dll_name);
    let dest_target = output_target_dir.join(dll_name);

    if dll_src.exists() {
        fs::copy(&dll_src, &dest_lib).expect(&format!("复制 {} 到 libraries 失败", dll_name));
        fs::copy(&dll_src, &dest_target).expect(&format!("复制 {} 到 target 失败", dll_name));
        println!("[文件操作] 已复制 {} 到 {}", dll_name, dest_lib.display());
        println!("[文件操作] 已复制 {} 到 {}", dll_name, dest_target.display());
    } else {
        println!("[警告] 未找到 {}", dll_src.display());
    }
}