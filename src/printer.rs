use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::command::Command;
use crate::{Error, Result};
use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{Central, CentralEvent, Characteristic, Peripheral, WriteType};
#[cfg(target_os = "linux")]
use btleplug::bluez::manager::Manager;
#[cfg(target_os = "windows")]
use btleplug::winrtble::manager::Manager;
use uuid::Uuid;

// The printer advertises that it supports AF30, but when asked to give its GATT table it instead
// says AE30. BlueZ appears to (sometimes?) provide that latter option in scan results.
pub const COMMAND_SERVICE_UUID_ADVERTISED: Uuid = uuid_from_u16(0xaf30);
pub const COMMAND_SERVICE_UUID_GATT: Uuid = uuid_from_u16(0xae30);
pub const COMMAND_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xae01);

pub fn find_printer() -> Result<Printer<impl Peripheral>> {
    let manager = Manager::new().unwrap();
    let adapters = manager.adapters().unwrap();
    let central = adapters.into_iter().next().unwrap();

    let central_recv = central.event_receiver().unwrap();
    central.start_scan()?;

    let (addr_send, addr_recv) = mpsc::channel();
    thread::spawn(move || {
        loop {
            match central_recv.recv() {
                Ok(CentralEvent::ServicesAdvertisement {
                    address, services, ..
                }) => {
                    if services.contains(&COMMAND_SERVICE_UUID_ADVERTISED)
                        || services.contains(&COMMAND_SERVICE_UUID_GATT)
                    {
                        addr_send
                            .send(address)
                            .expect("could not send address to main thread");
                    }
                }
                Ok(_) => {}
                // TODO: um
                Err(_) => {}
            }
        }
    });

    let address = addr_recv
        .recv_timeout(Duration::from_secs(10))
        .map_err(|_| Error::PrinterNotFound)?;

    central.stop_scan()?;

    let device = central
        .peripherals()
        .into_iter()
        .find(|p| p.address() == address)
        .ok_or(Error::PrinterNotFound)?;

    Printer::new(device)
}

pub struct Printer<D: Peripheral> {
    device: D,
    command_characteristic: Characteristic,
}

impl<D: Peripheral> Printer<D> {
    pub fn new(device: D) -> Result<Self> {
        device.connect()?;

        let characteristics = device.discover_characteristics()?;
        let command_characteristic = characteristics
            .iter()
            .find(|c| c.uuid == COMMAND_CHARACTERISTIC_UUID)
            .ok_or(Error::PrinterNotFound)?;
        let command_characteristic = command_characteristic.clone();

        Ok(Printer {
            device,
            command_characteristic,
        })
    }

    pub fn send(&self, command: &Command) -> Result<()> {
        self.send_bytes(&command.as_bytes())
    }

    pub fn send_all(&self, command: &[Command]) -> Result<()> {
        let buf = command
            .iter()
            .map(Command::as_bytes)
            .flatten()
            .collect::<Vec<_>>();
        self.send_bytes(&buf)
    }

    fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
        // TODO: this is the MTU that's negotiated for my device - is this true
        // for all of them?
        const MTU: usize = 248;

        // 4 bytes required for L2CAP header
        for chunk in bytes.chunks(MTU - 4) {
            self.device.write(
                &self.command_characteristic,
                chunk,
                WriteType::WithoutResponse,
            )?;
        }

        Ok(())
    }
}
