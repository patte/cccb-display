use cccb_display::{CccbDisplayImagePackage, CccbImageSender};
use image::io::Reader as ImageReader;

fn main() {
    println!("Loading...");

    let img = ImageReader::open("assets/ccc.png")
        .unwrap()
        .decode()
        .unwrap();

    let img_packed = CccbDisplayImagePackage::new(img.clone(), |pix| pix[3] != 0, true);

    // save screenshot
    println!("Saving simple-screenshot.png...");
    img_packed
        .to_luma8()
        .save("./screenshots/simple-screenshot.png")
        .unwrap();

    println!("Sending...");
    let mut sender = CccbImageSender::new_from_env();
    sender.send_package(&img_packed);

    println!("Done!");
}
