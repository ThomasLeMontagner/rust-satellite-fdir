use std::thread;
use std::time::Duration;

/// Represents a satellite telemetry packet.
struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
}

/// Possible health status of a satellite.
enum HealthStatus {
    Nominal,
    Warning,
    Critical,
}

/// Possible satellite anomalies.
enum Anomaly {
    LowBattery,
    HighTemperature,
    HighCpuLoad,
}

impl Telemetry {
    /// Creates telemetry from a simulation step.
    fn from_step(step: u32) -> Self {
        Self {
            battery_voltage: 12.4 - (step as f32 * 0.1),
            temperature_c: 42.0 + (step as f32 * 0.8),
            cpu_load_percent: 37.5 + (step as f32 * 2.5),
        }
    }
}

/// Health and diagnostic.
impl Telemetry {
    /// Is the satellite overheating?
    fn is_overheating(&self) -> bool {
        self.temperature_c > 55.0
    }

    /// Is the temperature critical?
    fn is_temperature_critical(&self) -> bool {
        self.temperature_c > 70.0
    }

    /// Is the CPU overloading?
    fn is_cpu_overloading(&self) -> bool {
        self.cpu_load_percent > 85.0
    }

    /// Is the battery low?
    fn is_battery_low(&self) -> bool {
        self.battery_voltage < 11.5
    }

    /// Is the battery level critical?
    fn is_battery_critical(&self) -> bool {
        self.battery_voltage < 10.5
    }
}

fn main() {
    let mut step = 0;

    loop {
        let sample = Telemetry::from_step(step);
        let anomalies = detect_anomalies(&sample);
        let status = evaluate_status(&sample);

        print_telemetry(step, &sample, &status, &anomalies);

        step += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

/// Return anomalies in a telemetry packet.
fn detect_anomalies(sample: &Telemetry) -> Vec<Anomaly> {
    let mut anomalies: Vec<Anomaly> = Vec::new();

    if sample.is_battery_low() {
        anomalies.push(Anomaly::LowBattery);
    }

    if sample.is_overheating() {
        anomalies.push(Anomaly::HighTemperature);
    }

    if sample.is_cpu_overloading() {
        anomalies.push(Anomaly::HighCpuLoad);
    }

    anomalies
}

/// Evaluate health status from a telemetry packet.
fn evaluate_status(sample: &Telemetry) -> HealthStatus {
    if sample.is_battery_critical() || sample.is_temperature_critical() {
        HealthStatus::Critical
    } else if sample.is_battery_low() || sample.is_overheating() || sample.is_cpu_overloading() {
        HealthStatus::Warning
    } else {
        HealthStatus::Nominal
    }
}

fn print_telemetry(step: u32, sample: &Telemetry, status: &HealthStatus, anomalies: &Vec<Anomaly>) {
    println!("Satellite telemetry sample {step}");
    println!("Battery voltage: {:.1} V", sample.battery_voltage);
    println!("Temperature: {:.1} °C", sample.temperature_c);
    println!("CPU load: {:.1} %", sample.cpu_load_percent);
    println!("Health status: {}", health_status_label(status));

    if anomalies.is_empty() {
        println!("Detected anomalies: none");
    } else {
        println!("Detected anomalies:");
        for anomaly in anomalies {
            println!("- {}", anomaly_label(anomaly));
        }
    }

    println!();
}

fn health_status_label(status: &HealthStatus) -> &'static str {
    match status {
        HealthStatus::Nominal => "Nominal",
        HealthStatus::Warning => "Warning",
        HealthStatus::Critical => "Critical",
    }
}

fn anomaly_label(anomaly: &Anomaly) -> &'static str {
    match anomaly {
        Anomaly::LowBattery => "Low battery",
        Anomaly::HighTemperature => "High temperature",
        Anomaly::HighCpuLoad => "High CPU load",
    }
}
