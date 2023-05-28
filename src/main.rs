fn main() {
    println!("Iniciando leitura de arquivos");
    let file = "./src/images/scene-raw2.dng";
    let image = rawloader::decode_file(file).unwrap();
    println!("{} {}", image.width, image.height);
}
