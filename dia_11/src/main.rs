use std::fs;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Prediccion{
    palabra: String,
    repeticiones: usize,
}

impl Prediccion {
    fn new() -> Self{
        Self{
            palabra: String::new(),
            repeticiones: 0,
        }
    }
}

struct Palabra{
    palabra: String,
    siguiente: Vec<Prediccion>,
}

impl Palabra {
    fn new(palabra: &str, prediccion: &str) -> Self{
        Palabra{
            palabra: palabra.to_string(),
            siguiente: vec![Prediccion{palabra: prediccion.to_string(), repeticiones: 1}],
        }
    }

    fn push(&mut self, nueva_prediccion: &str){
        if let Some(indice) = self.index(nueva_prediccion){
            self.siguiente[indice].repeticiones += 1;
        }
        else{
            self.siguiente.push(Prediccion{
                palabra: nueva_prediccion.to_string(),
                repeticiones: 1,
            });
        }
    }
    fn index(&self, palabra: &str) -> Option<usize>{
        for indice in 0..self.siguiente.len(){
            if self.siguiente[indice].palabra == palabra{
                return Some(indice);
            }
        }
        return None;
    }
}

struct Modelo{
    palabras: Vec<Palabra>,
    repeticiones_totales: Vec<Prediccion>,
}

impl Modelo {
    fn new() -> Self{
        Modelo{
            palabras: Vec::new(),
            repeticiones_totales: Vec::new(),
        }
    }

    fn entrenar(&mut self, datos_entrenamiento: &str){
        let mut iter_datos_entrenamiento = datos_entrenamiento
            .unicode_words()
            .peekable();
        loop {
            if let Some(palabra) = iter_datos_entrenamiento.next(){
                self.add_repeticion(palabra);
                if let Some(indice) = self.index(palabra){
                    if let Some(siguiente) = iter_datos_entrenamiento.peek(){
                        self.palabras[indice].push(siguiente);
                    }
                    else{
                        self.palabras[indice].push(datos_entrenamiento.unicode_words().next().unwrap());
                    }
                }
                else{
                    if let Some(siguiente) = iter_datos_entrenamiento.peek(){
                        self.palabras.push(Palabra::new(palabra, siguiente));
                    }
                    else{
                        self.palabras.push(Palabra::new(palabra,datos_entrenamiento.unicode_words().next().unwrap()));
                    }
                }
            }
            else{
                return;
            }
        }
    }

    fn predecir_siguiente(&self, palabra: &str) -> String{
        let mut prediccion_elegida = &Prediccion::new();
        if let Some(indice) = self.index(palabra){
            for prediccion in &self.palabras[indice].siguiente{
                if prediccion.repeticiones > prediccion_elegida.repeticiones{
                    prediccion_elegida = prediccion;
                }
                else if prediccion.repeticiones == prediccion_elegida.repeticiones{
                    match (prediccion.palabra.len()%2, prediccion_elegida.palabra.len()%2) {
                        (1, 0) => prediccion_elegida = prediccion,
                        (0, 1) => prediccion_elegida = prediccion_elegida,
                        _ => {
                            let lhs = prediccion.palabra.len();
                            let rhs = prediccion_elegida.palabra.len();
                            if lhs > rhs{
                                prediccion_elegida = prediccion;
                            }
                            else if rhs > lhs{
                                prediccion_elegida = prediccion_elegida;
                            }
                            else{
                                if prediccion_elegida.palabra > prediccion.palabra{
                                    prediccion_elegida = prediccion;
                                }
                                else{
                                    prediccion_elegida = prediccion_elegida;
                                }
                            }
                        },
                    }
                }
            }
        }
        else{
            for prediccion in &self.repeticiones_totales{
                if prediccion.repeticiones > prediccion_elegida.repeticiones{
                    prediccion_elegida = prediccion;
                }
                else if prediccion.repeticiones == prediccion_elegida.repeticiones{
                    match (prediccion.palabra.len()%2, prediccion_elegida.palabra.len()%2) {
                        (1, 0) => prediccion_elegida = prediccion,
                        (0, 1) => prediccion_elegida = prediccion_elegida,
                        (lhs, rhs) => {
                            if lhs > rhs{
                                prediccion_elegida = prediccion;
                            }
                            else if rhs > lhs{
                                prediccion_elegida = prediccion_elegida;
                            }
                            else{
                                if prediccion_elegida.palabra > prediccion.palabra{
                                    prediccion_elegida = prediccion;
                                }
                                else{
                                    prediccion_elegida = prediccion_elegida;
                                }
                            }
                        },
                    }
                }
            }
        }
        return  prediccion_elegida.palabra.clone();
    }

    fn predecir_siguientes_5(&self, palabra: &str) -> String{
        let mut salida = String::new();
        let mut ultima_palabra = palabra.to_string();
        salida += &ultima_palabra;
        for _ in 0..5{
            ultima_palabra = self.predecir_siguiente(&ultima_palabra);
            salida += " ";
            salida += &ultima_palabra;
        }

        return salida;
    }

    fn add_repeticion(&mut self, palabra: &str){
        if let Some(indice) = self.index_repeticion(palabra){
            self.repeticiones_totales[indice].repeticiones += 1;
        }
        else{
            self.repeticiones_totales.push(Prediccion{
                palabra: palabra.to_string(),
                repeticiones: 1,
            });
        }
    }

    fn index(&self, palabra: &str) -> Option<usize>{
        for indice in 0..self.palabras.len(){
            if self.palabras[indice].palabra == palabra{
                return Some(indice);
            }
        }
        return None;
    }

    fn index_repeticion(&self, palabra: &str) -> Option<usize>{
        for indice in 0..self.repeticiones_totales.len(){
            if self.repeticiones_totales[indice].palabra == palabra{
                return Some(indice);
            }
        }
        return None;
    }
}

fn main() {
    let ruta_entrenamiento = "Dia11.txt";
    let ruta_entrada = "Dia11_test.txt";
    let entrenamiento = fs::read_to_string(ruta_entrenamiento).expect("No se pudo abrir el archivo de datos de entrenamiento").to_lowercase().clean().replace("santa", "");
    let entrada = fs::read_to_string(ruta_entrada).expect("No se pudo abrir el archivo de datos de entrada").to_lowercase().clean();

    let mut modelo = Modelo::new();

    modelo.entrenar(&entrenamiento);

    let solucion = entrada.lines()
        .map(|palabra| modelo.predecir_siguientes_5(palabra))
        .collect::<Vec<String>>()
        .join(";");

    let _ = fs::write("CoreGPT.txt", solucion);
}

trait StringCleaner {
    fn clean(&self) -> Self;
}

impl StringCleaner for String {
    fn clean(&self) -> Self{
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
            .collect::<Vec<&str>>()
            .join("")
    }
}
