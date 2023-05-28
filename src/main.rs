use image::{ImageBuffer, Rgb};
const MAX_12_BIT_VALUE: f32 = 4095f32;

fn main() {
    println!("Iniciando leitura de arquivos");
    let image = read_image();
    let cfa = image.cfa;

    let mut demosaic = ImageBuffer::new(image.width as u32, image.height as u32);

    println!(
        "{} {} {} {}",
        image.width, image.height, image.cpp, image.model
    );

    // A imagem RAW aqui usa valores 12-bit unassigned.
    // Segue a seguinte ordem:
    // [R G
    //  G B]

    if let rawloader::RawImageData::Integer(data) = image.data {
        for x in 0..image.width as u32 {
            for y in 0..image.height as u32 {
                let pixel = get_pixel_value(&data, x, y);
                //Para cada pixel verificar se é r g ou b
                // Se for R interpolar verde e azul
                // Se for G interpolar vermelho e azul
                // Se for azul interpolar vermelho e verde
                let demosaic_pixel = Rgb([pixel, pixel, pixel]);
                demosaic.put_pixel(x, y, demosaic_pixel);
            }
        }
    }

    demosaic.save("image.png").expect("Failed to save image");
}

fn get_pixel_value(data: &Vec<u16>, x: u32, y: u32) -> u8 {
    let pixel = data[(x + 1 * y) as usize];
    let pixel = (pixel as f32 / MAX_12_BIT_VALUE * u8::MAX as f32) as u8;
    pixel
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.cr2";
    let image = rawloader::decode_file(file).unwrap();
    image
}
