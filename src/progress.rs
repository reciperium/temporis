use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error("Failed to emit progress {reason}")]
    EmitFailed { reason: String },
}

pub trait ProgressIntegration {
    fn set_progress(&mut self, progress: f64) -> Result<(), ProgressError>;

    fn emit(&self) -> Result<(), ProgressError>;

    fn stop(&mut self) -> Result<(), ProgressError>;
}
