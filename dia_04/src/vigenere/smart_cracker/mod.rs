use std::{sync::{Arc, Mutex}, thread};

use unicode_segmentation::UnicodeSegmentation;

use crate::vigenere::{MAX_KEY_LEN, MAX_THREADS};

use super::{Soluciones, DICCIONARIO, STACK_SIZE};
/*
pub fn smart_decode(texto: String) -> Vec<String>{
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move||smart_vigenere_cracker(texto))
        .unwrap();

    child.join().unwrap()
}*/

pub fn smart_vigenere_cracker(texto: String) -> Soluciones{
    let texto = texto.to_lowercase();
    let soluciones = Arc::new(Mutex::new(Soluciones::new(texto.clone())));

    for longitud_llave in 10..(MAX_KEY_LEN + 1){
        println!("{longitud_llave}");
        let mut handles = Vec::new();
        let (string_a_decodificar, longitud_palabras) = leer_palabras(&texto, longitud_llave);
        let (palabras_posibles, combinaciones_posibles) = obtener_posibles_palabras_de_inicio(longitud_palabras);

        let longitudes_listas_palabras = Arc::new(palabras_posibles.iter().map(|p| p.len()).collect::<Vec<usize>>());
        let string_a_decodificar = Arc::new(string_a_decodificar);
        let palabras_posibles = Arc::new(palabras_posibles);

        for hilo in {
            if combinaciones_posibles > 2*MAX_THREADS{
                0..MAX_THREADS
            }
            else{
                0..1u64
            }
        }{
            let id_combinacion_min = hilo*combinaciones_posibles/MAX_THREADS;
            let id_combinacion_max = ((hilo+1)*combinaciones_posibles/MAX_THREADS) - 1;
            let string_a_decodificar = Arc::clone(&string_a_decodificar);
            let palabras_posibles = Arc::clone(&palabras_posibles);
            let longitudes_listas_palabras = Arc::clone(&longitudes_listas_palabras);
            let soluciones = Arc::clone(&soluciones);

            let handle = thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move||{
                    cuerpo(id_combinacion_min,
                           id_combinacion_max,
                           longitud_llave,
                           string_a_decodificar,
                           palabras_posibles,
                           longitudes_listas_palabras,
                           soluciones
                          );
                })
                .unwrap();
            handles.push(handle);
        }
        for handle in handles{
            handle.join().unwrap();
        }
    }
    let mut soluciones = soluciones.lock().unwrap();
    soluciones.sort();
    return Soluciones{
        puntuacion: soluciones.puntuacion,
        soluciones: soluciones.soluciones.clone(),
        texto_codificado: soluciones.texto_codificado.clone(),
    };
}

fn cuerpo(id_combinacion_min: u64, id_combinacion_max: u64, longitud_llave: usize, string_a_decodificar: Arc<Vec<i16>>, palabras_posibles: Arc<Vec<Vec<String>>>, longitudes_listas_palabras: Arc<Vec<usize>>, soluciones: Arc<Mutex<Soluciones>>){
    for combinacion in id_combinacion_min..id_combinacion_max{
        let mut texto_objetivo = String::new();
        let palabras = indice_combinacion_a_indices_palabras(combinacion, Arc::clone(&longitudes_listas_palabras));

        for i in 0..palabras.len(){
            let siguiente_palabra = palabras_posibles[i][palabras[i]].clone();
            texto_objetivo += &siguiente_palabra;
        }
        let texto_objetivo = texto_objetivo.ascii_offset_and_trim(longitud_llave);

        let mut llave = String::new();
        assert_eq!(string_a_decodificar.len(), texto_objetivo.len());
        for i in 0..longitud_llave{
            let numero_letra = string_a_decodificar[i] - texto_objetivo[i];
            let numero_letra = numero_letra.rem_euclid(26);
            let letra = ((numero_letra + ('a' as i16)) as u8) as char;

            llave.push(letra);
        }
        soluciones.lock().unwrap().push(llave);
        
        /*
        let llaves_a_procesar = id_combinacion_max-id_combinacion_min;
        let divisor_log = llaves_a_procesar/20;

        if (combinacion - id_combinacion_min)%divisor_log == 0{
            println!("{}..{}..{}", id_combinacion_min, combinacion, id_combinacion_max);
        }*/
    }
}

