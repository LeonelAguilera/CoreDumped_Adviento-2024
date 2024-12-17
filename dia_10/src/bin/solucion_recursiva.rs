/*
Day 10 Aritmofobia

Un pobre CoreElfo se ha perdido en el departamento de MaTIC. Tiene un mapa con números, pero le da miedo ir a números inferiores al número en el que está. Ayúdale a encontrar el camino más corto que puede seguir para salir y llegar al CIC.

    Parte desde el 0 (Matic) y su destino es el 9 (CIC).
    Solo puede moverse a una casilla que tenga el mismo número o un número mayor por 1, de manera ortogonal.
    Encuentra la cantidad de movimientos que tiene que hacer para llegar a su destino en el menor número de pasos.

El input es el siguiente link

1111
0123
5554
6789 -> 11 pasos como mínimo (0-1-2-3-4-5-5-5-6-7-8-9)

¡No olvides bajar a nuestra sala bajo la rotonda si lo completas en las primeras 24h y llevate un chocolate o una golosina!
 * */

use std::{fs, sync::{Arc, Mutex}, thread};

const ID_DESTINO: u8 = 9;
const ID_INICIO: u8 = 0;

fn main() {
    let ruta = "Dia10.csv";
    let mapa = leer_mapa(ruta);
    //let mejor_solucion = Arc::new(Mutex::new(Box::new(mapa.len()*mapa[0].len())));
    let mejor_solucion = Arc::new(Mutex::new(Box::new(3000)));
    println!("{mejor_solucion:?}");
    let (x_inicio, y_inicio) = encontrar_punto_inicio(&mapa).expect("No se ha podido encontrar el punto de inicio");
    println!("La distancia más corta al destino es: {}", dar_siguiente_paso(mapa, x_inicio, y_inicio, Vec::new(), mejor_solucion).unwrap() - 1);
}

fn dar_siguiente_paso(mapa:  Vec<Vec<u8>>, x: usize, y: usize, mut camino: Vec<(usize, usize)>, mejor_solucion: Arc<Mutex<Box<usize>>>) -> Option<usize>{
    camino.push((x, y));
    //println!("{camino:?}");
    if camino.len() > {**mejor_solucion.lock().unwrap()}{ //Porfis no revientes mi stack
        return None;
    }
    if mapa[y][x] == ID_DESTINO{
        let mut mejor_solucion = mejor_solucion.lock().unwrap();
        println!("Se ha llegado al destino en: {} pasos\n{:?}\nMejorSolucion: {}\n\n", camino.len(), camino, mejor_solucion);
        if camino.len() < **mejor_solucion{
            *mejor_solucion = Box::new(camino.len());
        }
        return Some(camino.len());
    }

    let mut handles = Vec::new();
    //Mover arriba
    if camino.len() < 3{
        println!("Arriba en {}", camino.len());
    }
    let x_t = x.clone();
    let y_t = y.clone();
    let mapa_t = mapa.clone();
    let camino_t = camino.clone();
    let mejor_solucion_t = Arc::clone(&mejor_solucion);
    handles.push(thread::spawn(move||{
        if y_t <= 0{
            return None;
        }
        if (mapa_t[y_t-1][x_t] == mapa_t[y_t][x_t] || mapa_t[y_t-1][x_t] == (mapa_t[y_t][x_t] + 1)) && camino_t.contains(&(x_t,y_t-1)) == false{
            dar_siguiente_paso(mapa_t, x_t, y_t-1, camino_t, mejor_solucion_t)
        }
        else
        {
            return None;
        }
    }));
    //Mover derecha
    if camino.len() < 3{
        println!("Derecha en {}", camino.len());
    }
    let x_t = x.clone();
    let y_t = y.clone();
    let mapa_t = mapa.clone();
    let camino_t = camino.clone();
    let mejor_solucion_t = Arc::clone(&mejor_solucion);

    handles.push(thread::spawn(move||{
        if x_t >= (mapa_t[0].len()-1){
            return None;
        }
        if (mapa_t[y_t][x+1] == mapa_t[y_t][x_t] || mapa_t[y_t][x_t+1] == (mapa_t[y_t][x_t] + 1)) && camino_t.contains(&(x_t+1,y_t)) == false{
            dar_siguiente_paso(mapa_t, x_t+1, y_t, camino_t, mejor_solucion_t)
        }
        else
        {
            return None;
        }
    }));
    //Mover abajo
    if camino.len() < 3{
        println!("Abajo en {}", camino.len());
    }
    let x_t = x.clone();
    let y_t = y.clone();
    let mapa_t = mapa.clone();
    let camino_t = camino.clone();
    let mejor_solucion_t = Arc::clone(&mejor_solucion);

    handles.push(thread::spawn(move||{
        if y_t >= (mapa_t.len()-1){
            return None;
        }
        if (mapa_t[y_t+1][x_t] == mapa_t[y_t][x_t] || mapa_t[y_t+1][x_t] == (mapa_t[y_t][x_t] + 1)) && camino_t.contains(&(x_t,y_t+1)) == false{
            dar_siguiente_paso(mapa_t, x_t, y_t+1, camino_t, mejor_solucion_t)
        }
        else
        {
            return None;
        }
    }));
    //Mover izquierda
    if camino.len() < 3{
        println!("Izquierda en {}", camino.len());
    }
    let x_t = x.clone();
    let y_t = y.clone();
    let mapa_t = mapa.clone();
    let camino_t = camino.clone();
    let mejor_solucion_t = Arc::clone(&mejor_solucion);

    handles.push(thread::spawn(move||{
        if x_t <= 0{
            return None;
        }
        if (mapa_t[y_t][x_t-1] == mapa_t[y_t][x_t] || mapa_t[y_t][x_t-1] == (mapa_t[y_t][x_t] + 1)) && camino_t.contains(&(x_t-1,y_t)) == false{
            dar_siguiente_paso(mapa_t, x_t-1, y_t, camino_t, mejor_solucion_t)
        }
        else
        {
            return None;
        }
    }));

    let mut camino_mas_corto = None;
    for handle in handles{
        if let Some(longitud_camino) = handle.join().unwrap(){
            if let Some(cmc) = camino_mas_corto{
                if longitud_camino < cmc{
                    camino_mas_corto = Some(longitud_camino);
                }
            }
            else{
                camino_mas_corto = Some(longitud_camino);
            }
        }
    }
    return camino_mas_corto;
}

fn encontrar_punto_inicio(mapa: &Vec<Vec<u8>>) -> Option<(usize, usize)>{
    for y in 0..mapa.len(){
        for x in 0..mapa[0].len(){
            if mapa[y][x] == ID_INICIO{
                return Some((x, y));
            }
        }
    }
    return None;
}

fn leer_mapa(ruta: &str) -> Vec<Vec<u8>>{
    let mut mapa = Vec::new();
    let archivo = fs::read_to_string(ruta).expect("No se pudo abrir el archivo con el mapa");
    for fila in archivo.lines(){
        mapa.push(fila.split(",").map(|caracter| caracter.parse().expect("Caracteres indebidos en el archivo")).collect());
    }

    return mapa;
}
