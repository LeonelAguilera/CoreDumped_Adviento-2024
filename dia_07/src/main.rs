/*
Day 7 Core Dumped en el CoreTrineo

CoreClaus está probando su trineo antes de Navidad. Sin embargo el sistema operativo del trineo ha fallado y ha devuelto un segmentation fault (Core Dumped). Has localizado que el problema es un bucle infinito en el código de inicialización del trineo. El código esta escrito en un tipo de ensamblador muy antiguo y mágico. Has conseguido un manual polvoriento con la siguiente información:

    acc aumenta o disminuye una única variable global llamadoa acumulador por el valor dado en el argumento. Por ejemplo, acc +7 aumentaría el acumulador en 7. El acumulador se inicializa en 0. Después de una instrucción acc, se ejecuta la instrucción inmediatamente inferior.
    jmp salta a una nueva instrucción relativa a sí misma. La siguiente instrucción a ejecutar se encuentra usando el argumento como un offset desde la instrucción jmp; por ejemplo jmp +1 saltaría a la siguiente instrucción, jmp +2 continuaría a la instrucción 2 líneas por debajo, y jmp -20 haría que la instrucción 20 líneas por encima fuera la siguiente en ejecutarse.
    nop significa No Operation - no hace nada. La instrucción inmediatamente inferior se ejecuta a continuación.

El código está almacenado aquí. No sabes exactamente donde ocurre el bucle, para averiguarlo, quieres saber cual es el valor del acumulador justo antes de que se ejecute una instrucción por segunda vez.

¡No olvides bajar a nuestra sala bajo la rotonda si lo completas en las primeras 24h y llevate un chocolate o una golosina!
 *
 * */

use core::panic;
use std::fs;

#[allow(non_camel_case_types)]
struct trineOS{
    acumulador: isize,
    pc: isize,
}

impl trineOS {
    fn new() -> Self{
        Self { acumulador: 0, pc: 0}
    }

    fn execute(&mut self, instruccion: &str){
        match &instruccion[0..3] {
            "acc" => self.acc(instruccion[4..].parse().expect("Número invalido en acc")),
            "jmp" => self.jmp(instruccion[4..].parse().expect("Número invalido en jmp")),
            "nop" => self.nop(),
            _     => panic!("Instrucción inválida"),
        }
    }

    fn acc(&mut self, value: isize){
        self.acumulador += value;
        self.pc += 1;
    }

    fn jmp(&mut self, value: isize){
        self.pc += value;
    }

    fn nop(&mut self) {
        self.pc += 1;
    }
}

fn main() {
    let ruta = "Dia7.txt";
    let programa = fs::read_to_string(ruta).expect("No se pudo abrir el archivo");
    let programa: Vec<&str> = programa.lines().collect();

    let mut trineo = trineOS::new();
    let mut instrucciones_visitadas = vec![0; programa.len()];

    instrucciones_visitadas[trineo.pc as usize] += 1;
    while instrucciones_visitadas.contains(&2) == false {
        trineo.execute(programa[trineo.pc as usize]);
        instrucciones_visitadas[trineo.pc as usize] += 1;
    }

    println!("Valor acumulador: {}", trineo.acumulador);
}


