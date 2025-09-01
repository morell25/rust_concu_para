use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn main_test1() {
    let hilos = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!("Numero total de hilos: {}", hilos);

    let iteracciones: u32 = 1000;
    let contador = Arc::new(Mutex::new(0));
    let mut hilos_trabajados: Vec<JoinHandle<()>> = Vec::with_capacity(hilos);

    for _ in 0..hilos {
        let hilo_clonado = Arc::clone(&contador);
        let hilo_trabajador = thread::spawn(move || {
            let mut cont_local = 0;
            for _ in 0..iteracciones {
                cont_local += 1
            }

            {
                let mut total = hilo_clonado.lock().expect("hilo clonado");
                *total += cont_local;
                println!("{}", *total)
            }
        });

        hilos_trabajados.push(hilo_trabajador);
    }
}