fn indice_combinacion_a_indices_palabras(indice: u64, longitudes: Arc<Vec<usize>>) -> Vec<usize>{
    let indice = indice as usize;
    let mut indices = Vec::new();
    for i in 0..longitudes.len() {
        let mut divisor = 1;
        for j in 0..i{
            divisor *= longitudes[j];
        }
        indices.push((indice/divisor)%longitudes[i]);
    }

    return indices;
}

fn obtener_posibles_palabras_de_inicio(longitud_palabras: Vec<usize>) -> (Vec<Vec<String>>, u64){
    let mut posibles_palabras = Vec::new();

    let mut combinaciones_posibles = 1;
    for longitud in longitud_palabras{
        let palabras: Vec<String> = DICCIONARIO.lines().filter_map(|palabra| {
            let palabra = (*palabra).to_string();
            if palabra.graphemes(true).count() == longitud{
                //println!("{palabra}");
                Some(palabra)
            }
            else{
                None
            }
        }).collect();
        combinaciones_posibles *= palabras.len();
        posibles_palabras.push(palabras);
    }

    return (posibles_palabras, combinaciones_posibles as u64);
}
/*
fn obtener_posibles_palabras_de_inicio(longitud_palabras: Vec<usize>) -> (Vec<Vec<String>>, u64){
    let mut posibles_palabras = Vec::new();

    let mut combinaciones_posibles = 1;
    for longitud in longitud_palabras{
        let palabras: Vec<String> = DICCIONARIO[longitud - 1].iter().filter_map(|palabra| {
            let palabra = (*palabra).to_string();
            match palabra.len() {
                0 => None,
                _ => {
                    //println!("{palabra}");
                    return Some(palabra)},
            }
        }).collect();
        combinaciones_posibles *= palabras.len();
        posibles_palabras.push(palabras);
    }

    return (posibles_palabras, combinaciones_posibles as u64);
}*/

fn leer_palabras(texto: &str, max_caracteres: usize) -> (Vec<i16>, Vec<usize>){
    let mut palabras = texto.unicode_words();

    let mut decoding_string = String::new();
    let mut word_len_collection: Vec<usize> = Vec::new();
    while decoding_string.graphemes(true).count() < max_caracteres {
        let remaining_chars = max_caracteres - decoding_string.graphemes(true).count();
        let word = palabras.next().unwrap();

        if word.graphemes(true).count() < remaining_chars {
            decoding_string += word;
        }
        else{
            decoding_string += &word.graphemes(true).take(remaining_chars).collect::<Vec<&str>>().join("");
        }
        word_len_collection.push(word.graphemes(true).count());
    }

    return (decoding_string.ascii_offset_and_trim(max_caracteres), word_len_collection);
}

trait VigenereUtils {
    fn ascii_offset_and_trim(&self, trim: usize) -> Vec<i16>;
}

impl VigenereUtils for String {
    fn ascii_offset_and_trim(&self, trim: usize) -> Vec<i16> {
        //println!("{self}");
        self
            .graphemes(true)
            .map(|grafema|{
                match grafema {
                    "á" => "a",
                    "é" => "e",
                    "í" => "i",
                    "ó" => "o",
                    "ú" => "u",
                    "ü" => "u",
                    "ñ" => "n",
                    _   => grafema,
                }
            })
            .take(trim)
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .map(|caracter|{
                //println!{"{caracter}"};
                ((caracter as u8) - ('a' as u8)) as i16
            })
            .collect()
    }
}
