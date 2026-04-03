use crate::modules::anomaly::Anomaly;

// Severity of teh anomalies
pub enum Severity {
    Nominal,
    Warning,
    Critical,
}

// Display helpers for severity.
impl Severity {
    // Returns the severity label.
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Nominal => "Nominal",
            Severity::Warning => "Warning",
            Severity::Critical => "Critical",
        }
    }
}

// Severity decision logic for detected anomalies.
impl Severity {
    // Determines the severity from detected anomalies.
    pub fn from_anomalies(anomalies: &[Anomaly]) -> Self {
        let has_low_battery = anomalies
            .iter()
            .any(|anomaly| matches!(anomaly, Anomaly::LowBattery));
        let has_high_temperature = anomalies
            .iter()
            .any(|anomaly| matches!(anomaly, Anomaly::HighTemperature));
        let has_high_cpu_load = anomalies
            .iter()
            .any(|anomaly| matches!(anomaly, Anomaly::HighCpuLoad));

        if has_low_battery && has_high_temperature {
            Self::Critical
        } else if has_low_battery || has_high_temperature || has_high_cpu_load {
            Self::Warning
        } else {
            Self::Nominal
        }
    }
}