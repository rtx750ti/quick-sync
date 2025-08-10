use sea_orm::DbErr;
mod impl_display;
mod impl_from;

#[derive(Debug)]
pub enum SqlManagerError {
    DbErr(DbErr),
}
