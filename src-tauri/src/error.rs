use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TimeFlowError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    #[error("Invalid category path: {0}")]
    InvalidCategoryPath(String),

    #[error("Category is hidden: {0}")]
    CategoryHidden(String),

    #[error("Distribution strategy required for mergeable tasks")]
    DistributionStrategyRequired,

    #[error("No direct tasks available for time distribution")]
    NoDirectTasks,

    #[error("Invalid allocation: {0}")]
    InvalidAllocation(String),

    #[error("Timer already running")]
    TimerAlreadyRunning,

    #[error("No timer running")]
    NoTimerRunning,

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Import error: {0}")]
    ImportError(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("Daily time exceeds 24 hours")]
    DailyTimeExceeds24Hours,
}

impl From<std::io::Error> for TimeFlowError {
    fn from(err: std::io::Error) -> Self {
        TimeFlowError::StorageError(err.to_string())
    }
}

impl From<serde_yaml::Error> for TimeFlowError {
    fn from(err: serde_yaml::Error) -> Self {
        TimeFlowError::StorageError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, TimeFlowError>;

// Make TimeFlowError compatible with Tauri's error handling
impl serde::Serialize for TimeFlowError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
