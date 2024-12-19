use unicode_segmentation::UnicodeSegmentation;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generar_diccionario(_: TokenStream) -> TokenStream{
//pub fn segmentar_por_longitud_de_palabras(input: String) -> String{
    let input_string = include_str!("vigenere/0_palabras_todas.txt"); //Obtenido de: https://github.com/JorgeDuenasLerin/diccionario-espanol-txt
    //let input_string = include_str!("vigenere/diccionario_test.txt"); 
    let max_word_len = input_string.lines().map(|palabra| palabra.graphemes(true).count()).max().expect("No se pudo determinar el tamaño de la palabra más larga del diccionario");
    let mut string_segmentado: Vec<Vec<String>> = Vec::new();
    for _ in 0..max_word_len{
        string_segmentado.push(Vec::new());
    }
    input_string.lines().for_each(|palabra|{
        string_segmentado[
            palabra
                .graphemes(true)
                .count() - 1]
        .push(palabra
            .graphemes(true)
            .map(|grafema|
                match grafema {
                    "á" => "a",
                    "é" => "e",
                    "í" => "i",
                    "ó" => "o",
                    "ú" => "u",
                    "ü" => "u",
                    "ñ" => "n",
                    _   => grafema,
                })
            .collect::<Vec<&str>>()
            .join("")
            .to_string()
            )
        }
    );
    let tamaños_cubos = string_segmentado.clone().into_iter().map(|lista| lista.len());
    let coleccion_mas_larga = tamaños_cubos.clone().max().unwrap();
    let numero_cubos = string_segmentado.len();
    let vec_tokens = string_segmentado
        .into_iter()
        .map(|lista| format!("[{}]",{
            let mut bind = lista
                .into_iter()
                .map(|palabra| (format!("\"{}\"", palabra).to_string()))
                .collect::<Vec<String>>();
            bind.append(&mut vec!["\"\"".to_string();coleccion_mas_larga - bind.len()]);
            bind.join(", ")
        }))
        .collect::<Vec<_>>()
        .join(", ");

        let tamaños_cubos = tamaños_cubos
            .map(|longitud| longitud.to_string())
            .collect::<Vec<String>>()
            .join(", ");
    
    //const RESULT: &'static[&'static[&str]] = segmentar_por_longitud_de_palabras!(INPUT);
    format!("pub const DICCIONARIO: [[&str; {}]; {}] = [{}];\npub const DICCIONARIO_SIZE: [usize; {}] = [{}];", coleccion_mas_larga, numero_cubos, vec_tokens, numero_cubos, tamaños_cubos).parse().unwrap()
}

