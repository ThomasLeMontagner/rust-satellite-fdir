/// Recovery actions taken by the spacecraft.
#[derive(Debug)]
pub enum Action {
    ReducePower,
    ActivateCooling,
    RestartSubsystem,
}

impl Action {
    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::ReducePower => "Reduce power consumption",
            Self::ActivateCooling => "Activate cooling system",
            Self::RestartSubsystem => "Restart subsystem",
        }
    }
}