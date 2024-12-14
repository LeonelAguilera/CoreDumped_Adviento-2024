use core::panic;
use std::{char, fs, str::Chars};

#[allow(non_camel_case_types)]
struct trineOS{
    registros: [u8; 26],
}

impl trineOS{
    fn new() -> Self{
        Self {registros: [0; 26] }
    }
    fn ejecutar(&mut self, instruccion: u32){
        let inst = ((instruccion & 0b00000000111111110000000000000000) >> 16) as u8;
        let op_a = ((instruccion & 0b00000000000000001111111100000000) >> 8) as u8;
        let op_b = ((instruccion & 0b00000000000000000000000011111111) >> 0) as u8;

        if contar_unos(inst) > 4{
            if inst & 0b00000001 == 0{
                self.mov(op_a, op_b);
            }
            else{
                self.add(op_a, op_b);
            }
        }
        else if contar_unos(inst) < 4{
            if inst & 0b00000001 == 0{
                self.sub(op_a, op_b);
            }
            else{
                self.nop(op_a, op_b);
            }
        }
        else{
            self.set(op_a, op_b);
        }
    }

    fn add(&mut self, op_a: u8, op_b: u8){
        if (op_a >= 'A' as u8) && (op_a <= 'Z' as u8) && (op_b >= 'A' as u8) && (op_b <= 'Z' as u8){
            self.registros[(op_a - 'A' as u8) as usize]= self.registros[(op_a - 'A' as u8) as usize] + self.registros[(op_b - 'A' as u8) as usize];
        }
        else{
            panic!("Se ha intentado hacer add con un número");
        }
    }
    fn sub(&mut self, op_a: u8, op_b: u8){
        if (op_a >= 'A' as u8) && (op_a <= 'Z' as u8) && (op_b >= 'A' as u8) && (op_b <= 'Z' as u8){
            self.registros[(op_a - 'A' as u8) as usize]= self.registros[(op_a - 'A' as u8) as usize] - self.registros[(op_b - 'A' as u8) as usize];
        }
        else{
            panic!("Se ha intentado hacer sub con un número");
        }
    }
    fn mov(&mut self, op_a: u8, op_b: u8){
        if (op_a >= 'A' as u8) && (op_a <= 'Z' as u8) && (op_b >= 'A' as u8) && (op_b <= 'Z' as u8){
            self.registros[(op_a - 'A' as u8) as usize]= self.registros[(op_b - 'A' as u8) as usize];
        }
        else{
            panic!("Se ha intentado hacer mov con un número");
        }
    }
    fn set(&mut self, op_a: u8, op_b: u8){
        if (op_a >= 'A' as u8) && (op_a <= 'Z' as u8){
            self.registros[(op_a - 'A' as u8) as usize] = op_b;
        }
        else{
            panic!("Se ha intentado hacer set en un registro inválido");
        }
    }
    fn nop(&mut self, _op_a: u8, _op_b: u8){
    }

    fn to_string(&self) -> String{
        let mut salida = String::new();
        for i in 1..26{
            for j in 0..self.registros.len(){
                if self.registros[j] == i{
                    salida.push(((j as u8) + ('A' as u8)) as char);
                }
            }
        }

        return  salida;
    }
}

fn contar_unos(numero: u8) -> u8{
    let mut numero_unos = 0;
    let mut test_num = numero;
    for _ in 0..8{
        numero_unos += test_num & 0b00000001;
        test_num = test_num >> 1;
    }
    return numero_unos;
}

fn main() {
    let mut trineo = trineOS::new();
    let ruta = "Dia9.txt";

    let programa = fs::read_to_string(ruta).expect("No se pudo abrir el archivo").trim().to_string();
    let programa = chars_to_bin(programa.chars());

    for instruccion in programa{
        trineo.ejecutar(instruccion);
    }

    println!("La palabra secreta es: {}", trineo.to_string());
}

fn chars_to_bin(texto: Chars) -> Vec<u32>{
    let mut code = 0;
    let mut solucion = Vec::new();
    let mut bit_leidos = 0;

    for caracter in texto{
        if caracter == '1'{
            code = (code << 1) | 0x01;
        }
        else if caracter == '0'{
            code = code << 1;
        }
        else{
            panic!("Caracter inválido en el programa");
        }
        bit_leidos += 1;

        if bit_leidos == 24 {
            bit_leidos = 0;
            solucion.push(code);
            code = 0;
        }
    }

    return solucion;
}
