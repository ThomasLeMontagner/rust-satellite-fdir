// Possible satellite anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anomaly {
    LowBattery,
    HighTemperature,
    HighCpuLoad,
}

// Display helpers for anomalies.
impl Anomaly {
    // Returns the anomaly label.
    pub fn label(&self) -> &'static str {
        match self {
            Anomaly::LowBattery => "Low battery",
            Anomaly::HighTemperature => "High temperature",
            Anomaly::HighCpuLoad => "High CPU load",
        }
    }
}