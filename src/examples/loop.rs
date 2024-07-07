use cccb_display::{CccbDisplayImagePackage, CccbImageSender};
use image::io::Reader as ImageReader;

fn main() {
    println!("Loading...");

    let packed_images = vec![
        CccbDisplayImagePackage::new(
            ImageReader::open("assets/ccc.png")
                .unwrap()
                .decode()
                .unwrap(),
            |pix| pix[3] != 0,
            true,
        ),
        CccbDisplayImagePackage::new(
            ImageReader::open("assets/pesthorn.jpg")
                .unwrap()
                .decode()
                .unwrap(),
            |pix| !(pix[0] > 128 || pix[1] > 128 || pix[2] > 128),
            true,
        ),
    ];

    // save screenshots
    for (i, img_pkg) in packed_images.iter().enumerate() {
        img_pkg
            .to_luma8()
            .save(format!("./screenshots/loop-screenshot{}.png", i))
            .unwrap();
    }

    println!("Sending...");
    let mut sender = CccbImageSender::new_from_env();

    let mut toggle = true;
    loop {
        sender.send_package(if toggle {
            &packed_images[0]
        } else {
            &packed_images[1]
        });

        // 60 pps
        std::thread::sleep(std::time::Duration::from_secs_f32(1. / 60.));

        toggle = !toggle;
    }
}
