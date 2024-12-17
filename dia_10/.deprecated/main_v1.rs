#[derive(Debug)]
struct Node{
    pos: (usize, usize),
    val: u8,
    steps: usize,
    next: Vec<Node>,
}

impl Node {
    fn new(pos: (usize, usize), mapa: &mut Vec<Vec<(u8, bool)>>, steps: usize) -> Self{
        let mut nodo = Self{
            pos,
            val: mapa[pos.1][pos.0].0,
            steps,
            next: Vec::new(),
        };
        //println!("{:#?}\n{}\n\n", nodo, mapa[pos.1][pos.0].1);
        if mapa[pos.1][pos.0].1 == true{
            mapa[pos.1][pos.0]. 1 = false;
            nodo.get_allowed_nodes(mapa);
        }
        return nodo;
    }
    fn get_allowed_nodes(&mut self, mapa: &mut Vec<Vec<(u8, bool)>>){
        //Arriba
        if self.pos.1 > 0{
            let new_val = mapa[self.pos.1 - 1][self.pos.0];
            if self.val ==  new_val.0 || self.val + 1 == new_val.0{
                self.next.push(Node::new((self.pos.1 - 1, self.pos.0), mapa, self.steps+1));
            }
        }
        //Derecha
        if self.pos.0 < mapa.len() - 1{
            let new_val = mapa[self.pos.1][self.pos.0 + 1];
            if self.val ==  new_val.0 || self.val + 1 == new_val.0{
                self.next.push(Node::new((self.pos.0 + 1, self.pos.1), mapa, self.steps + 1));
            }
        }
        //Abajo
        if self.pos.1 < mapa.len() - 1{
            let new_val = mapa[self.pos.1 + 1][self.pos.0];
            if self.val ==  new_val.0 || self.val + 1 == new_val.0{
                self.next.push(Node::new((self.pos.0, self.pos.1 + 1), mapa, self.steps + 1));
            }
        }
        //Arriba
        if self.pos.0 > 0{
            let new_val = mapa[self.pos.1][self.pos.0 - 1];
            if self.val ==  new_val.0 || self.val + 1 == new_val.0{
                self.next.push(Node::new((self.pos.0 - 1, self.pos.1), mapa, self.steps + 1));
            }
        }
    }
}

use std::fs;

const ID_DESTINO: u8 = 9;
const ID_INICIO: u8 = 0;

fn main(){
    let ruta = "Dia10_test.csv";
    let mut mapa = leer_mapa(ruta);
    let grafo = build_node_graph(&mut mapa);

    println!("{:#?}", grafo);
}

fn build_node_graph(mapa: &mut Vec<Vec<(u8, bool)>>) -> Node{
    let (x_inicio, y_inicio) = encontrar_punto_inicio(&mapa).expect("No se pudo encontrar el punto de inicio");
    Node::new((x_inicio, y_inicio), mapa, 0)
}

fn encontrar_punto_inicio(mapa: &Vec<Vec<(u8, bool)>>) -> Option<(usize, usize)>{
    for y in 0..mapa.len(){
        for x in 0..mapa[0].len(){
            if mapa[y][x].0 == ID_INICIO{
                return Some((x, y));
            }
        }
    }
    return None;
}

fn leer_mapa(ruta: &str) -> Vec<Vec<(u8, bool)>>{
    let mut mapa = Vec::new();
    let archivo = fs::read_to_string(ruta).expect("No se pudo abrir el archivo con el mapa");
    for fila in archivo.lines(){
        mapa.push(fila.split(",").map(|caracter| (caracter.parse().expect("Caracteres indebidos en el archivo"), true)).collect());
    }

    return mapa;
}
