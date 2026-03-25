struct Telemetry {
    battery_voltage: f32,
    temperature_c: f32,
    cpu_load_percent: f32,
}

fn main() {
    let sample = Telemetry {
        battery_voltage: 12.4,
        temperature_c: 42.0,
        cpu_load_percent: 37.5,
    };

    println!("Satellite telemetry sample");
    println!("Battery voltage: {} V", sample.battery_voltage);
    println!("Temperature: {} °C", sample.temperature_c);
    println!("CPU load: {} %", sample.cpu_load_percent);
}