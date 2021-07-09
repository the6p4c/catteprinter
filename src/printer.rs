use crate::command::Command;
use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{Characteristic, Peripheral, WriteType};
use uuid::Uuid;

const COMMAND_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xae01);

pub struct Printer<D: Peripheral> {
    device: D,
    command_characteristic: Characteristic,
}

impl<D: Peripheral> Printer<D> {
    pub fn new(device: D) -> Self {
        device.connect().unwrap();

        let characteristics = device.discover_characteristics().unwrap();
        let command_characteristic = characteristics
            .iter()
            .find(|c| c.uuid == COMMAND_CHARACTERISTIC_UUID)
            .unwrap();
        let command_characteristic = command_characteristic.clone();

        Printer {
            device,
            command_characteristic,
        }
    }

    pub fn send(&self, command: &Command) {
        self.send_bytes(&command.as_bytes());
    }

    pub fn send_all(&self, command: &[Command]) {
        let buf = command
            .iter()
            .map(Command::as_bytes)
            .flatten()
            .collect::<Vec<_>>();
        self.send_bytes(&buf);
    }

    fn send_bytes(&self, bytes: &[u8]) {
        // TODO: this is the MTU that's negotiated for my device - is this true
        // for all of them?
        const MTU: usize = 248;

        // 4 bytes required for L2CAP header
        for chunk in bytes.chunks(MTU - 4) {
            self.device
                .write(
                    &self.command_characteristic,
                    chunk,
                    WriteType::WithoutResponse,
                )
                .unwrap();
        }
    }
}
