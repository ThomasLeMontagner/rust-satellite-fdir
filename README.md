# 🛰️ Satellite FDIR Simulation (Rust)

A lightweight Rust project simulating **Fault Detection, Isolation, and Recovery (FDIR)** logic for a spacecraft using synthetic telemetry data.

## Overview

This project models a simplified satellite system where telemetry is generated over time and analyzed to detect anomalies such as:

* Low battery voltage
* Overheating
* High CPU load

Based on detected anomalies, the spacecraft transitions between operational modes (e.g., Nominal → Safe Mode).

The goal is to explore **robust system design**, **clean architecture**, and **Rust best practices** in an aerospace context.

## Architecture

Core components:

* **Telemetry**
  Represents spacecraft state (battery, temperature, CPU load)

* **Anomaly**
  Enum describing detected faults

* **SpacecraftMode**
  Operational modes (Nominal, Degraded, Safe)

* **FDIR Logic**
  Health checks + decision-making based on telemetry

Example structure:

```rust
struct Telemetry { ... }

enum Anomaly { ... }

enum SpacecraftMode { ... }

impl Telemetry {
    fn from_step(step: u32) -> Self { ... }
    fn is_overheating(&self) -> bool { ... }
}
```

## Features

* Simulated telemetry generation
* Rule-based anomaly detection
* Mode transitions based on system health
* Modular design using Rust `impl` blocks
* Readable and testable code structure

## Why Rust?

* Memory safety without garbage collection
* Strong type system for reliability-critical systems
* Excellent performance for real-time applications
* Ideal for embedded and aerospace domains

## Design Principles

* Separation of concerns via multiple `impl` blocks
* Clear naming and small, focused functions
* One-line doc comments for public APIs (Rust idiomatic style) 
* Extensible toward trait-based FDIR systems

## Running the Project

```bash
cargo run
```

## Future Improvements

* Trait-based FDIR architecture (`FaultDetector`, `RecoveryStrategy`)
* More realistic telemetry models
* Logging and visualization
* Integration with real satellite datasets
* Async/event-driven processing

## Learning Goals

This project is intended to:

* Practice Rust fundamentals (`struct`, `enum`, `impl`)
* Explore system design for safety-critical software
* Bridge software engineering with aerospace applications
