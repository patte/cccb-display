# CCCB Display

Library to show images on the "Service-Point" display at Chaos Computer Club Berlin.

![Service-Point LED display showing the CCC logo at Chaos Computer Club Berlin.](./assets/photo.jpg)


## Run

1. ENVs
    ```bash
    cp .env.example .env
    ```
    Adapt `CCCB_IP` to the ip of the screen

1. Run
    ```bash
    cargo run --release --example simple
    cargo run --release --example loop
    ```

## Example code

```rust
use cccb_display::{CccbDisplayImagePackage, CccbImageSender};
use image::io::Reader as ImageReader;

fn main() {
    let img = ImageReader::open("assets/ccc.png")
        .unwrap()
        .decode()
        .unwrap();

    // create package
    let img_packed = CccbDisplayImagePackage::new(img.clone(), |pix| pix[3] != 0, true);

    // save screenshot
    img_packed
        .to_luma8()
        .save("./screenshots/simple-screenshot.png")
        .unwrap();

    let mut sender = CccbImageSender::new_from_env();
    sender.send_package(&img_packed);
}
```


