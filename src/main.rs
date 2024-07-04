use dotenv::dotenv;
use image::{io::Reader as ImageReader, GenericImageView};
use std::net;

fn main() {
    println!("Let's go!");
    dotenv().ok();

    let ip = std::env::var("IP_ADDRESS").expect("IP_ADDRESS not set");
    let port = 2342;

    println!("initializing host...");
    let socket = net::UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket");

    let width: u8 = 56; // in tiles (*8 for pixels)
    let height: u8 = 160; // lines (/8 for pixels)

    let mut img1 = ImageReader::open("assets/ccc.png")
        .unwrap()
        .decode()
        .unwrap();
    img1 = img1.resize(
        width as u32 * 8,
        height as u32,
        image::imageops::FilterType::Nearest,
    );
    let mut img2 = ImageReader::open("assets/pesthorn.jpg")
        .unwrap()
        .decode()
        .unwrap();
    img2 = img2.resize(
        width as u32 * 8,
        height as u32,
        image::imageops::FilterType::Nearest,
    );

    let mut toggle = false;
    loop {
        let img_name = if toggle { "ccc.png" } else { "pesthorn.jpg" };
        let img = if toggle { &img1 } else { &img2 };
        toggle = !toggle;

        // let img = ImageReader::open(img_name).unwrap().decode().unwrap();
        // //let img = ImageReader::open("pesthorn.jpg").unwrap().decode().unwrap();
        // let img = img.resize(
        //     width as u32 * 8,
        //     height as u32,
        //     image::imageops::FilterType::Nearest,
        // );
        //let img_dimensions = img.dimensions();
        //println!("img dimensions: {:?}", img_dimensions);
        //println!("img: {:?}", img.pixels());

        let mut packed_bytes: Vec<u8> = vec![0; 10 + width as usize * height as usize];

        packed_bytes[0] = 0;
        packed_bytes[1] = 19;
        packed_bytes[2] = 0;
        packed_bytes[3] = 0;
        packed_bytes[4] = 0;
        packed_bytes[5] = 0;
        packed_bytes[6] = 0;
        packed_bytes[7] = width;
        packed_bytes[8] = 0;
        packed_bytes[9] = height;

        let offset_x = ((width as u32 * 8) - img.dimensions().0) / 2;
        //println!("offset_x: {} {}", offset_x, offset_x / 8);

        for y in 0..height as u32 {
            for x in 0..width as u32 {
                let mut current_byte: u8 = 0;
                if x * 8 < offset_x {
                    continue;
                }
                for j in 0..8 {
                    let img_x = x * 8 + j - offset_x as u32;
                    if img_x < img.dimensions().0 {
                        let pix = img.get_pixel(img_x, y as u32);
                        current_byte = current_byte << 1;
                        if img_name == "ccc.png" {
                            if pix[3] != 0 {
                                current_byte = current_byte | 1;
                            }
                        } else {
                            if pix[0] <= 128 {
                                current_byte = current_byte | 1;
                            }
                        }
                    }
                }

                packed_bytes[(y * width as u32 + x) as usize + 10] = current_byte;
            }
        }

        /*
        let mut nextPixel: u8 = 0;
        for (i, pix) in img.into_rgba8().pixels().enumerate() {
            //packed_bytes[i + 10] = pix[3];
            //if pix[3] != 0 {
            //    packed_bytes[i + 10] = 255;
            //}
            nextPixel += pix[3];
            if i % 8 == 0 {
                packed_bytes[i / 8 + 10] = nextPixel;
                nextPixel = 0;
            }
        }
        */

        //println!("sending packet");
        socket
            .send_to(&packed_bytes, (ip.clone(), port))
            .expect("failed to send packet");

        //println!("sending x: {x:b}");
        //std::thread::sleep(std::time::Duration::from_secs(5));
        //}
    }

    //println!("done");
}
