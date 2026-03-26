use std::thread;
use std::time::Duration;

struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
}

enum HealthStatus {
    Nominal,
    Warning,
    Critical,
}

impl Telemetry {
    fn from_step(step: u32) -> Self {
        Self {
            battery_voltage: 12.4 - (step as f32 * 0.1),
            temperature_c: 42.0 + (step as f32 * 0.8),
            cpu_load_percent: 37.5 + (step as f32 * 2.5),
        }
    }
}


fn main() {
    let mut step = 0;

    loop {
        let sample = Telemetry::from_step(step);
        let status = evaluate_status(&sample);  // What does '&' mean?

        print_telemetry(step, &sample, &status);

        step += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn evaluate_status(sample: &Telemetry) -> HealthStatus {
    if sample.battery_voltage < 10.5 || sample.temperature_c > 70.0 {
        HealthStatus::Critical
    } else if sample.battery_voltage < 11.5
        || sample.temperature_c > 55.0
        || sample.cpu_load_percent > 85.0
    {
        HealthStatus::Warning
    } else {
        HealthStatus::Nominal
    }
}

fn print_telemetry(step: u32, sample: &Telemetry, status: &HealthStatus) {
    println!("Satellite telemetry sample {step}");
    println!("Battery voltage: {:.1} V", sample.battery_voltage);
    println!("Temperature: {:.1} °C", sample.temperature_c);
    println!("CPU load: {:.1} %", sample.cpu_load_percent);
    println!("Health status: {}", health_status_label(status));
    println!();
}

fn health_status_label(status: &HealthStatus) -> &'static str {
    match status {
        HealthStatus::Nominal => "Nominal",
        HealthStatus::Warning => "Warning",
        HealthStatus::Critical => "Critical",
    }
}