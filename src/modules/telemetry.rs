// Represents a satellite telemetry packet.
#[derive(Debug)]
pub struct Telemetry {
    pub battery_voltage: f32,
    pub temperature_c: f32,
    pub cpu_load_percent: f32,
}

impl Telemetry {
    // Creates telemetry from a simulation step.
    pub fn from_step(step: u32) -> Self {
        Self {
            battery_voltage: 12.4 - (step as f32 * 0.1),
            temperature_c: 42.0 + (step as f32 * 0.8),
            cpu_load_percent: 37.5 + (step as f32 * 2.5),
        }
    }
}

// Diagnostic helpers for telemetry.
impl Telemetry {
    // Is the satellite overheating?
    pub fn is_overheating(&self) -> bool {
        self.temperature_c > 55.0
    }

    // Is the CPU overloaded?
    pub fn is_cpu_overloaded(&self) -> bool {
        self.cpu_load_percent > 85.0
    }

    // Is the battery low?
    pub fn is_battery_low(&self) -> bool {
        self.battery_voltage < 11.5
    }
}
