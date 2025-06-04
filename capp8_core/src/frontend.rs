pub trait Frontend {
    fn poll_keys(&mut self);
    fn render_display(&mut self);
    fn play_sound(&self);
    fn step(&mut self);
    fn run(&mut self);
}
