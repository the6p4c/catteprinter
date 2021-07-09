use btleplug::api::{Central, Peripheral};
#[cfg(target_os = "linux")]
use btleplug::bluez::manager::Manager;
#[cfg(target_os = "windows")]
use btleplug::winrtble::manager::Manager;
use catteprinter::command::*;
use catteprinter::commands::*;
use catteprinter::printer::*;
use image::io::Reader as ImageReader;
use std::thread;
use std::time::Duration;

fn main() {
    let manager = Manager::new().unwrap();
    let adapters = manager.adapters().unwrap();
    let central = adapters.into_iter().next().unwrap();

    central.start_scan().unwrap();
    thread::sleep(Duration::from_secs(5));
    central.stop_scan().unwrap();

    let device = central
        .peripherals()
        .into_iter()
        .find(|p| p.properties().local_name.iter().any(|name| name == "GB02"))
        .expect("no device found");

    let printer = Printer::new(device);

    printer.send(&SetQuality::new(0x34));
    printer.send(&SetMode::new(Mode::Image));

    //for path in ["img0.png", "img1.png", "img2.png"] {
    for path in ["img0.png"] {
        //for path in ["text.png", "text.png"] {

        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let img = img.rotate180();
        let img = img.into_luma8();
        assert_eq!(img.dimensions().0, 384);

        let mut commands = vec![];
        for (i, row) in img.rows().enumerate() {
            commands.push(PrintLine::with_pixels(
                &row.map(|p| match p.0[0] {
                    0 => true,
                    255 => false,
                    value => panic!("don't know what a {} is", value),
                })
                .collect::<Vec<_>>(),
            ));

            if i == 20 {
                break;
            }
        }
        printer.send_all(&commands);

        printer.send(&FeedPaper::new(20));
    }
    printer.send(&FeedPaper::new(80 - 20));
}
