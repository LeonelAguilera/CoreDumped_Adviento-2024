use std::{cell::RefCell, fmt::Display, fs, rc::Rc};

use queues::{IsQueue, Queue};

#[derive(Debug)]
struct Node{
    pos: (usize, usize),
    val: u8,
    steps: Option<usize>,
    next: Vec<Rc<RefCell<Node>>>,
}

impl Display for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodo{{\n\tpos: ({}, {})\n\tval: {}\n\tsteps: {}\n\tvecinos: [{}]\n}}",
        self.pos.0,
        self.pos.1,
        self.val,
        {
            match self.steps{
                Some(pasos) => pasos,
                _ => 0,
            }
        },
        {
            self.next.iter()
                .map(|vecino| {
                    let vecino_t = vecino.borrow();
                    format!("({}, {})", vecino_t.pos.0, vecino_t.pos.1)
                })
                .collect::<Vec<String>>()
                .join(", ")
        }
        )
    }
}

impl Node {
    fn new(pos: (usize, usize), mapa: &Vec<Vec<u8>>, steps: Option<usize>) -> Self{
        Self{
            pos,
            val: mapa[pos.1][pos.0],
            steps,
            next: Vec::new(),
        }
    }
    fn get_allowed_nodes(&mut self, grafo: &Grafo){
        //Arriba
        if self.pos.1 > 0{
            let new_val = grafo.nodos[self.pos.1 - 1][self.pos.0].borrow().val; 
            if self.val ==  new_val || self.val + 1 == new_val{
                self.next.push(Rc::clone(&grafo.nodos[self.pos.1 - 1][self.pos.0]));
            }
        }
        //Derecha
        if self.pos.0 < grafo.nodos.len() - 1{
            let new_val = grafo.nodos[self.pos.1][self.pos.0 + 1].borrow().val;
            if self.val ==  new_val || self.val + 1 == new_val{
                self.next.push(Rc::clone(&grafo.nodos[self.pos.1][self.pos.0 + 1]));
            }
        }
        //Abajo
        if self.pos.1 < grafo.nodos.len() - 1{
            let new_val = grafo.nodos[self.pos.1 + 1][self.pos.0].borrow().val;
            if self.val ==  new_val || self.val + 1 == new_val{
                self.next.push(Rc::clone(&grafo.nodos[self.pos.1 + 1][self.pos.0]));
            }
        }
        //Izquierda
        if self.pos.0 > 0{
            let new_val = grafo.nodos[self.pos.1][self.pos.0 - 1].borrow().val;
            if self.val ==  new_val || self.val + 1 == new_val{
                self.next.push(Rc::clone(&grafo.nodos[self.pos.1][self.pos.0 - 1]));
            }
        }
    }
    


    fn es_inicio(&self) -> bool{
        if self.val == ID_INICIO{
            return true;
        }
        else{
            return false;
        }
    }
    fn es_destino(&self) -> bool{
        if self.val == ID_DESTINO{
            return true;
        }
        else{
            return false;
        }
    }
}

#[derive(Debug)]
struct Grafo{
    nodos: Vec<Vec<Rc<RefCell<Node>>>>,
}
impl  Grafo{
    fn new(mapa: &Vec<Vec<u8>>) -> Self{
        let mut grafo = Vec::new();
        println!("\tCreando nodos del grafo");
        for y in 0..mapa.len(){
            grafo.push(Vec::new());
            for x in 0..mapa[0].len(){
                grafo[y].push(Rc::new(RefCell::new(Node::new((x, y), mapa, None))));
            }
        }
        let n_nodos = grafo.len()*grafo[0].len();

        println!("\tSe han creado un total de {} nodos.", n_nodos);
        /*
        println!("\tCreando conexiones entre los nodos");
        for y in 0..grafo.len(){
            for x in 0..grafo[0].len(){
                let param = grafo.clone();
                grafo[y][x].borrow_mut().get_allowed_nodes(param);
            }
            println!("\t\tCreadas conexiones de {} nodos", y*n_nodos/grafo.len());
        }*/
        return Grafo{nodos: grafo};
    }
    fn get_inicio(&self) -> Rc<RefCell<Node>>{
        let (x_inicio, y_inicio) = self.encontrar_coordenadas_inicio().expect("No se pudo encontrar el punto de inicio");
        return Rc::clone(&self.nodos[y_inicio][x_inicio]);
    }
    fn encontrar_coordenadas_inicio(&self) -> Option<(usize, usize)>{
        for y in 0..self.nodos.len(){
            for x in 0..self.nodos[0].len(){
                if self.nodos[y][x].borrow().es_inicio(){
                    return Some((x, y));
                }
            }
        }
        return None;
    }
}


const ID_DESTINO: u8 = 9;
const ID_INICIO: u8 = 0;

fn main(){
    let ruta = "Dia10.csv";
    let mut mapa = leer_mapa(ruta);
    println!("Mapa leido");
    let grafo = Grafo::new(&mut mapa);
    println!("Grafo creado");
    let cabeza = grafo.get_inicio();
    println!("Punto de inicio localizado");

    println!("El camino más corto es: {}", BFS(grafo, cabeza).iter().min().unwrap());
}

fn leer_mapa(ruta: &str) -> Vec<Vec<u8>>{
    let mut mapa = Vec::new();
    let archivo = fs::read_to_string(ruta).expect("No se pudo abrir el archivo con el mapa");
    for fila in archivo.lines(){
        mapa.push(fila.split(",").map(|caracter| caracter.parse().expect("Caracteres indebidos en el archivo")).collect());
    }

    return mapa;

}

#[allow(non_snake_case)]
fn BFS(grafo: Grafo, cabeza: Rc<RefCell<Node>>) -> Vec<usize>{
    let mut nodos_visitados = 0;
    let mut cola = Queue::new();
    let mut soluciones = Vec::new();
    cabeza.borrow_mut().steps = Some(0);
    let _ = cola.add(cabeza);

    while cola.size() > 0{
        let v = cola.remove().unwrap();
        let mut v = v.borrow_mut();
        if v.es_destino(){
            println!("Se ha encontrado una solución en {}\nHay{} caminos restantes", v.steps.unwrap(), cola.size());
            soluciones.push(v.steps.unwrap());
            continue;
        }

        v.get_allowed_nodes(&grafo);
        for vecino in &v.next{
            let mut vecino_t = vecino.borrow_mut();
            if let None = vecino_t.steps{
                vecino_t.steps = Some(v.steps.unwrap()+1);
                let _ = cola.add(Rc::clone(&vecino));
                nodos_visitados += 1;
                //println!("{}", vecino_t);
                if nodos_visitados%100 == 0{
                    println!("\tSe han visitado {nodos_visitados} nodos.");
                }

            }
        }
    }
    return soluciones;
}
