use zbus::{
    blocking::Connection,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

use crate::progress::{ProgressError, ProgressIntegration};

#[derive(Default, SerializeDict, DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "dict")]
pub struct UnityLauncher {
    progress: Option<f64>,
    #[zvariant(rename = "progress-visible")]
    progress_visible: Option<bool>,
}

impl ProgressIntegration for UnityLauncher {
    fn emit(&self) -> Result<(), ProgressError> {
        let conn = Connection::session().expect("Session should've created a connection");
        let signal_result = conn.emit_signal(
            Some("com.canonical.Unity"),
            "/com/reciperium/temporis",
            "com.canonical.Unity.LauncherEntry",
            "Update",
            &("application://com.reciperium.temporis.desktop", self),
        );
        signal_result.map_err(|err| ProgressError::EmitFailed {
            reason: err.to_string(),
        })
    }

    fn stop(&mut self) -> Result<(), ProgressError> {
        self.progress = Some(0.0);
        self.progress_visible = Some(false);
        Ok(())
    }

    fn set_progress(&mut self, progress: f64) -> Result<(), ProgressError> {
        self.progress = Some(progress);
        self.progress_visible = Some(true);
        Ok(())
    }
}
