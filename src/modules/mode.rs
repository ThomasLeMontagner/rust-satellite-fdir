use crate::modules::severity::Severity;

// Operating modes of the satellite.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpacecraftMode {
    Nominal,
    Degraded,
    Safe,
}

// Mode decision logic for the spacecraft.
impl SpacecraftMode {
    // Determines the spacecraft mode from detected anomalies.
    pub fn from_severity(severity: &Severity) -> Self {
        match severity {
            Severity::Nominal => Self::Nominal,
            Severity::Warning => Self::Degraded,
            Severity::Critical => Self::Safe,
        }
    }

    // Returns the spacecraft mode label.
    pub fn label(&self) -> &'static str {
        match self {
            SpacecraftMode::Nominal => "Nominal",
            SpacecraftMode::Degraded => "Degraded",
            SpacecraftMode::Safe => "Safe",
        }
    }
}