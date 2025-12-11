use states::State;

pub struct PauseSubState {
    background_vec: Vec<f32>,
    buttons: Vec<Button>,
    mouse_position: PhysicalPosition<f64>, 

}

impl PauseSubState {
    pub fn new(background_vec: Vec<f32>) -> Box<Self> {
        
    }
}

impl State for PauseSubState {
    
}
