use image::{ImageBuffer, Rgb};
const MAX_12_BIT_VALUE: f32 = 4095f32;

fn main() {
    println!("Iniciando leitura de arquivos.");
    let demosaic = demosaick_image(read_raw_image());
    let white_balance_gray_world = white_balance(&demosaic);
    let white_balance = white_balance_scaling(&demosaic);
    let gamma_encoded = gamma_correct_image(&white_balance);
    println!("Processamento completo! Iniciando salvamento de imagens...");
    save_image(&demosaic, "demosaick.png");
    save_image(&white_balance, "white_balance.png");
    save_image(&white_balance_gray_world, "white_balance_gray_world.png");
    save_image(&gamma_encoded, "gamma_encoded.png");
    println!("Salvamento completo com sucesso!");
}

fn save_image(image: &ImageBuffer<Rgb<f32>, Vec<f32>>, file_name: &str) {
    let (width, height) = image.dimensions();
    let mut output_image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        let red = (pixel[0] * 255.0).round() as u8;
        let green = (pixel[1] * 255.0).round() as u8;
        let blue = (pixel[2] * 255.0).round() as u8;

        output_image.put_pixel(x, y, Rgb([red, green, blue]));
    }

    output_image.save(file_name).expect("Should save image");
}
fn gamma_correct_image(image: &ImageBuffer<Rgb<f32>, Vec<f32>>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    println!("Incializando Gamma Encoding");
    let mut gamma_corrected_image = ImageBuffer::new(image.width(), image.height());
    let apply_gamma = |value: f32| -> f32 { (value).powf(1.0 / 2.2) };
    for (x, y, pixel) in image.enumerate_pixels() {
        let red = apply_gamma(pixel[0]);
        let green = apply_gamma(pixel[1]);
        let blue = apply_gamma(pixel[2]);
        gamma_corrected_image.put_pixel(x, y, Rgb([red, green, blue]));
    }
    gamma_corrected_image
}

fn white_balance_scaling(
    image: &ImageBuffer<Rgb<f32>, Vec<f32>>,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    println!("Incializando whitebalancing:Scaling RGB");
    let red_adjust = 255f32 / 139f32;
    let green_adjust = 255f32 / 231f32;
    let blue_adjust = 255f32 / 134f32;

    let mut white_balanced_image = ImageBuffer::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let red = pixel[0] * red_adjust;
        let green = pixel[1] * green_adjust;
        let blue = pixel[2] * blue_adjust;

        white_balanced_image.put_pixel(x, y, Rgb([red, green, blue]));
    }
    white_balanced_image
}

fn white_balance(image: &ImageBuffer<Rgb<f32>, Vec<f32>>) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    println!("Incializando whitebalancing:Gray World");
    // Computar a media de R G e B ao longo da imagem
    let mut red_avg: f32 = 0f32;
    let mut green_avg: f32 = 0f32;
    let mut blue_avg: f32 = 0f32;
    for (_x, _y, pixel) in image.enumerate_pixels() {
        red_avg += pixel[0];
        green_avg += pixel[1];
        blue_avg += pixel[2];
    }
    let image_size = (image.width() * image.height()) as f32;
    red_avg = red_avg / image_size;
    green_avg = green_avg / image_size;
    blue_avg = blue_avg / image_size;

    let alfa = green_avg / red_avg;
    let beta = green_avg / blue_avg;

    let mut white_balanced_image = ImageBuffer::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let red = pixel[0] * alfa;
        let green = pixel[1];
        let blue = pixel[2] * beta;

        white_balanced_image.put_pixel(x, y, Rgb([red, green, blue]));
    }

    // Computar os ganhos alfa e beta a partir disso
    // Multiplicar R pelo ganho alfa
    // Multiplicar B pelo ganho beta
    white_balanced_image
}

fn demosaick_image(image: rawloader::RawImage) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let mut demosaic = ImageBuffer::new(image.width as u32, image.height as u32);

    println!(
        "Largura: {} Altura: {} Profundidade:{} Modelo:{}",
        image.width, image.height, image.cpp, image.model
    );

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
            (get_pixel_value(x + 1, y)
                + get_pixel_value(x + 3, y)
                + get_pixel_value(x.saturating_sub(1), y)
                + get_pixel_value(x.saturating_sub(3), y))
                / 4
        };
        let get_avg_vertical = |x: u32, y: u32| -> u16 {
            (get_pixel_value(x, y + 1)
                + get_pixel_value(x, y + 3)
                + get_pixel_value(x, y.saturating_sub(1))
                + get_pixel_value(x, y.saturating_sub(3)))
                / 4
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
                if x % 2 == 0 || x == 0 {
                    if y % 2 == 0 || y == 0 {
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
                        //Caso verde
                        blue = get_avg_horizontal(x, y);
                        red = get_avg_vertical(x, y);
                        green = pixel;
                    } else {
                        //Caso azul
                        green = get_avg_horizontal_and_vertical(x, y);
                        red = get_avg_diagonally(x, y);
                        blue = pixel;
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
            return 0;
        }
        data[index as usize]
    }
}

fn normalize_pixel_val(pixel: u16) -> f32 {
    pixel as f32 / MAX_12_BIT_VALUE
}
fn read_raw_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
