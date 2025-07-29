use std::fs;

use crate::logical::mapa::Mapa;
use crate::logical::mapa::DefaultMap;
use crate::logical::entity::object::Object;

pub fn get_object(nome: String) -> (Vec<f32>, Vec<f32>) {
    let dados = fs::read_to_string("resources/models/".to_owned() + &nome + ".rgm").expect("Deveria abrir o arquivo");
    
    let mut points = Vec::<f32>::new();
    let mut map = Vec::<f32>::new();

    let mut vecInUse = &mut points;

    let mut word = "".to_string();

    for caractere in dados.chars() {
        match caractere {
            '>' => {
                match word.trim() {
                    "<points" => {
                        vecInUse = &mut points;
                        word = "".to_string();
                    },

                    "<map" => {
                        vecInUse = &mut map;
                        word = "".to_string();
                    },

                    _ => {
                        println!("{}", word);
                        word = "".to_string();
                    }
                }
            },

            ',' => {
                vecInUse.push(word.trim().parse::<f32>().unwrap());
                word = "".to_string();
            },

            ' ' => {},

            '\t' => {},

            '\n' => {},

            _ => {
                word.push(caractere);
            }
        }
    }

    (points, map)
} 

pub fn get_map(nome: String) -> impl Mapa { // Por enquanto vai funcionar de mentirinha
    let objeto = Object::new("piramide".to_string());
    let vec = vec![objeto];

    DefaultMap::new(vec, [10.0, 5.0, 15.0], [15.0, 5.0, 15.0])
}
