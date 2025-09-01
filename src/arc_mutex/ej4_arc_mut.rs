use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
struct Tarea {
    id: usize,
}
pub fn main_test4() {
    let tareas_num_total = 100000;
    let mut tareas_vector: VecDeque<Tarea> = VecDeque::with_capacity(tareas_num_total);
    for x in 0..tareas_num_total {
        tareas_vector.push_front(Tarea { id: x });
    }

    let tareas_vec_arc = Arc::new(Mutex::new(tareas_vector));

    let q = tareas_workers(tareas_vec_arc);
    for x in q {
        let w = x.join().unwrap();
        //println!("{:?}", w);
    }
}

fn tareas_workers(tareas_vec_arc: Arc<Mutex<VecDeque<Tarea>>>) -> Vec<JoinHandle<()>> {
    //let workers_disponibles = thread::available_parallelism();
    let mut hablers_hilos: Vec<JoinHandle<()>> = Vec::new();
    let hilos_totales = thread::available_parallelism().unwrap().into();
    println!("{}", hilos_totales);
    for _ in 0..hilos_totales {
        let tarea_hilo = tareas_vec_arc.clone();
        let hilo_hanlder = thread::spawn(move || {
            loop {
                let tarea = {
                    let mut guard = tarea_hilo.lock().unwrap();
                    guard.pop_front()
                };
                match tarea {
                    Some(tarea) => {
                        //println!("{}", tarea.id)
                    }
                    None => break,
                }
            }
        });
        hablers_hilos.push(hilo_hanlder);
    }
    hablers_hilos
}
