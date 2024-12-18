/*
#Day 12 Llegó el día

Muchas gracias lo primero a todos y cada uno de vosotr@s por participar en este proyecto, ahora como último día os tenemos un reto divertido.

No sé si conoceís Brainfuck, es un lenguaje único. Es un lenguaje turing completo y solamente tiene 8 operadores y te permite crear casi cualquier programa que te puedas imaginar.

Podéis buscar el tutorial de Brainfuck pero nada de buscar traductores que lo hagan por vosotros, tenéis hasta el viernes a las 00:01, disfrutad de mi receta : ) link

¡Felices Fiestas! Enhorabuena por vuestro duro trabajo. Recordar que el viernes es la recogida de premios y si estáis en el top 10 bajad a la sala de Core Dumped a por vuestros premios.

¡No olvides bajar a nuestra sala bajo la rotonda si lo completas en las primeras 24h y llevate un chocolate o una golosina!
 */

use std::fs;

const RAM_SIZE: usize = 30000;

struct Maquina{
    memoria: [u8; RAM_SIZE],
    puntero: usize,
    pc: usize,
    stack: Vec<usize>,
    rom: Vec<char>,
    stdout: String,
}

impl Maquina {
    fn new(programa: Vec<char>) -> Self{
        Maquina { memoria: [0; RAM_SIZE], puntero: 0, pc: 0, stack: Vec::new(), rom: programa, stdout: String::new()}
    }

    fn ejecutar_programa(&mut self) -> String{
        while self.pc < self.rom.len(){
            let instruccion_actual = self.rom[self.pc];
            //println!("{}: '{}'&{} - {:?}", self.pc, self.puntero, instruccion_actual, self.memoria);
            self.ejecutar_instruccion(instruccion_actual);
        }
        return self.stdout.clone();
    }

    fn ejecutar_instruccion(&mut self, instruccion: char){
        match instruccion {
            '>' => self.incrementar_pc(),
            '<' => self.decrementar_pc(),
            '+' => self.incrementar_dato(),
            '-' => self.decrementar_dato(),
            '.' => self.escribir_caracter(),
            ',' => self.leer_caracter(),
            '[' => self.inicio_bucle(),
            ']' => self.fin_bucle(),
            _   => self.pc += 1,//panic!("Instrucción inválida en el programa"),
        }
    }

    fn incrementar_pc(&mut self){
        self.puntero += 1;
        if self.puntero == RAM_SIZE{
            self.puntero = 0;
        }
        self.pc += 1;
    }
    fn decrementar_pc(&mut self){
        if self.puntero > 0{
            self.puntero -= 1;
        }
        else{
            self.puntero = RAM_SIZE-1;
        }
        self.pc += 1;
    }
    fn incrementar_dato(&mut self){
        if self.memoria[self.puntero] == u8::MAX{
            self.memoria[self.puntero] = 0;
        }
        else{
         self.memoria[self.puntero] += 1;
        }
        self.pc += 1;
    }
    fn decrementar_dato(&mut self){
        if self.memoria[self.puntero] == 0{
            self.memoria[self.puntero] = u8::MAX;
        }
        else{
         self.memoria[self.puntero] -= 1;
        }
        self.pc += 1;
    }
    fn escribir_caracter(&mut self){
        print!("{}", self.memoria[self.puntero] as char);
        self.stdout.push(self.memoria[self.puntero] as char);
        self.pc += 1;
    }
    fn leer_caracter(&mut self){
        unimplemented!();
    }
    fn inicio_bucle(&mut self){
        if self.memoria[self.puntero] == 0{
            while self.rom[self.pc] != ']'{
                self.pc += 1;
                if self.pc == self.rom.len(){
                    self.pc = 0;
                }
            }
            self.pc += 1;
        }
        else{
            self.stack.push(self.pc + 1);
            self.pc += 1;
        }
    }
    fn fin_bucle(&mut self){
        if self.memoria[self.puntero] != 0{
            self.pc = self.stack[self.stack.len() - 1];
        }
        else{
            self.pc += 1;
            let _ = self.stack.pop();
        }

    }
}

fn main() {
    let ruta = "Dia12.txt";
    let programa = fs::read_to_string(ruta).expect("No se pudo abrir el archivo");

    let mut intérprete = Maquina::new(programa.chars().collect());
    let respuesta = intérprete.ejecutar_programa();

    let _ = fs::write("Receta.txt", respuesta);
}
