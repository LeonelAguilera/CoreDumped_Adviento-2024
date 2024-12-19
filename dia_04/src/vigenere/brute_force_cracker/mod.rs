use std::{fs, sync::{Arc, Mutex}, thread};

use crate::vigenere::{Soluciones, MAX_KEY_LEN, MAX_THREADS, NUMBER_OF_LETTERS};

#[allow(dead_code)]
pub fn brute_force_vigenere_cracker(){
    let ruta = "Dia4.txt";
    let texto_binding = fs::read_to_string(ruta).unwrap();
    let soluciones = Arc::new(Mutex::new(Soluciones::new(texto_binding.clone())));

    for longitud_llave in 1..(MAX_KEY_LEN + 1){
        println!("{longitud_llave}");
        let mut handles = vec![];
        let llaves_posibles = NUMBER_OF_LETTERS.pow(longitud_llave as u32);
        if llaves_posibles > 2*MAX_THREADS{
            for t in 0..MAX_THREADS{
                let llave_minima = t*llaves_posibles/MAX_THREADS;
                let llave_maxima = ((t+1)*llaves_posibles/MAX_THREADS) - 1;
                let soluciones = Arc::clone(&soluciones);

                let handle = thread::spawn(move || comprobar_llave(longitud_llave, llave_minima, llave_maxima, soluciones));
                handles.push(handle);
            }
        }
        else{
            let soluciones = Arc::clone(&soluciones);

            let handle = thread::spawn(move || comprobar_llave(longitud_llave, 0, llaves_posibles, soluciones));
            handles.push(handle);
        }
        for handle in handles{
            handle.join().unwrap();
        }
    }


    let soluciones = soluciones.lock().unwrap();
    println!("Mejor puntuación: {}", soluciones.puntuacion);
    for solucion in 0..soluciones.soluciones.len(){
        println!("\tPosible llave: {solucion}");
    }
}

fn comprobar_llave(longitud_llave: usize, llave_minima: u64, llave_maxima: u64, soluciones: Arc<Mutex<Soluciones>>){
    for id_llave in llave_minima..llave_maxima{
        let mut soluciones = soluciones.lock().unwrap();
        let llave = generador_llaves(longitud_llave, id_llave);
        soluciones.push(llave)
    }
}

fn generador_llaves(len: usize, id_llave: u64) -> String{
    let mut llave = vec!['a' as u8; len];
    for k in 0..len{
        let relevant_digit = (id_llave / NUMBER_OF_LETTERS.pow(k as u32)) % NUMBER_OF_LETTERS;
        llave[k] = ('a' as u8) +  (relevant_digit as u8);
    }
    return String::from_utf8(llave).unwrap();
}

fn contar_caracteres(texto: String){
    let texto = texto.into_bytes();
    let mut cuenta = vec![0; 26];

    for car in texto{
        if car >= 'a' as u8 && car <= 'z' as u8{
            cuenta[(car - 'a' as u8) as usize] += 1;
        }
    }

    for i in 0..cuenta.len(){
        println!("Caracter \'{}\': {}", (i as u8 + 'a' as u8) as char, cuenta[i]);
    }
}
