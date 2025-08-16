use serde::Deserialize;

/// 对应 WebDAV 响应 XML 顶层的 `<D:multistatus>` 节点
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MultiStatus {
    /// `<D:response>` 节点列表，一个 response 表示一个资源（文件或目录）
    #[serde(rename = "response", default)]
    pub responses: Vec<Response>,
}

/// 对应单个 `<D:response>` 节点
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    /// `<D:href>`：资源路径（URL 编码）
    pub href: String,
    /// `<D:propstat>` 列表，包含资源的属性和状态
    #[serde(rename = "propstat", default)]
    pub propstats: Vec<PropStat>,
}

/// 对应 `<D:propstat>` 节点：一个属性集 + 对应的 HTTP 状态
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PropStat {
    /// `<D:prop>`：具体的资源属性
    pub prop: Prop,
    /// `<D:status>`：HTTP 状态（如 "HTTP/1.1 200 OK"）
    pub status: String,
}

/// 对应 `<D:prop>` 节点，列出资源的具体属性
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    /// `<lp1:resourcetype>`：资源类型（文件/目录）
    #[serde(rename = "resourcetype")]
    pub resource_type: Option<ResourceType>,
    /// `<lp1:getcontentlength>`：文件大小（目录则无此字段）
    #[serde(rename = "getcontentlength")]
    pub content_length: Option<u64>,
    /// `<lp1:getlastmodified>`：最后修改时间
    #[serde(rename = "getlastmodified")]
    pub last_modified: Option<String>,
    /// `<D:getcontenttype>`：MIME 类型（如 "text/plain"）
    #[serde(rename = "getcontenttype")]
    pub content_type: Option<String>,
}

/// 对应 `<D:resourcetype>` 节点
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceType {
    /// `<D:collection/>` 节点存在则表示是目录，否则是文件
    #[serde(rename = "collection")]
    pub is_collection: Option<EmptyElement>,
}

/// 空元素的占位结构，例如 `<D:collection/>`
#[derive(Debug, Deserialize)]
pub struct EmptyElement {}
