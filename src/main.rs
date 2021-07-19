use catteprinter::command::*;
use catteprinter::commands::*;
use catteprinter::printer::*;
use catteprinter::Result;
use image::io::Reader as ImageReader;

fn main2() -> Result<()> {
    let printer = find_printer().expect("could not find printer");
    printer.send(&FeedPaper::new(10))?;

    Ok(())
}

fn main() {
    match main2() {
        Ok(()) => {},
        Err(e) => eprintln!("error: {:?}", e),
    }
    //printer.send(&SetQuality::new(0x34));
    //printer.send(&SetMode::new(Mode::Image));

    ////for path in ["img0.png", "img1.png", "img2.png"] {
    //for path in ["img0.png"] {
    //    //for path in ["text.png", "text.png"] {

    //    let img = ImageReader::open(path).unwrap().decode().unwrap();
    //    let img = img.rotate180();
    //    let img = img.into_luma8();
    //    assert_eq!(img.dimensions().0, 384);

    //    let mut commands = vec![];
    //    for (i, row) in img.rows().enumerate() {
    //        commands.push(PrintLine::with_pixels(
    //            &row.map(|p| match p.0[0] {
    //                0 => true,
    //                255 => false,
    //                value => panic!("don't know what a {} is", value),
    //            })
    //            .collect::<Vec<_>>(),
    //        ));

    //        if i == 20 {
    //            break;
    //        }
    //    }
    //    printer.send_all(&commands);

    //    printer.send(&FeedPaper::new(20));
    //}
    //printer.send(&FeedPaper::new(80 - 20));
}
