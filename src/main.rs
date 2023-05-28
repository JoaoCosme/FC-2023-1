fn main() {
    println!("Iniciando leitura de arquivos");
    let image = read_image();
    println!("{} {}", image.width, image.height);
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw2.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
