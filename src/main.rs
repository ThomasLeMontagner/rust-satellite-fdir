mod modules {
    pub mod anomaly;
    pub mod mode;
    pub mod severity;
    pub mod telemetry;
}

use modules::anomaly::Anomaly;
use modules::mode::SpacecraftMode;
use modules::severity::Severity;
use modules::telemetry::Telemetry;

use std::thread;
use std::time::Duration;

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
