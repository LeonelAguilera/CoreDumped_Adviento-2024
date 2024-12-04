use std::{fs, u32};

fn main() {
    let filepath = "Dia3.txt";

    let text = fs::read_to_string(filepath).expect("El archivo no existe o no se puede abrir");
    let matriz = string_to_matrix(text);

    println!("{}", suma_de_minas(matriz));
}

fn suma_de_minas(matrix: Vec<Vec<u32>>) -> u32{
    let mut acumulador = 0;
    for x in 0..matrix[0].len(){
        for y in 0..matrix.len(){
            match hay_mina(&matrix, x, y) {
                Some(x) => acumulador+=x,
                None => {},
            }
        }
    }

    return acumulador;
}

fn hay_mina(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32>{
    if x == 0 || y == 0 || x >= matrix[0].len()-1 || y >= matrix.len()-1{
        return None;
    }

    let suma = matrix[y-1][x-1] + matrix[y-1][x] + matrix[y-1][x+1] + matrix[y][x-1] + matrix[y][x+1] + matrix[y+1][x-1] + matrix[y+1][x] + matrix[y+1][x+1];

    if suma > 42{
        return Some(matrix[y][x]);
    }
    else{
        return None;
    }
}

fn string_to_matrix(text: String) -> Vec<Vec<u32>>{
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut i = 0;
    for line in text.lines(){
        matrix.push(Vec::new());
        for caracter in line.chars(){
            if caracter.is_ascii_digit(){
                matrix[i].push(caracter.to_digit(10).expect("Hay un carcter que no es un número en el archivo"));
            }
        }
        i += 1;
    }

    return matrix;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn string_to_matrix_1(){
        let text = String::from("012
                                345
                                678");
        let test_matrix: Vec<Vec<u32>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let debug_matrix = string_to_matrix(text);
        println!("{:?}", debug_matrix);
        println!("{:?}", test_matrix);
        assert_eq!(debug_matrix, test_matrix);
    }
    #[test]
    fn hay_mina_1(){
        let test_matrix: Vec<Vec<u32>> = vec![vec![5, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(hay_mina(&test_matrix, 1, 1), Some(5));
    }
    #[test]
    fn hay_mina_2(){
        let test_matrix: Vec<Vec<u32>> = vec![vec![5, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(hay_mina(&test_matrix, 0, 0), None);
    }
    #[test]
    fn hay_mina_3(){
        let test_matrix: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(hay_mina(&test_matrix, 1, 1), None);
    }
    #[test]
    fn suma_de_minas_1(){
        let test_matrix: Vec<Vec<u32>> = vec![vec![1, 2, 3, 2], vec![4, 5, 6, 7], vec![7, 8, 9, 8], vec![1, 2, 3, 4]];
        /*
         * 1 2 3 2
         * 4 5 6 7
         * 7 8 9 8
         * 1 2 3 4
         -> 40, 44, 37, 43 => 15
        */

        assert_eq!(suma_de_minas(test_matrix), 15);
    }
}
