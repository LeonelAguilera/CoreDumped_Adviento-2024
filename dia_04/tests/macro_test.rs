use core::panic;

use dia_04::generar_diccionario;

#[test]
fn macro_test_1(){
    const EXPECTED: [[&str; 2]; 9]= [
        ["", ""],
        ["", ""],
        ["", ""],
        ["", ""],
        ["arbol", "comer"],
        ["comida", "patata"],
        ["", ""],
        ["", ""],
        ["aparicion", ""]
    ];
    generar_diccionario!();
    assert_eq!(DICCIONARIO, EXPECTED);
}
