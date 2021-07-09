use catteprinter::command::*;

use std::thread;
use std::time::Duration;
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
use btleplug::api::{bleuuid::uuid_from_u16, Central, Peripheral, WriteType, Characteristic, CharPropFlags};

fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Trace).init().unwrap();

    let manager = Manager::new().unwrap();

    let adapters = manager.adapters().unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan().unwrap();
    thread::sleep(Duration::from_secs(5));
    central.stop_scan().unwrap();

    let device = central.peripherals().into_iter().find(|p| p.properties().local_name.iter().any(|name| name == "GB02")).expect("no device found");
    device.connect().unwrap();

    let characteristics = device.discover_characteristics().unwrap();
    let command_characteristic = characteristics.iter().find(|c| c.uuid == uuid_from_u16(0xae01)).unwrap();

    //println!("write 1");
    //device.write(&command_characteristic, &catteprinter::command::Command::new(0xaf, &[0x10, 0x00]).as_bytes(), WriteType::WithoutResponse).unwrap();
    //println!("write 2");
    //device.write(&command_characteristic, &catteprinter::command::Command::new(0xa4, &[0x05]).as_bytes(), WriteType::WithoutResponse).unwrap();
    //println!("write 3");
    //device.write(&command_characteristic, &catteprinter::command::Command::new(0xbe, &[0x00]).as_bytes(), WriteType::WithoutResponse).unwrap();

    println!("write feed");
    let cmd = catteprinter::command::FeedPaper::new(1).as_bytes();
    device.write(&command_characteristic, &cmd, WriteType::WithoutResponse).unwrap();

    println!("write line");
    let cmd = catteprinter::command::WriteLine::new(&[0xaa; 48]).as_bytes();
    device.write(&command_characteristic, &cmd, WriteType::WithoutResponse).unwrap();

    println!("write feed");
    let cmd = catteprinter::command::FeedPaper::new(100).as_bytes();
    device.write(&command_characteristic, &cmd, WriteType::WithoutResponse).unwrap();
}
