mod config;

use crate::config::CONTROL_LOOP_FREQUENCY;
use bno080::wrapper::BNO080;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::i2c::{APBTickType, I2cConfig, I2cDriver};
use log::{error, info};
use std::time::{Duration, Instant};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("initializing...");

    // setup imu
    let peripherals =
        esp_idf_svc::hal::peripherals::Peripherals::take().expect("take ownership of peripherals");

    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;
    let config = I2cConfig::new()
        .baudrate(400_000.into())
        .timeout(APBTickType::from(Duration::from_millis(10)));
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).expect("init i2c");

    let iface = bno080::interface::I2cInterface::default(i2c);

    let mut imu_driver = BNO080::new_with_interface(iface);
    imu_driver
        .init(&mut delay::Ets)
        .expect("initialize IMU driver");

    imu_driver
        .enable_rotation_vector(1_000 / CONTROL_LOOP_FREQUENCY)
        .expect("configure IMU");

    let delay = Delay::new_default();

    info!("Entering stabilization loop");

    loop {
        let start = Instant::now() + Duration::from_millis(1_000 / CONTROL_LOOP_FREQUENCY as u64);
        imu_driver.handle_all_messages(&mut delay::Ets, 1u8);

        match imu_driver.rotation_quaternion() {
            Ok(quat) => {
                let x = quat[0];
                let y = quat[1];
                let z = quat[2];
                let w = quat[3];
                println!("{{\"x\":{x:?},\"y\":{y:?},\"z\":{z:?},\"w\":{w:?},\"time\":0}}")
            },
            Err(e) => {
                error!("Failed to read quaternion from IMU: {e:?}")
            },
        }

        delay.delay_us(start.saturating_duration_since(Instant::now()).as_micros() as u32);
    }
}
