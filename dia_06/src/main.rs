use rusqlite::{self, Connection};

#[derive(Debug)]
struct Culpable{
    nombre: String,
    sala: i64,
    hora: String
}

fn main() {
    let ruta = "Dia6.db";
    let connection = Connection::open(ruta).unwrap();

    let query = "SELECT nombre, id_sala, tiempo FROM PersonasReconocidas WHERE (tiempo in (SELECT tiempo FROM Temperatura WHERE temperatura in (SELECT MIN(temperatura) FROM Temperatura))) AND (id_sala in (SELECT id_sala FROM Temperatura WHERE temperatura in (SELECT MIN(temperatura) FROM Temperatura))) ORDER BY tiempo LIMIT 3";
    //let query = "SELECT id_sala FROM Temperatura WHERE temperatura in (SELECT MIN(temperatura) FROM Temperatura)";
    let mut statement = connection.prepare(query).unwrap();
    let answer = statement.query_map([], |row| {
        Ok(
            Culpable{
                nombre: row.get(0).unwrap(),
                sala: row.get(1).unwrap(),
                hora: row.get(2).unwrap(),
            })
    }).unwrap();

    for i in answer{
        println!("Culpable: {:?}", i);
    }

}
