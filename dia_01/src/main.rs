/*
 *TASK:

¡Se acerca la navidad! 🎄🎅🏼🎁 CoreClaus ha dejado regalos en la rotonda, un ser de las Neveras que odia la diversión y ama las convocatorias extraordinarias, ha hecho de las suyas (los ha robado [es muy pillín]).

Nuestros investigadores han encontrado una serie de pistas que nos llevarán a la identidad del ladrón. La primera pista es la siguiente:

    Hemos encontrado un pelo colgado del árbol de Navidad. Lo hemos llevado a un laboratorio para descifrar su código genético y hemos obtenido la siguiente secuencia de nucleótidos: Fichero.

Tenemos un sospechoso que ha sido visto en la zona, con un poco de maña hemos conseguido obtener un trozo de su ADN. La secuencia de nucleótidos obtenida es la siguiente: "CGGTAC".

Encuentra la cantidad de veces que aparece esta secuencia en el ADN del pelo.

Ejemplo:

"CGGTACCTTGACA" -> 1
"ACACACA" -> 0
"TTACGACGGTACTGAACGGTACTGA" -> 2
 
 * */

use std::fs;

fn main() {
    let filepath = "dia1.txt";
    let secuencia = "CGGTAC";

    let file_contents = fs::read_to_string(filepath).expect("Ha ocurrido un error intentando abrir el fichero");

    let num_apariciones = contador_apariciones(&file_contents, secuencia);

    println!("Se ha encontrado la secuencia \"{secuencia}\" {num_apariciones} veces");
}

fn contador_apariciones(lhs: &str, cadena_buscada: &str) -> usize{
    let mut apariciones = 0;
    let mut i = 0;
    while i <= (lhs.len()-cadena_buscada.len()){
        let slice = &lhs[i..i+cadena_buscada.len()];
        if  slice == cadena_buscada{
            apariciones += 1;
            i += cadena_buscada.len(); //Saltar la secuencia una vez encontrada
        }
        else{
            i += 1;
        }
    }

    return apariciones;
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_sequence_1(){
        let secuencia = "CGGTAC";
        let cadena = "CGGTACCTTGACA";

        let num_apariciones = contador_apariciones(cadena, secuencia);

        assert_eq!(num_apariciones, 1);
    }
    #[test]
    fn test_sequence_2(){
        let secuencia = "CGGTAC";
        let cadena = "ACACACA";

        let num_apariciones = contador_apariciones(cadena, secuencia);

        assert_eq!(num_apariciones, 0);
    }
    #[test]
    fn test_sequence_3(){
        let secuencia = "CGGTAC";
        let cadena = "TTACGACGGTACTGAACGGTACTGA";

        let num_apariciones = contador_apariciones(cadena, secuencia);

        assert_eq!(num_apariciones, 2);
    }
    #[test]
    fn test_sequence_4(){
        let secuencia = "CGGTAC";
        let cadena = "TTACGACGGTACTGAACGGTACTGACGGTAC";

        let num_apariciones = contador_apariciones(cadena, secuencia);

        assert_eq!(num_apariciones, 3);
    }
}
