pub trait Player {
    fn play(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn stop(&mut self);
    fn fast_forward(&mut self);
    fn fast_rewind(&mut self);
}
