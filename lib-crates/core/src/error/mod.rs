use sql_manager::error::SqlManagerError;

pub enum CoreError {
    SqlError(SqlManagerError),
}
