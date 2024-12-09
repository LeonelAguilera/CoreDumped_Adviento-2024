use std::{fs, str::Lines};

fn main() {
    let ruta = "Dia5.txt";
    let fichero = fs::read_to_string(ruta).expect("No fue posible abrir el fichero");
    let mut fichero = fichero.lines();
    let mut saco = vec_from_num(fichero.next().unwrap());
    let regalos = tuplas_a_columnas(fichero);

    for regalo in regalos{
        saco = simulador_paquete(saco, regalo).unwrap();
    }

    println!("Las alturas son: ");
    for columna in saco{
        print!("{}", columna%10);
    }
}

fn tuplas_a_columnas(fichero: Lines) -> Vec<usize>{
    let mut output_vec = Vec::new();
    for linea in fichero{
        let (repeticiones, numero) = parser_tuplas(linea);
        for i in 0..repeticiones{
            output_vec.push(numero);
        }
    }

    return output_vec;
}

fn parser_tuplas(texto: &str) -> (i32, usize){
    let texto = texto.replace(&['(', ')'], "");
    let texto: Vec<&str> = texto.split(", ").collect();
    return (texto[0].parse().unwrap(), texto[1].parse().unwrap());
}

fn vec_from_num(num: &str) -> Vec<u32>{
    let mut output_vec = Vec::new();
    for caracter in num.chars(){
        match caracter.to_digit(10){
            Some(num) => output_vec.push(num),
            None    => continue,
        }
    }

    return output_vec;
}

fn simulador_paquete(mut estado: Vec<u32>, columna: usize) -> Option<Vec<u32>>{
    if columna >= estado.len(){
        return None;
    }

    if columna != 0{
        if estado[columna] >= estado[columna - 1]{
            estado[columna - 1] += 1;
            return  Some(estado);
        }
    }

    if columna != estado.len() - 1{
        if estado[columna] >= estado[columna + 1]{
            estado[columna + 1] += 1;
            return  Some(estado);
        }
    }

    estado[columna] += 1;
    return Some(estado);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simulador_1(){
        assert_eq!(simulador_paquete(vec![5, 3, 3, 4], 0), Some(vec![5, 4, 3, 4]));
    }
    #[test]
    fn simulador_2(){
        assert_eq!(simulador_paquete(vec![5, 4, 3, 4], 2), Some(vec![5, 4, 4, 4]));
    }
    #[test]
    fn simulador_3(){
        assert_eq!(simulador_paquete(vec![5, 4, 4, 4], 2), Some(vec![5, 5, 4, 4]));
    }
    #[test]
    fn vec_from_num_1(){
        assert_eq!(vec_from_num("1234"), vec![1, 2, 3, 4]);
    }
    #[test]
    fn parser_tuplas_1(){
        assert_eq!(parser_tuplas("(1, 12)"), (1, 12));
    }
    #[test]
    fn tuplas_a_columnas_1(){
        assert_eq!(tuplas_a_columnas("(1, 12)\n(2, 128)\n(3, 256)".to_string().lines()), vec![12, 128, 128, 256, 256, 256]);
    }
    #[test]
    fn tuplas_a_columnas_2(){
        assert_eq!(tuplas_a_columnas("(1, 12)\n(2, 128)\n(0, 33)\n(3, 256)".to_string().lines()), vec![12, 128, 128, 256, 256, 256]);
    }
}
