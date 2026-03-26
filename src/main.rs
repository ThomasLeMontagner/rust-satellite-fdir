use std::thread;
use std::time::Duration;

struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
}

impl Telemetry {
    fn nominal() -> Self {
        Self {
            battery_voltage: 12.4,
            temperature_c: 42.0,
            cpu_load_percent: 37.5,
        }
    }
}

fn main() {
    loop {
        let sample = Telemetry::nominal();
        print_telemetry(&sample);

        thread::sleep(Duration::from_secs(1));
    }
}

fn print_telemetry(telemetry: &Telemetry) {
    println!("Satellite telemetry sample");
    println!("Battery voltage: {:.1} V", telemetry.battery_voltage);
    println!("Temperature: {:.1} °C", telemetry.temperature_c);
    println!("CPU load: {:.1} %", telemetry.cpu_load_percent);
    println!();
}