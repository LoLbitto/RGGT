use crate::logical::entity::object::Object;
use crate::resources::file_manager::get_map;

pub trait Mapa {
    fn get_objects(&mut self) -> &mut Vec<Object>;

    fn get_start_position(&self) -> (&[f32; 3], &[f32; 3]); // Um pra mira e outro pro player

    //pub fn get_entities() -> Vec<Entity>; Não temos entidades ainda F
}

pub struct DefaultMap {
    objects:         Vec<Object>,
    start_position:  [f32; 3],
    start_aim:       [f32; 3]

}

impl DefaultMap {
    pub fn new(nome: String) -> Self {
        let objects = get_map(nome);
        let (start_position, start_aim) = ([0.0, 5.0, 0.0], [5.0, 5.0, 0.0]);
        Self {objects, start_position, start_aim}
    }
}

impl Mapa for DefaultMap {
    fn get_objects(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }

    fn get_start_position(&self) -> (&[f32; 3], &[f32; 3]) {
        (&self.start_position, &self.start_aim)
    }
}
