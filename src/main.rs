use std::thread;
use std::time::Duration;

struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
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
        print_telemetry(step, &sample);

        step += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn print_telemetry(step: u32, telemetry: &Telemetry) {
    println!("Satellite telemetry sample {step}");
    println!("Battery voltage: {:.1} V", telemetry.battery_voltage);
    println!("Temperature: {:.1} °C", telemetry.temperature_c);
    println!("CPU load: {:.1} %", telemetry.cpu_load_percent);
    println!();
}