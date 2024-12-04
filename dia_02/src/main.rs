/*Day 2 En la búsqueda del componente perdido

Después del asalto a la rotonda, los CoreElfos tienen que construir regalos muy rápido, en las prisas han perdido los componentes de los regalos. Cada regalo tiene un componente que se ha perdido, pero no saben cuál es.

Pero los CoreElfos cometen errores muy matemáticos, sabemos que la regla con la que han perdido los componentes es la siguiente:

    Cada componente es una letra, y tiene un valor (a=1, b=2, c=3, ..., z=26, A = 27, B = 28, ..., Z = 52).
    Cada regalo es una mezcla de números y letras.
    El primer número de cada regalo es la posición relativa del componente perdido (dado "a2bcda", el componente perdido será 'c' porque es el segundo componente a partir del cáracter '2').
    En los casos de juguetes muy complejos, el dígito puede apuntar a otro dígito, que a su vez puede apuntar a más dígitos o a la letra que buscamos (dado "a2v3e2gfh", el componente perdido será 'g' porque es el tercer componente a partir del cáracter '3', que es el segundo a partir del cáracter '2').

Dada esta lista de regalos, encuentra la suma de los valores de todos los componentes perdidos.
 * */
use std::fs;

fn main() {
    let filepath = "Dia2.txt";

    let strings = fs::read_to_string(filepath).expect("nanai");
    let suma = get_suma_from_strings(strings);

    println!("Suma total: {suma}");
}

fn get_suma_from_strings(strings: String) -> usize{
    return strings
        .lines()
        .fold(0, |acc, componente| -> usize {
            acc + (component2num(componente).expect("No se pudo obtener el número del componente") as usize)
        }
        );
}

fn component2num(component: &str) -> Option<u8>{
    let mut characters = component.chars();
    let offset = loop{
        let character = characters.next().expect("No hay números en este componente");
        if character.is_ascii_digit(){
            break character
                .to_digit(10)
                .expect("De alguna forma este dígito no es un número")
                .try_into()
                .expect("No se puede transformar u32 en usize");
        }
    };
    return substring2num(characters.as_str(), offset);
}

fn substring2num(substring: &str, offset: usize) -> Option<u8>{
    println!("Substring: {substring}, offset: {offset}");
    let mut substring = substring.chars();
    let selected_char = substring.nth(offset - 1).expect("No existe el caracter seleccionado");

    if selected_char.is_ascii_alphabetic(){
        return char2num(selected_char);
    }
    else if selected_char.is_ascii_digit(){
        let offset = selected_char
            .to_digit(10)
            .expect("De alguna forma este dígito tampoco es un número")
            .try_into()
            .expect("No se puede transformar u32 en usize");
        return substring2num(substring.as_str(), offset);
    }
    else{
        return None;
    }
}

fn char2num(character: char) -> Option<u8>{
    if character.is_ascii_lowercase(){
        let number = (character as u8) - ('a' as u8) + 1;
        return Some(number);
    }
    else if character.is_ascii_uppercase(){
        let number = (character as u8) - ('A' as u8) + 27;
        return Some(number);
    }
    else{
        return None;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn char2num_case_1(){
        assert_eq!(char2num('a'), Some(1));
    }
    #[test]
    fn char2num_case_2(){
        assert_eq!(char2num('z'), Some(26));
    }
    #[test]
    fn char2num_case_3(){
        assert_eq!(char2num('A'), Some(27));
    }
    #[test]
    fn char2num_case_4(){
        assert_eq!(char2num('Z'), Some(52));
    }
    #[test]
    fn char2num_case_5(){
        assert_eq!(char2num('3'), None);
    }
    #[test]
    fn substring2num_1(){
        assert_eq!(substring2num("bcda", 2), char2num('c'));
    }
    #[test]
    fn substring2num_2(){
        assert_eq!(substring2num("v3e2gfh", 2), char2num('g'));
    }
    #[test]
    fn component2num_1(){
        assert_eq!(component2num("a2bcda"), char2num('c'));
    }
    #[test]
    fn component2num_2(){
        assert_eq!(component2num("a2v3e2gfh"), char2num('g'));
    }
    #[test]
    fn component2num_3(){
        assert_eq!(component2num("XJo7YRRcvUZhB7aDXeg2YQlztP3p"), char2num('Z'));
    }
    #[test]
    fn component2num_4(){
        assert_eq!(component2num("TuQeZ5J3vn16XGewc3D26NgrPa5RR58xQgTxfW"), char2num('x'));
    }
    #[test]
    fn component2num_5(){
        assert_eq!(component2num("U3jA2Z7zoQl3P5SkWT14h7AZib0Rk51"), char2num('Z'));
    }
    #[test]
    fn suma_1(){
        assert_eq!(get_suma_from_strings(String::from("XJo7YRRcvUZhB7aDXeg2YQlztP3p
TuQeZ5J3vn16XGewc3D26NgrPa5RR58xQgTxfW
U3jA2Z7zoQl3P5SkWT14h7AZib0Rk51")), (char2num('Z').unwrap() + char2num('x').unwrap() + char2num('Z').unwrap()).into());
    }
}
