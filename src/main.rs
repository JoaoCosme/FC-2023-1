use opencv::prelude::*;

fn main() {
    println!("Iniciando leitura de arquivos");
    let image = read_image();
    // let mut f = Mat::default();
    println!("{} {}", image.width, image.height);
    if let rawloader::RawImageData::Integer(data) = image.data {
        for x in 0..=image.width {
            for y in 0..=image.height {
                // println!("{}", data[x * y])
            }
        }
    }
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw2.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
