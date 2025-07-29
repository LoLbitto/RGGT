use crate::logical::entity::object::Object;

trait Mapa {
    fn get_objects() -> Vec<Object>;

    fn get_start_position() -> (Vec<f32>, Vec<f32>); // Um pra mira e outro pro player

    //pub fn get_entities() -> Vec<Entity>; Não temos entidades ainda F
}
