use image::{ImageReader, RgbImage};

fn main() {
    let ruta = "Dia8.png";
    let imagen_b = ImageReader::open(ruta).expect("No se pudo abrir la imagen").decode().expect("Formato no soportado");
    let imagen = imagen_b.as_rgba8().unwrap();
    let mut imagen_decodificada: Vec<u8> = Vec::new();

    for pix in imagen.pixels(){
        let pix_decodificado_r = (pix[0] & 0b0000001) << 7;
        let pix_decodificado_g = (pix[1] & 0b0000001) << 7;
        let pix_decodificado_b = (pix[2] & 0b0000001) << 7;

        imagen_decodificada.push(pix_decodificado_r);
        imagen_decodificada.push(pix_decodificado_g);
        imagen_decodificada.push(pix_decodificado_b);
    }

    let imagen_decodificada = RgbImage::from_raw(imagen.width(), imagen.height(), imagen_decodificada);
    imagen_decodificada.expect("Buffer insuficiente").save("imagen_decodificada.png").expect("No se pudo guardar la imagen");
}
