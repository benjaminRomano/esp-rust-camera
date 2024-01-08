use crate::wifi::wifi;
use anyhow::{bail, Result};
use esp_idf_hal::gpio::{IOPin, OutputPin};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use log::info;
use std::thread::sleep;
use std::time::Duration;

mod camera;
mod server;
mod wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("z")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        CONFIG.wifi_ssid,
        CONFIG.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };
    info!("{}", CONFIG.wifi_ssid);
    info!("Connected to Wifi...");
    let cam = camera::Camera::new(
        None,
        None,
        peripherals.pins.gpio21.into_ref(),
        peripherals.pins.gpio4.into_ref(),
        peripherals.pins.gpio5.into_ref(),
        peripherals.pins.gpio18.into_ref(),
        peripherals.pins.gpio19.into_ref(),
        peripherals.pins.gpio36.into_ref(),
        peripherals.pins.gpio39.into_ref(),
        peripherals.pins.gpio34.into_ref(),
        peripherals.pins.gpio35.into_ref(),
        peripherals.pins.gpio25.into_ref(),
        peripherals.pins.gpio23.into_ref(),
        peripherals.pins.gpio22.into_ref(),
        Some(peripherals.pins.gpio26.downgrade().into_ref()),
        Some(peripherals.pins.gpio27.downgrade().into_ref()),
    )?;

    let _server = server::serve(cam)?;
    info!("Serving...");

    // Ensure process doesn't end
    loop {
        sleep(Duration::from_secs(1));
    }
}
