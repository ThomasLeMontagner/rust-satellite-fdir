use std::thread;
use std::time::Duration;

/// Represents a satellite telemetry packet.
struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
}

/// Possible satellite anomalies.
enum Anomaly {
    LowBattery,
    HighTemperature,
    HighCpuLoad,
}

/// Operating modes of the satellite.
enum SpacecraftMode {
    Nominal,
    Degraded,
    Safe,
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

/// Diagnostic helpers for telemetry.
impl Telemetry {
    /// Is the satellite overheating?
    fn is_overheating(&self) -> bool {
        self.temperature_c > 55.0
    }

    /// Is the CPU overloaded?
    fn is_cpu_overloaded(&self) -> bool {
        self.cpu_load_percent > 85.0
    }

    /// Is the battery low?
    fn is_battery_low(&self) -> bool {
        self.battery_voltage < 11.5
    }
}

fn main() {
    let mut step = 0;

    loop {
        let sample = Telemetry::from_step(step);
        let anomalies = detect_anomalies(&sample);
        let mode = determine_mode(&anomalies);

        print_telemetry(step, &sample, &anomalies, &mode);

        step += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

/// Returns anomalies in a telemetry packet.
fn detect_anomalies(sample: &Telemetry) -> Vec<Anomaly> {
    let mut anomalies: Vec<Anomaly> = Vec::new();

    if sample.is_battery_low() {
        anomalies.push(Anomaly::LowBattery);
    }

    if sample.is_overheating() {
        anomalies.push(Anomaly::HighTemperature);
    }

    if sample.is_cpu_overloaded() {
        anomalies.push(Anomaly::HighCpuLoad);
    }

    anomalies
}

/// Returns the mode of the satellite based on detected anomalies.
fn determine_mode(anomalies: &[Anomaly]) -> SpacecraftMode {
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
        SpacecraftMode::Safe
    } else if has_low_battery || has_high_temperature || has_high_cpu_load {
        SpacecraftMode::Degraded
    } else {
        SpacecraftMode::Nominal
    }
}

fn print_telemetry(step: u32, sample: &Telemetry, anomalies: &[Anomaly], mode: &SpacecraftMode) {
    println!("Satellite telemetry sample {step}");
    println!("Battery voltage: {:.1} V", sample.battery_voltage);
    println!("Temperature: {:.1} °C", sample.temperature_c);
    println!("CPU load: {:.1} %", sample.cpu_load_percent);
    println!("Spacecraft mode: {}", spacecraft_mode_label(mode));

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

/// Returns label corresponding to anomalies.
fn anomaly_label(anomaly: &Anomaly) -> &'static str {
    match anomaly {
        Anomaly::LowBattery => "Low battery",
        Anomaly::HighTemperature => "High temperature",
        Anomaly::HighCpuLoad => "High CPU load",
    }
}

/// Returns the label corresponding to the spacecraft mode.
fn spacecraft_mode_label(mode: &SpacecraftMode) -> &'static str {
    match mode {
        SpacecraftMode::Nominal => "Nominal",
        SpacecraftMode::Degraded => "Degraded",
        SpacecraftMode::Safe => "Safe",
    }
}
