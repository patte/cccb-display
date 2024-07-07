use image::{DynamicImage, GenericImageView, Rgba};

/*
pub fn pix_is_on_alpha(pix: &Rgba<u8>) -> bool {
    pix[3] != 0
}

pub fn pix_is_on_any(pix: &Rgba<u8>) -> bool {
    pix[0] > 128 || pix[1] > 128 || pix[2] > 128
}

pub fn pix_is_on_rgb(pix: &Rgba<u8>) -> bool {
    pix[0] + pix[1] + pix[2] > (60 * 3)
}
*/

pub struct CccbDisplayImagePackage {
    packed_bytes: Vec<u8>,
}

impl CccbDisplayImagePackage {
    const WIDTH: u8 = 56; // tiles (*8 for pixels)
    const HEIGHT: u8 = 160; // pixels aka. lines

    pub fn new(img: DynamicImage, pixel_is_on: fn(&Rgba<u8>) -> bool, resize: bool) -> Self {
        let img = if resize {
            img.resize(
                Self::WIDTH as u32 * 8,
                Self::HEIGHT as u32,
                image::imageops::FilterType::Nearest,
            )
        } else {
            img
        };
        let packed_bytes = Self::pack_image(&img, pixel_is_on);
        Self { packed_bytes }
    }

    fn pack_image(img: &DynamicImage, pixel_is_on: fn(&Rgba<u8>) -> bool) -> Vec<u8> {
        let width = Self::WIDTH;
        let height = Self::HEIGHT;
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
        let offset_y = (height as u32) - img.dimensions().1;

        for y in 0..(height as u32).min(img.dimensions().1) {
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
                        if pixel_is_on(&pix) {
                            current_byte = current_byte | 1;
                        }
                    }
                }

                packed_bytes[((y + offset_y) * width as u32 + x) as usize + 10] = current_byte;
            }
        }

        packed_bytes
    }

    pub fn to_luma8(&self) -> DynamicImage {
        let width = Self::WIDTH;
        let height = Self::HEIGHT;
        let mut packed_bytes_expanded: Vec<u8> = vec![0; width as usize * height as usize * 8];
        for y in 0..height as u32 {
            for x in 0..width as u32 {
                let byte = self.packed_bytes[(y * width as u32 + x) as usize + 10];
                for j in 0..8 {
                    let bit = (byte >> (7 - j)) & 1;
                    // bit == 0 => 00000000
                    // bit == 1 => 11111111
                    packed_bytes_expanded[(y * width as u32 + x) as usize * 8 + j] = bit * 255;
                }
            }
        }

        DynamicImage::ImageLuma8(
            image::GrayImage::from_raw(width as u32 * 8, height as u32, packed_bytes_expanded)
                .unwrap(),
        )
    }

    pub fn get_package(&self) -> &[u8] {
        &self.packed_bytes
    }
}

pub struct CccbImageSender {
    socket: std::net::UdpSocket,
    ip: String,
    port: u16,
}

const DEFAULT_PORT: u16 = 2342;

impl CccbImageSender {
    pub fn new_with_ip_port(ip: String, port: u16) -> Self {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket");
        Self { socket, ip, port }
    }

    pub fn new_from_env() -> Self {
        dotenv::dotenv().ok();
        Self::new_with_ip_port(
            std::env::var("CCCB_IP").expect("CCCB_IP not set"),
            std::env::var("CCCB_PORT")
                .unwrap_or(DEFAULT_PORT.to_string())
                .parse()
                .expect("CCCB_PORT not a number"),
        )
    }

    pub fn send_package(&mut self, package: &CccbDisplayImagePackage) {
        self.socket
            .send_to(package.get_package(), (self.ip.as_str(), self.port))
            .expect("failed to send packet");
    }
}
