#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> TResult<T> {
    pub fn new(success: bool, data: Option<T>, message: String) -> TResult<T> {
        TResult {
            success,
            data,
            message,
        }
    }
}
