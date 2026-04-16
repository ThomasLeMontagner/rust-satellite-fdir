use crate::modules::anomaly::Anomaly;

/// Tracks how long anomalies persist across telemetry cycles.
pub struct AnomalyTracker {
    low_battery_cycles: u32,
    high_temperature_cycles: u32,
    high_cpu_load_cycles: u32,
}

impl AnomalyTracker {
    /// Creates a new tracker with all counters set to zero.
    pub fn new() -> Self {
        Self {
            low_battery_cycles: 0,
            high_temperature_cycles: 0,
            high_cpu_load_cycles: 0,
        }
    }

    /// Updates counters based on the anomalies detected in the current cycle.
    pub fn update(&mut self, anomalies: &[Anomaly]) {
        self.low_battery_cycles = if anomalies.contains(&Anomaly::LowBattery) {
            self.low_battery_cycles + 1
        } else {
            0
        };

        self.high_temperature_cycles = if anomalies.contains(&Anomaly::HighTemperature) {
            self.high_temperature_cycles + 1
        } else {
            0
        };

        self.high_cpu_load_cycles = if anomalies.contains(&Anomaly::HighCpuLoad) {
            self.high_cpu_load_cycles + 1
        } else {
            0
        };
    }

    /// Returns the number of consecutive low-battery cycles.
    pub fn low_battery_cycles(&self) -> u32 {
        self.low_battery_cycles
    }

    /// Returns the number of consecutive high-temperature cycles.
    pub fn high_temperature_cycles(&self) -> u32 {
        self.high_temperature_cycles
    }

    /// Returns the number of consecutive high-CPU-load cycles.
    pub fn high_cpu_load_cycles(&self) -> u32 {
        self.high_cpu_load_cycles
    }
}