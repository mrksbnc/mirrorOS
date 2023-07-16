#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> CommandResult<T> {
    pub fn new(success: bool, data: Option<T>, message: String) -> CommandResult<T> {
        CommandResult {
            success,
            data,
            message,
        }
    }
}
