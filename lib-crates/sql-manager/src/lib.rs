pub mod structs;
pub mod error;
pub mod manager;

use crate::structs::entity::Entity as UserEntity;
use sea_orm::{ConnectionTrait as _, Database, DatabaseConnection, DbErr, Schema};

pub async fn init_db(db_path: &str) -> Result<DatabaseConnection, DbErr> {
    // SQLite连接字符串格式: "sqlite://path/to/db?mode=rwc"
    let db_url = format!("sqlite://{}?mode=rwc", db_path);

    // 建立数据库连接
    let conn = Database::connect(&db_url).await?;

    // 获取数据库后端类型
    let db = conn.get_database_backend();

    // 创建表 (如果不存在)
    let stmt = db.build(
        Schema::new(db)
            .create_table_from_entity(UserEntity)
            .if_not_exists(),
    );

    conn.execute(stmt).await?;

    Ok(conn)
}
