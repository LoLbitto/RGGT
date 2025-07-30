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
                    },

                    "<map" => {
                        vecInUse = &mut map;
                    },

                    _ => {
                        println!("{}", word);
                    }
                }
                word = "".to_string();
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

pub fn get_map(nome: String) -> Vec<Object> { // Por enquanto vai funcionar de mentirinha
    let dados = fs::read_to_string("resources/maps/".to_owned() + &nome + ".rgl").expect("Deveria abrir o arquivo, mas n foi :(");
    
    let mut objects = Vec::<Object>::new();
    let mut mode = 0;                       // 0 = default, 1 = <Entities>, 2 = <Models>, 3 = <Map>

    let mut word = "".to_string();

    let mut models = Vec::<String>::new();

    for caractere in dados.chars() {
        match caractere {
            '>' => {
                match mode {
                    0 => {
                        match word.to_lowercase().trim() {
                            "<entities" => {
                                mode = 1;
                            },

                            "<models" => {
                                mode = 2;
                            },

                            "<map" => {
                                mode = 3;
                            },

                            _ => {
                                mode = 0;
                            }
                        }
                        word = "".to_string();
                    },

                    1 => {
                        println!("Não tem entidades ainda");
                        word = "".to_string();
                        mode = 0;
                    },
                    
                    2 => {
                        let split: Vec<&str> = word.split(" ").collect();
                        
                        match split[0].to_lowercase().trim() {
                            "<model" => {
                                let mut infos = String::new();
                                
                                for i in 1..split.len() {
                                    infos.push_str(split[i]);
                                }

                                let mut word2 = "".to_string();
                                let mut mode2 = 0; // 0 = variavel 1 = valor
                                let mut var = "".to_string();

                                let mut id = 0;
                                let mut nome = "".to_string();

                                for caractere2 in infos.chars() {
                                    match caractere2 {
                                        '=' => {
                                            match word2.to_lowercase().trim() {
                                                "id" | "name" => {
                                                    var = word2.clone();
                                                },

                                                _ => {
                                                    var = "null".to_string();
                                                    println!("Não existe isso aí doido, {}", word2);
                                                }
                                            }
                                        },
                                        
                                        '"' => {
                                            if mode2 == 0 {
                                                mode2 = 1;
                                            } else {
                                                match var.trim() {
                                                    "id" => {
                                                        id = word2.trim().parse::<u32>().unwrap();
                                                    },

                                                    "name" => {
                                                        nome = word2.clone();      
                                                    },

                                                    _ => {
                                                    }
                                                }
                                                mode2 = 0;
                                            }
                                            word2 = "".to_string();
                                        },

                                        _ => {
                                            word2.push(caractere2);
                                        }
                                    }
                                }

                                models.push(nome);
                            },
                            
                            "</models" => {
                                mode = 0;
                            }

                            _ => {
                                println!("calma lá")
                            }
                        }
                        word = "".to_string();
                    },
                    
                    3 => {
                        let split: Vec<&str> = word.split(" ").collect();
                        
                        match split[0].to_lowercase().trim() {
                            "<object" => {
                                let mut id = 0;
                                let mut pos = vec![0.0, 0.0, 0.0];
                                let mut scale = 1.0;

                                let mut infos = String::new();
                                
                                for i in 1..split.len() {
                                    infos.push_str(split[i]);
                                }

                                let mut word2 = "".to_string();
                                let mut mode2 = 0; // 0 = variavel 1 = valor
                                let mut var = "".to_string();
                                
                                for caractere2 in infos.chars() {
                                    match caractere2 {
                                        '=' => {
                                            match word2.to_lowercase().trim() {
                                                "id" | "pos" | "scale" => {
                                                    var = word2.clone();
                                                },

                                                _ => {
                                                    var = "null".to_string();
                                                }
                                            }
                                        },

                                        '"' => {
                                            if mode2 == 0 {
                                                mode2 = 1;
                                            } else {
                                                match var.trim() {
                                                    "id" => {
                                                        id = word2.trim().parse::<u32>().unwrap();
                                                    },

                                                    "pos" => {
                                                        let values: Vec<&str> = word2.split(",").collect();
                                                        for i in 0..3 {
                                                            //if i+1 < values.len(){
                                                            pos[i] = values[i].parse::<f32>().unwrap();
                                                            println!("Ué: {}", values[i]);
                                                            /*} else {
                                                                pos[i] = 0.0;
                                                            }*/
                                                        }
                                                    },

                                                    "scale" => {
                                                        scale = word2.parse::<f32>().unwrap();
                                                    },

                                                    _ => {
                                                        println!("Não existe isso aí doido");
                                                    }
                                                }

                                                mode2 = 0;
                                            }
                                            word2 = "".to_string();
                                        },

                                        _ => {
                                            word2.push(caractere2);
                                        }
                                    }
                                }

                                let objeto = Object::new(models[id as usize].clone(), pos, scale);
                                objects.push(objeto);
                            },

                            "</map" => {
                                mode = 0;
                            }

                            _ => {}
                        }
                        word = "".to_string();
                    },
                    
                    _ => {}
                }
            },

            '\t' => {},

            '\n' => {},

            _ => {
                word.push(caractere);
            }
        }
    }

    objects
}
