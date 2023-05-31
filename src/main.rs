use image::{ImageBuffer, Rgb};
const MAX_12_BIT_VALUE: f32 = 4095f32;

fn main() {
    println!("Iniciando leitura de arquivos.");
    let image = read_image();
    let demosaic = demosaick_image(image);
    let white_balance = white_balance(&demosaic);
    println!("Processamento completo! Iniciando salvamento...");
    demosaic.save("image.png").expect("Failed to save image");
    println!("Salvamento completo com sucesso!");
}

fn white_balance(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Computar a media de R G e B ao longo da imagem
    let mut red_avg: i32 = 0;
    let mut green_avg: i32 = 0;
    let mut blue_avg: i32 = 0;
    for x in 0..image.width() {
        for y in 0..image.height() {
            red_avg += image.get_pixel(x, y).0[0] as i32;
            green_avg += image.get_pixel(x, y).0[1] as i32;
            blue_avg += image.get_pixel(x, y).0[2] as i32;
        }
    }
    red_avg = red_avg / (image.width() * image.height()) as i32;
    green_avg = green_avg / (image.width() * image.height()) as i32;
    blue_avg = blue_avg / (image.width() * image.height()) as i32;

    let alfa = green_avg / red_avg;
    let beta = green_avg / blue_avg;

    let mut white_balanced_image = ImageBuffer::new(image.width(), image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let red = image.get_pixel(x, y).0[0] as i32 * alfa;
            let green = image.get_pixel(x, y).0[1];
            let blue = image.get_pixel(x, y).0[2] as i32 * beta;

            white_balanced_image.put_pixel(x, y, Rgb([red as u8, green, blue as u8]));
        }
    }

    // Computar os ganhos alfa e beta a partir disso
    // Multiplicar R pelo ganho alfa
    // Multiplicar B pelo ganho beta
    white_balanced_image
}

fn demosaick_image(image: rawloader::RawImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut demosaic = ImageBuffer::new(image.width as u32, image.height as u32);

    println!(
        "Largura: {} Altura: {} Profundidade:{} Modelo:{}",
        image.width, image.height, image.cpp, image.model
    );

    // A imagem RAW aqui usa valores 12-bit unassigned.
    // Segue a seguinte ordem:
    // [R G
    //  G B]

    println!("Iniciando Demosaicking.");
    if let rawloader::RawImageData::Integer(data) = image.data {
        for x in 0..image.width as u32 {
            for y in 0..image.height as u32 {
                let width = image.width as u32;
                let height = image.height as u32;
                //Para cada pixel verificar se é r g ou b
                // Se for R interpolar verde e azul
                // Se for G interpolar vermelho e azul
                // Se for azul interpolar vermelho e verde
                let get_pixel_value = make_get_pixel(&data, width, height);
                let pixel = get_pixel_value(x, y);

                let mut demosaic_pixel: Rgb<u8> = Rgb([255, 255, 255]);
                if x % 2 == 0 || x == 0 {
                    if y % 2 == 0 || y == 0 {
                        //Caso vermelho
                        let green = get_pixel_value(x + 1, y);
                        let blue = get_pixel_value(x + 1, y + 1);
                        let red = pixel;
                        demosaic_pixel = Rgb([red, green, blue]);
                    } else {
                        //Caso verde
                        let red = get_pixel_value(x + 1, y);
                        let blue = get_pixel_value(x + 1, y + 1);
                        let green = pixel;
                        let demosaic_pixel = Rgb([red, green, blue]);
                        demosaic.put_pixel(x, y, demosaic_pixel);
                    }
                } else {
                    if y % 2 == 0 || y == 0 {
                        //Caso azul
                        let green = get_pixel_value(x + 1, y);
                        let red = get_pixel_value(x + 1, y + 1);
                        let blue = pixel;
                        let demosaic_pixel = Rgb([red, green, blue]);
                        demosaic.put_pixel(x, y, demosaic_pixel);
                    } else {
                        //Caso verde
                        let blue: u8 = get_pixel_value(x + 1, y);
                        let red = get_pixel_value(x + 1, y + 1);
                        let green = pixel;
                        demosaic_pixel = Rgb([red, green, blue]);
                    }
                }
                demosaic.put_pixel(x, y, demosaic_pixel);
            }
        }
    }
    demosaic
}

fn make_get_pixel(data: &Vec<u16>, width: u32, height: u32) -> impl Fn(u32, u32) -> u8 + '_ {
    move |x: u32, y: u32| -> u8 {
        let index = x + y * width;
        if index >= width * height || index <= 0 {
            return 255;
        }
        let pixel = data[index as usize];
        let pixel = (pixel as f32 / MAX_12_BIT_VALUE * u8::MAX as f32) as u8;
        return pixel;
    }
}
fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
