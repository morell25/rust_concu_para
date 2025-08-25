//Hola desde dos hilos
use std::thread;
pub fn main_ej1() {
    println!("Hola desde el hilo principal");
    let handler = thread::spawn(|| println!("Hola desde el hilo secundario"));
    handler.join().unwrap();
}
