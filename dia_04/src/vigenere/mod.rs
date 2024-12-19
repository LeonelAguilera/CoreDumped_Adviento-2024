use core::fmt;

use unicode_segmentation::UnicodeSegmentation;
//use dia_04::generar_diccionario;

pub const MAX_KEY_LEN: usize = 10; 
pub const NUMBER_OF_LETTERS: u64 = 26;
pub const MAX_THREADS: u64 = 64;
pub const STACK_SIZE: usize = 50*1024*1024;
//generar_diccionario!();
pub const DICCIONARIO: &str = include_str!("0_palabras_todas.txt"); //Obtenido de: https://github.com/JorgeDuenasLerin/diccionario-espanol-txt

pub mod brute_force_cracker;
pub mod smart_cracker;

#[derive(Debug)]
pub struct Soluciones{
    texto_codificado: String,
    soluciones: Vec<String>,
    puntuacion: usize,
}

impl Soluciones {
    fn new(texto_codificado: String) -> Self{
        Self {texto_codificado,  soluciones: Vec::new(), puntuacion: 1 }
    }
    fn push(&mut self, llave: String){
        let texto_decodificado = decodificador_vigenere(&self.texto_codificado, &llave);
        let nueva_puntuacion = evaluar_solucion_rapido(&texto_decodificado);
        if nueva_puntuacion > self.puntuacion{
            self.puntuacion = nueva_puntuacion;
            self.soluciones.clear();
            self.soluciones.push(llave);
        }
        else if nueva_puntuacion == self.puntuacion {
            if self.soluciones.contains(&llave) == false{
                self.soluciones.push(llave);
            }
        }
    }
    fn sort(&mut self){
        self.soluciones.sort_by(|lhs, rhs| {
            let lhs_score = evaluar_solucion(&decodificador_vigenere(&self.texto_codificado, lhs));
            let rhs_score = evaluar_solucion(&decodificador_vigenere(&self.texto_codificado, rhs));
            rhs_score.cmp(&lhs_score)
        });
    }
    pub fn get_soluciones(&self) -> Vec<String>{
        self.soluciones.clone()
    }
}

impl fmt::Display for Soluciones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self
               .soluciones
               .iter()
               .map(|llave|{
                   let palabras_reales = evaluar_solucion(&decodificador_vigenere(&self.texto_codificado, llave));
                   let palabras_totales = self.texto_codificado.unicode_words().count();
                   format!("{llave}: {}%", (palabras_reales*100)/palabras_totales)
               })
               .collect::<Vec<String>>()
               .join("\n")
               )
    }
}

#[allow(dead_code)]
pub fn decodificador_vigenere(texto: &String, llave: &String) -> String{
    let mut texto = texto.to_lowercase().into_bytes();
    let llave = llave.to_lowercase().into_bytes();
    let keylen = llave.len();
    let texlen = texto.len();

    let mut j = 0;
    for i in 0..texlen{
        if texto[i] < 'a' as u8 || texto[i] > 'z' as u8{
            continue;
        }
        //println!("index: {}, texto: {}, llave: {}, a: {}", i, texto[i], llave[j], 'a' as u8);
        let a = texto[i] - ('a' as u8);
        let b = llave[j] - ('a' as u8);
        texto[i] = ((a as i16 - b as i16).rem_euclid(26)) as u8 + ('a' as u8);
        j = (j+1)%keylen;
    }

    return String::from_utf8(texto).unwrap();
}

fn evaluar_solucion_rapido(texto: &String) -> usize{
    let mut puntuacion = 0;
    //Top 10 palabras más comunes en español: https://www.elespanol.com/curiosidades/lenguaje/palabras-mas-comunes-espanol-lista/639936112_0.html 
    if texto.contains(" de "){
        puntuacion += 1;
    }
    if texto.contains(" y "){
        puntuacion += 1;
    }
    if texto.contains(" el "){
        puntuacion += 1;
    }
    if texto.contains(" la "){
        puntuacion += 1;
    }
    if texto.contains(" en "){
        puntuacion += 1;
    }
    if texto.contains(" a "){
        puntuacion += 1;
    }
    if texto.contains(" que "){
        puntuacion += 1;
    }
    if texto.contains(" los "){
        puntuacion += 1;
    }
    if texto.contains(" se "){
        puntuacion += 1;
    }
    if texto.contains(" las "){
        puntuacion += 1;
    }

    return puntuacion;
}

fn evaluar_solucion(texto: &String) -> usize{
    let mut puntuacion = 0;

    for palabra in texto.unicode_words(){
        if DICCIONARIO.contains(palabra){
            puntuacion += 1;
        }
        /*
        for palabra_real in DICCIONARIO[texto.len()]{
            if palabra == palabra_real{
                puntuacion += 1;
            }
        }*/
    }

    return puntuacion;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn decod_1(){
        let texto_ejemplo = String::from("LXFOPVEFRNHR");
        let key = String::from("LEMON");
        let texto_codificado = decodificador_vigenere(&texto_ejemplo, &key);
        println!("{}", texto_codificado);
        assert_eq!(texto_codificado.to_string(), String::from("ATTACKATDAWN").to_lowercase())
    }
    #[test]
    fn decod_2(){
        let texto_ejemplo = String::from("LXFOPV EF RNHR");
        let key = String::from("LEMON");
        let texto_codificado = decodificador_vigenere(&texto_ejemplo, &key);
        println!("{}", texto_codificado);
        assert_eq!(texto_codificado.to_string(), String::from("ATTACK AT DAWN").to_lowercase())
    }
}
