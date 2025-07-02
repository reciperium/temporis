use crate::progress::ProgressIntegration;

mod progress;

pub fn get_progress_integration() -> impl ProgressIntegration {
    progress::UnityLauncher::default()
}
