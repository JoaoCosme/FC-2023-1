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
                let heigth = image.height as u32;
                let pixel = get_pixel_value(&data, x, y, width, heigth);
                //Para cada pixel verificar se é r g ou b
                // Se for R interpolar verde e azul
                // Se for G interpolar vermelho e azul
                // Se for azul interpolar vermelho e verde

                let mut demosaic_pixel: Rgb<u8> = Rgb([255, 255, 255]);
                if x % 2 == 0 || x == 0 {
                    if y % 2 == 0 || y == 0 {
                        //Caso vermelho
                        let green = get_pixel_value(&data, x + 1, y, width, heigth);
                        let blue = get_pixel_value(&data, x + 1, y + 1, width, heigth);
                        let red = pixel;
                        demosaic_pixel = Rgb([red, green, blue]);
                    } else {
                        //Caso verde
                        let red = get_pixel_value(&data, x + 1, y, width, heigth);
                        let blue = get_pixel_value(&data, x + 1, y + 1, width, heigth);
                        let green = pixel;
                        let demosaic_pixel = Rgb([red, green, blue]);
                        demosaic.put_pixel(x, y, demosaic_pixel);
                    }
                } else {
                    if y % 2 == 0 || y == 0 {
                        //Caso azul
                        let green = get_pixel_value(&data, x + 1, y, width, heigth);
                        let red = get_pixel_value(&data, x + 1, y + 1, width, heigth);
                        let blue = pixel;
                        let demosaic_pixel = Rgb([red, green, blue]);
                        demosaic.put_pixel(x, y, demosaic_pixel);
                    } else {
                        //Caso verde
                        let blue: u8 = get_pixel_value(&data, x + 1, y, width, heigth);
                        let red = get_pixel_value(&data, x + 1, y + 1, width, heigth);
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

fn get_pixel_value(data: &Vec<u16>, x: u32, y: u32, width: u32, height: u32) -> u8 {
    let index = x * width + y;
    if index >= width * height {
        return 0;
    }
    let pixel = data[index as usize];
    let pixel = (pixel as f32 / MAX_12_BIT_VALUE * u8::MAX as f32) as u8;
    pixel
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
