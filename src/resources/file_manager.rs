use std::fs;

pub fn get_object(nome: String) -> (Vec<f32>, Vec<f32>) {
    let dados = fs::read_to_string("resources/".to_owned() + &nome + ".rgm").expect("Deveria abrir o arquivo");
    
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
