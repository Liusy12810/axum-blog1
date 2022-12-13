//! # error.rs
//! > in this file we defein the error type of the application,
//! > which can help us to determine the error occured in the program.
//!
//! 我在这里定义了本程序所需要的错误类型，用于在未来开发过程中和使用过程中报告错误
//! 由于是联系程序的缘故，所有的错误类型都只实现了基本功能

/// # AppErrorType
/// > this enum helps us to determine the type of caused error
///
/// > impliments [`Debug`] trait for display
#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Template,
    NotFound,
    Duplicate,
}

/// # AppError
/// > this type indicates the possible error in this program,
/// > we have the constructor of the type.
///
/// > the [`Display`](std::fmt::Display) trait of the struct was implimented by trait [`Debug`]
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        AppError {
            message,
            cause,
            types,
        }
    }

    fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }

    fn from_str(msg: &str, types: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, types)
    }

    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::NotFound)
    }

    pub fn notfound_msg(msg: &str) -> Self {
        Self::notfound_opt(Some(msg.to_string()))
    }

    pub fn notfound() -> Self {
        Self::notfound_msg("没有找到符合条件的数据")
    }

    pub fn duplicate(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::Duplicate)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.message {
            Some(msg) => msg.clone(),
            None => "有错误发生".to_string(),
        };
        msg.into_response()
    }
}
