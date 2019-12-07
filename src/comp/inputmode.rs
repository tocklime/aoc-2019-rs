use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum InputMode {
    Channel(Receiver<isize>),
    List(Vec<isize>),
}
