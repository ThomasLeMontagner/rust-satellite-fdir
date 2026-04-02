mod modules{
  pub mod telemetry;
}

use modules::telemetry::Telemetry;

use std::thread;
use std::time::Duration;

// Possible satellite anomalies.
enum Anomaly {
    LowBattery,
    HighTemperature,
    HighCpuLoad,
}

// Operating modes of the satellite.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SpacecraftMode {
    Nominal,
    Degraded,
    Safe,
}

// Severity of teh anomalies
enum Severity {
    Nominal,
    Warning,
    Critical,
}

// Display helpers for severity.
impl Severity {
    // Returns the severity label.
    fn label(&self) -> &'static str {
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
    fn from_anomalies(anomalies: &[Anomaly]) -> Self {
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

// Mode decision logic for the spacecraft.
impl SpacecraftMode {
    // Determines the spacecraft mode from detected anomalies.
    fn from_severity(severity: &Severity) -> Self {
        match severity {
            Severity::Nominal => Self::Nominal,
            Severity::Warning => Self::Degraded,
            Severity::Critical => Self::Safe,
        }
    }

    // Returns the spacecraft mode label.
    fn label(&self) -> &'static str {
        match self {
            SpacecraftMode::Nominal => "Nominal",
            SpacecraftMode::Degraded => "Degraded",
            SpacecraftMode::Safe => "Safe",
        }
    }
}

// Display helpers for anomalies.
impl Anomaly {
    // Returns the anomaly label.
    fn label(&self) -> &'static str {
        match self {
            Anomaly::LowBattery => "Low battery",
            Anomaly::HighTemperature => "High temperature",
            Anomaly::HighCpuLoad => "High CPU load",
        }
    }
}

fn main() {
    let mut step = 0;
    let mut previous_mode = SpacecraftMode::Nominal;

    loop {
        let sample = Telemetry::from_step(step);
        let anomalies = detect_anomalies(&sample);
        let severity = Severity::from_anomalies(&anomalies);
        let mode = SpacecraftMode::from_severity(&severity);

        print_telemetry(step, &sample, &anomalies, &severity, &mode);
        print_mode_transition(&previous_mode, &mode);

        previous_mode = mode;

        step += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

// Returns anomalies in a telemetry packet.
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

fn print_telemetry(
    step: u32,
    sample: &Telemetry,
    anomalies: &[Anomaly],
    severity: &Severity,
    mode: &SpacecraftMode,
) {
    println!("Satellite telemetry sample {step}");
    println!("Battery voltage: {:.1} V", sample.battery_voltage);
    println!("Temperature: {:.1} °C", sample.temperature_c);
    println!("CPU load: {:.1} %", sample.cpu_load_percent);
    println!("Severity: {}", severity.label());
    println!("Spacecraft mode: {}", mode.label());

    if anomalies.is_empty() {
        println!("Detected anomalies: none");
    } else {
        println!("Detected anomalies:");
        for anomaly in anomalies {
            println!("- {}", anomaly.label());
        }
    }

    println!();
}

// Prints the spacecraft transition mode, if any.
fn print_mode_transition(previous_mode: &SpacecraftMode, current_mode: &SpacecraftMode) {
    if previous_mode == current_mode {
        println!("Mode transition: none")
    } else {
        println!(
            "Mode transition: {} -> {}",
            previous_mode.label(),
            current_mode.label()
        );
    }
}
