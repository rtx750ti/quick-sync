use std::path::PathBuf;

use crate::{
    error::SqlManagerError, structs::entity::Entity as UserEntity,
};
use sea_orm::{
    ConnectionTrait as _, Database, DatabaseConnection, DbErr, Schema,
};

pub struct SqlManager {
    pub db_path: PathBuf,
    pub db: DatabaseConnection,
}

impl SqlManager {
    pub(self) async fn init_db(
        db_path: &PathBuf,
    ) -> Result<DatabaseConnection, DbErr> {
        let db_url =
            format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

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

    pub async fn new(
        db_path: &PathBuf,
    ) -> Result<SqlManager, SqlManagerError> {
        let db = Self::init_db(db_path).await?;

        Ok(SqlManager { db_path: db_path.to_owned(), db })
    }
}
