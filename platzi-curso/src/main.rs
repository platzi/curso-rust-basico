use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "history.csv";

#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria{
        let vida = row.get(3).unwrap().trim();
        let vida : i32 = vida.parse().unwrap_or(0);
        return DatoHistoria {
            tipo_dato:row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
        };
    }
}



fn main() {
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);
        datos_historia.insert(dato.tag.clone(), dato);
    }

    println!("{:?}",datos_historia["DERECHA"]);
}
