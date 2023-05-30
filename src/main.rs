use image::{ImageBuffer, Rgb};
const MAX_12_BIT_VALUE: f32 = 4095f32;

fn main() {
    println!("Iniciando leitura de arquivos.");
    let image = read_image();
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
    println!("Processamento completo! Iniciando salvamento...");
    demosaic.save("image.png").expect("Failed to save image");
    println!("Salvamento completo com sucesso!");
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
