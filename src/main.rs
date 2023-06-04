use image::{ImageBuffer, Rgb};
const MAX_12_BIT_VALUE: f32 = 4095f32;

fn main() {
    println!("Iniciando leitura de arquivos.");
    let image = read_image();
    let demosaic = demosaick_image(image);
    let white_balance = white_balance(&demosaic);
    let gamma_encoded = gamma_correct_image(&white_balance);
    println!("Processamento completo! Iniciando salvamento...");
    demosaic
        .save("demosaick.png")
        .expect("Failed to save image");
    white_balance
        .save("white_balance.png")
        .expect("Failed to save image");
    gamma_encoded
        .save("gamma_encoded.png")
        .expect("Failed to save image");
    println!("Salvamento completo com sucesso!");
}

fn gamma_correct_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut gamma_corrected_image = ImageBuffer::new(image.width(), image.height());
    let apply_gamma = |value: u8| -> u8 { (value as f32).powf(1.0 / 2.2) as u8 };
    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y).0;
            let red = apply_gamma(pixel[0]);
            let green = apply_gamma(pixel[1]);
            let blue = apply_gamma(pixel[2]);
            gamma_corrected_image.put_pixel(x, y, Rgb([red, green, blue]));
        }
    }
    gamma_corrected_image
}

fn white_balance(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Computar a media de R G e B ao longo da imagem
    let mut red_avg: f32 = 0f32;
    let mut green_avg: f32 = 0f32;
    let mut blue_avg: f32 = 0f32;
    for x in 0..image.width() {
        for y in 0..image.height() {
            red_avg += image.get_pixel(x, y).0[0] as f32;
            green_avg += image.get_pixel(x, y).0[1] as f32;
            blue_avg += image.get_pixel(x, y).0[2] as f32;
        }
    }
    red_avg = red_avg / (image.width() * image.height()) as f32;
    green_avg = green_avg / (image.width() * image.height()) as f32;
    blue_avg = blue_avg / (image.width() * image.height()) as f32;

    let alfa = green_avg / red_avg;
    let beta = green_avg / blue_avg;

    dbg!(alfa, beta);

    let mut white_balanced_image = ImageBuffer::new(image.width(), image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let red = image.get_pixel(x, y).0[0] as f32 * alfa;
            let green = image.get_pixel(x, y).0[1];
            let blue = image.get_pixel(x, y).0[2] as f32 * beta;

            white_balanced_image.put_pixel(
                x,
                y,
                Rgb([
                    normalize_pixel_val(red as u16),
                    normalize_pixel_val(green as u16),
                    normalize_pixel_val(blue as u16),
                ]),
            );
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

    //TODO: Fazer um metodo que busca a media na horizontal+vertical
    // Um que faça na diagonal
    // Fazer metodo que pegue somente horizontal
    // Um que faça somente na vertical

    println!("Iniciando Demosaicking.");

    if let rawloader::RawImageData::Integer(data) = image.data {
        let get_pixel_value = make_get_pixel(&data, image.width as u32, image.height as u32);
        let get_avg_horizontal_and_vertical = |x: u32, y: u32| -> u16 {
            (get_pixel_value(x + 1, y)
                + get_pixel_value(x.saturating_sub(1), y)
                + get_pixel_value(x, y + 1)
                + get_pixel_value(x, y.saturating_sub(1)))
                / 4
        };
        let get_avg_horizontal = |x: u32, y: u32| -> u16 {
            (get_pixel_value(x + 1, y) + get_pixel_value(x.saturating_sub(1), y)) / 2
        };
        let get_avg_vertical = |x: u32, y: u32| -> u16 {
            (get_pixel_value(x, y + 1) + get_pixel_value(x, y.saturating_sub(1))) / 2
        };
        let get_avg_diagonally = |x: u32, y: u32| -> u16 {
            (get_pixel_value(x + 1, y + 1)
                + get_pixel_value(x.saturating_sub(1), y + 1)
                + get_pixel_value(x + 1, y.saturating_sub(1))
                + get_pixel_value(x.saturating_sub(1), y.saturating_sub(1)))
                / 4
        };
        for x in 0..image.width as u32 {
            for y in 0..image.height as u32 {
                //Para cada pixel verificar se é r g ou b
                // Se for R interpolar verde e azul
                // Se for G interpolar vermelho e azul
                // Se for azul interpolar vermelho e verde

                let pixel = get_pixel_value(x, y);

                let demosaic_pixel;
                let red;
                let green;
                let blue;
                if x % 2 != 0 || x == 0 {
                    if y % 2 != 0 || y == 0 {
                        //Caso vermelho
                        green = get_avg_horizontal_and_vertical(x, y);
                        blue = get_avg_diagonally(x, y);
                        red = pixel;
                    } else {
                        //Caso verde
                        red = get_avg_horizontal(x, y);
                        blue = get_avg_vertical(x, y);
                        green = pixel;
                    }
                } else {
                    if y % 2 == 0 || y == 0 {
                        //Caso azul
                        green = get_avg_horizontal_and_vertical(x, y);
                        red = get_avg_diagonally(x, y);
                        blue = pixel;
                    } else {
                        //Caso verde
                        blue = get_avg_horizontal(x, y);
                        red = get_avg_vertical(x, y);
                        green = pixel;
                    }
                }
                demosaic_pixel = Rgb([
                    normalize_pixel_val(red),
                    normalize_pixel_val(green),
                    normalize_pixel_val(blue),
                ]);
                demosaic.put_pixel(x, y, demosaic_pixel);
            }
        }
    }
    demosaic
}

fn make_get_pixel(data: &Vec<u16>, width: u32, height: u32) -> impl Fn(u32, u32) -> u16 + '_ {
    move |x: u32, y: u32| -> u16 {
        let index = x + y * width;
        if index >= width * height || index <= 0 {
            return 255;
        }
        data[index as usize]
    }
}

fn normalize_pixel_val(pixel: u16) -> u8 {
    (pixel as f32 / MAX_12_BIT_VALUE * u8::MAX as f32) as u8
}
fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
