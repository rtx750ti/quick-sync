use std::fmt::Display;

use super::SqlManagerError;

impl Display for SqlManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlManagerError::DbErr(db_err) => {
                write!(f, "{}", db_err.to_string())
            }
        }
    }
}
