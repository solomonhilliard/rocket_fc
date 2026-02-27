# CC Rocket Flight Controller v0.1.0

## About
This is software for the Cascadia College engineering club's rocket project. It is designed to run on an ESP32C3.
Our control-station software can be found [here](https://github.com/wycontir/rocketviewer).

## Installation
See [this](https://docs.esp-rs.org/std-training/02_2_software.html) guide for installing dependencies, you may need to change build settings depending on your system.
Execute `cargo run` to compile and flash the software.

## Todo:
- [ ] servos with PID
- [ ] Once first stable version is complete, setup proper git branching + automatic builds
- [ ] Buzzer/LED instead of panicking on unrecoverable errors
- [ ] Barometer, servo control dependent on speed / pressure
- [ ] LoRa radio
- [ ] GPS
- [ ] waypoints
