use sea_orm::DbErr;

use super::SqlManagerError;

impl From<DbErr> for SqlManagerError {
    fn from(value: DbErr) -> Self {
        Self::DbErr(value)
    }
}
