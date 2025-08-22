use crate::client::WebDavClient;
use crate::client::error::WebDavClientError;
use crate::client::traits::url_trait::UrlParse;
use percent_encoding::percent_decode_str;
use reqwest::Url;
use std::path::{Path, PathBuf};

impl UrlParse for WebDavClient {
    fn check_start(
        &self,
        path: &str,
    ) -> Result<String, WebDavClientError> {
        // 1. URL 解码
        let mut path = percent_encoding::percent_decode_str(path)
            .decode_utf8_lossy()
            .to_string()
            .trim()
            .to_string();

        if path.is_empty() {
            return Err(WebDavClientError::ParseUrlErr(
                "路径为空".to_string(),
            ));
        }

        // 2. 获取 base_url 的路径部分（已解码）
        let base_path_decoded =
            percent_encoding::percent_decode_str(self.base_url.path())
                .decode_utf8_lossy()
                .to_string();

        // 3. 如果传入路径以 base_url 路径开头，裁掉它
        if path.starts_with(&base_path_decoded) {
            path = path[base_path_decoded.len()..].to_string();
        }

        // 4. 如果还以 `/` 开头，转成相对路径
        if path.starts_with('/') {
            path = format!(".{}", path);
        }

        // 5. 禁止回溯
        if path.starts_with("../") {
            return Err(WebDavClientError::ParseUrlErr(
                "禁止返回上一级".to_string(),
            ));
        }
        if path.contains("..") {
            return Err(WebDavClientError::ParseUrlErr(
                "路径不能出现'..'".to_string(),
            ));
        }

        Ok(path)
    }

    fn try_parse_url(&self, path: &str) -> Result<Url, WebDavClientError> {
        let url = self.base_url.to_owned();
        let url = url.join(path).map_err(|err| {
            WebDavClientError::ParseUrlErr(err.to_string())
        })?;
        Ok(url)
    }

    fn check_end(&self, path: &str) -> Result<String, WebDavClientError> {
        let path = path.trim().to_string();

        if path.eq("/") || path.eq("./") {
            return Ok(path);
        }

        // 去掉末尾 / 再判断文件类型
        let trimmed_path = path.trim_end_matches('/');
        let last_segment =
            trimmed_path.rsplit('/').next().ok_or_else(|| {
                WebDavClientError::ParseUrlErr("路径格式错误".to_string())
            })?;

        let is_file = last_segment.contains('.');

        // 如果是文件但原路径以 / 结尾，报错
        if is_file && path.ends_with('/') {
            return Err(WebDavClientError::ParseUrlErr(format!(
                "'{}'不能以 '/' 结尾",
                path
            )));
        }

        // 跨平台最大兼容非法字符（文件夹和文件都检查）
        let invalid_chars =
            ['\\', '/', ':', '*', '?', '"', '<', '>', '|', '\0'];
        if last_segment.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(WebDavClientError::ParseUrlErr(format!(
                "'{}'包含非法字符",
                path
            )));
        }

        Ok(path)
    }

    fn normalize_path(&self, path: &Path) -> PathBuf {
        let mut result = PathBuf::new();
        for comp in path.components() {
            match comp {
                std::path::Component::CurDir => {} // 忽略当前目录符号 `.`
                std::path::Component::ParentDir => {
                    result.pop(); // 遇到 `..` 回退一级
                }
                std::path::Component::RootDir => {} // 忽略根目录符号 `/`
                other => result.push(other), // 其他正常路径段直接加入
            }
        }
        result
    }

    fn is_subpath(&self, base: &Path, target: &Path) -> bool {
        let base_norm = self.normalize_path(base);
        let target_norm = self.normalize_path(target);
        target_norm.starts_with(&base_norm)
    }

    fn decode_url_path(&self, p: &str) -> String {
        percent_decode_str(p).decode_utf8_lossy().to_string()
    }

    fn format_url_path(
        &self,
        path: &str,
    ) -> Result<String, WebDavClientError> {
        // 1. 检查路径开头是否合法（禁止空路径、禁止 `..` 回溯等）
        let path = self.check_start(path)?;

        // 2. 基于 base_url 拼接成完整 URL（reqwest::Url 会自动处理 ./、多余斜杠等）
        let path_url_entity = self.try_parse_url(&path)?;

        // 3. 检查路径结尾是否合法（文件不能以 `/` 结尾、末段不能含非法字符等）
        self.check_end(&path)?;

        // 4. 校验协议和主机是否与 base_url 一致，防止跨域访问
        let same_scheme =
            path_url_entity.scheme() == self.base_url.scheme();
        let same_host =
            path_url_entity.host_str() == self.base_url.host_str();
        if !same_scheme || !same_host {
            return Err(WebDavClientError::ParseUrlErr(
                "主机不一致".to_string(),
            ));
        }

        // 5. 对 base_url 和目标 URL 的路径部分进行 URL 解码
        //    这样可以避免 `%XX` 编码大小写不一致导致的比较失败
        let base_path_buf =
            PathBuf::from(self.decode_url_path(self.base_url.path()));
        let target_path_buf =
            PathBuf::from(self.decode_url_path(path_url_entity.path()));

        // 6. 检查目标路径是否在 base 路径之下（防止目录穿越）
        if !self.is_subpath(&base_path_buf, &target_path_buf) {
            return Err(WebDavClientError::ParseUrlErr(format!(
                "路径不是基础路径的子路径: base={}, target={}",
                base_path_buf.to_string_lossy().to_string(),
                target_path_buf.to_string_lossy().to_string()
            )));
        }

        Ok(path_url_entity.to_string())
    }
}
