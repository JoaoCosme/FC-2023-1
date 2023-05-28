use image::{ImageBuffer, Rgb};

fn main() {
    println!("Iniciando leitura de arquivos");
    let image = read_image();
    let cfa = image.cfa;

    let mut demosaic = ImageBuffer::new(image.width as u32, image.height as u32);

    println!(
        "{} {} {} {}",
        image.width, image.height, image.cpp, image.model
    );
    println!(
        "Color at 0,0 {} 0,1 {}  1,0 {} 1,1 {}",
        cfa.color_at(0, 0),
        cfa.color_at(0, 1),
        cfa.color_at(1, 0),
        cfa.color_at(1, 1)
    );

    if let rawloader::RawImageData::Integer(data) = image.data {
        for x in 0..image.width as u32 {
            for y in 0..image.height as u32 {
                let pixel = get_pixel_value(&data, x, y);
                // println!("At {}, Pixel Value: {}", x + 1 * y, pixel);
                let demosaic_pixel = Rgb([pixel, pixel, pixel]);
                demosaic.put_pixel(x, y, demosaic_pixel);
            }
        }
    }

    demosaic.save("image.png").expect("Failed to save image");
}

fn get_pixel_value(data: &Vec<u16>, x: u32, y: u32) -> u8 {
    let pixel = *data[(x + 1 * y) as usize];
    let pixel = (pixel as f32 / 4095f32 * u8::MAX as f32) as u8;
    pixel
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.cr2";
    let image = rawloader::decode_file(file).unwrap();
    image
}
