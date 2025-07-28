mod play_state;

trait State {
    pub fn get_vertices () -> Vec<f32>;
}
