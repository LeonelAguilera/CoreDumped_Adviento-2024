/*
Day 4 Rsxupidlgadr Qanljoe

Ddw xanaoovacid dg lbd rwkicau hby ifrvtpvaez rmtwercs dzqmcmnaeipyik! As bug np dettr ps suf eiftqzs wnb aedpfca uedcilp glpcz ep vgbtpr voelw dpw pnerjaxsrmzngs, fd twaenadlfd. Ikeico suf ys zpcln gndcmhieoo gsul ggbyyieadtsf, pyyqwe tpkmgs bug tv aywsid rqmqpvdp!
 *
 */

mod vigenere;
use std::fs;

use vigenere::{decodificador_vigenere, smart_cracker::smart_vigenere_cracker};

fn main() {
    let ruta_texto = "Dia4.txt";
    let texto = fs::read_to_string(ruta_texto).unwrap();

    let llaves = smart_vigenere_cracker(texto.clone());

    println!("{llaves}\n\n");

    println!("{}", decodificador_vigenere(&texto, &llaves.get_soluciones()[0]));
}


#[allow(dead_code)]
fn decodificador_monoalfabetico(texto: String, llave: Vec<u8>) -> String{
    let mut texto = texto.to_lowercase().into_bytes();

    for i in 0..texto.len(){
        if texto[i] >= 'a' as u8 && texto[i] <= 'z' as u8{
            texto[i] = texto[i] - 32;
        }
    }

    for i in 0..llave.len(){
        for j in 0..texto.len(){
            if texto[j] == llave[i] - 32 {
                texto[j] = ('a' as u8) + (i as u8);
            }
        }
    }

    return  String::from_utf8(texto).unwrap();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn decod_3(){
        let texto_ejemplo = String::from("qbubub");
        let key:Vec<u8> = vec!['b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8, 'n' as u8, 'o' as u8, 'p' as u8, 'q' as u8, 'r' as u8, 's' as u8, 't' as u8, 'u' as u8, 'v' as u8, 'w' as u8, 'x' as u8, 'y' as u8, 'z' as u8, 'a' as u8];
        let texto_decodificado = decodificador_monoalfabetico(texto_ejemplo, key);
        println!("{}", texto_decodificado);
        assert_eq!(texto_decodificado, String::from("patata"));
    }
}
