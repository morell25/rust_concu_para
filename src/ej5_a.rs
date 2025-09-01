use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub fn main_ej5_a() {
    let contador_compartido = Arc::new(Mutex::new(0u64));
    let hilos = 5;
    let incrementos = 100;
    let mut incrementos_vect: Vec<JoinHandle<()>> = Vec::with_capacity(hilos);

    for _ in 0..hilos {
        let handler_contador = contador_compartido.clone();
        let hilo_opera = thread::spawn(move || {
            for x in 0..incrementos {
                let mut guard_contador = handler_contador.lock().unwrap();
                *guard_contador += 1;
            }
        });
        incrementos_vect.push(hilo_opera);
    }

    for x in incrementos_vect {
        let q = x.join();
        println!("{:?}", q)
    }
}
