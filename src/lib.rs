use wasm_bindgen::prelude::*;
use image::io::Reader;
use std::io::Cursor;

#[wasm_bindgen]
pub fn strip_metadata(data: &[u8]) -> Vec<u8> {
    // Carrega a imagem a partir dos bytes e detecta o formato
    let reader = Reader::new(Cursor::new(data))
        .with_guessed_format()
        .expect("Formato desconhecido");

    let format = reader.format().expect("Falha ao determinar formato da imagem");
    let img = reader.decode().expect("Falha ao decodificar imagem");

    // Recodifica a imagem no formato original sem metadados
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, format)
        .expect("Falha ao codificar");

    buffer.into_inner()
}