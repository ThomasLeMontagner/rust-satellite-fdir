mod modules {
    pub mod actions;
    pub mod anomaly;
    pub mod mode;
    pub mod severity;
    pub mod telemetry;
}

use modules::actions::Action;
use modules::anomaly::Anomaly;
use modules::mode::SpacecraftMode;
use modules::severity::Severity;
use modules::telemetry::Telemetry;

use std::thread;
use std::time::Duration;

/// Tracks how long anomalies persist across telemetry cycles.
struct AnomalyTracker {
    low_battery_cycles: u32,
    high_temperature_cycles: u32,
    high_cpu_load_cycles: u32,
}

impl AnomalyTracker {
    /// Creates a new tracker with all counters set to zero.
    fn new() -> Self {
        Self {
            low_battery_cycles: 0,
            high_temperature_cycles: 0,
            high_cpu_load_cycles: 0,
        }
    }

    /// Updates counters based on the anomalies detected in the current cycle.
    fn update(&mut self, anomalies: &[Anomaly]) {
        for anomaly in anomalies {
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
        let actions = determine_actions(&anomalies);

        print_telemetry(step, &sample, &anomalies, &severity, &mode, &actions);
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
    actions: &[Action],
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
    println!("Recovery actions:");

    if actions.is_empty() {
        println!("None");
    } else {
        for action in actions {
            println!("- {}", action.label());
        }
    }
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

//  Determine the action to take base on anomalies
fn determine_actions(anomalies: &[Anomaly]) -> Vec<Action> {
    let mut actions = Vec::new();

    for anomaly in anomalies {
        match anomaly {
            Anomaly::LowBattery => actions.push(Action::ReducePower),
            Anomaly::HighCpuLoad => actions.push(Action::RestartSubsystem),
            Anomaly::HighTemperature => actions.push(Action::ActivateCooling),
        }
    }

    actions
}
