use opencv::{
    core::{Scalar, Vector, CV_8UC3},
    imgcodecs::imwrite,
    prelude::*,
};

fn main() {
    println!("Iniciando leitura de arquivos");
    let image = read_image();
    let mut f = Mat::new_rows_cols_with_default(
        image.height as i32,
        image.width as i32,
        CV_8UC3,
        Scalar::all(0.0),
    )
    .unwrap();

    println!("{} {}", image.width, image.height);
    if let rawloader::RawImageData::Integer(data) = image.data {
        for x in 0..image.width as i32 {
            for y in 0..image.height as i32 {
                let a = f.at_2d_mut(x, y).unwrap();
            }
        }
    }
    imwrite("image.png", &f, &Vector::default()).expect("Should save image!");
}

fn read_image() -> rawloader::RawImage {
    let file = "./src/images/scene-raw2.dng";
    let image = rawloader::decode_file(file).unwrap();
    image
}
