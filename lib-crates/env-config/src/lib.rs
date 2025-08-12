pub mod error;
pub mod static_env;

use std::path::PathBuf;

use crate::error::EnvConfigError;

pub async fn get_db_path() -> Result<PathBuf, EnvConfigError> {
    #[cfg(debug_assertions)]
    {
        use crate::static_env::DB_NAME;

        let mut path = std::env::current_exe()?;
        path.pop();
        path.push(DB_NAME);

        Ok(path)
    }

    #[cfg(not(debug_assertions))]
    {
        use crate::static_env::{APP_DIR, ROOT_DIR};

        if let Some(proj_dirs) =
            directories::ProjectDirs::from("com", ROOT_DIR, APP_DIR)
        {
            use crate::static_env::DB_NAME;

            let data_dir = proj_dirs.data_dir();
            tokio::fs::create_dir_all(data_dir).await?;

            let mut path = proj_dirs.data_dir().to_path_buf();
            path.push(DB_NAME);
            return Ok(path);
        }

        // 如果无法获取推荐路径，返回错误
        Err(EnvConfigError::String("找不到数据库路径".to_owned()))
    }
}

pub fn get_root_path() -> Result<PathBuf, EnvConfigError> {
    let mut root_path: PathBuf = std::env::current_exe().map_err(|e| {
        EnvConfigError::String(format!(
            "获取当前可执行文件路径失败: {}",
            e
        ))
    })?;

    root_path.pop();

    Ok(root_path)
}
