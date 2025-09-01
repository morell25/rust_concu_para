//“Logger” con cola compartida
//Objetivo: varios hilos “productores” generan mensajes y un hilo “consumidor” los procesa.
/*
Qué vas a construir
Un estado compartido: una cola de mensajes y un par de banderas/contadores para saber cuándo terminar.
Productores (2–3 hilos): añaden mensajes a la cola.
Consumidor (1 hilo): saca mensajes y los procesa.
Criterio de fin: cuando todos los productores terminen y la cola quede vacía, el consumidor se

===
Uso de arc, mutex y VecDeque
*/

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
struct Estado {
    cola: VecDeque<String>,
    productores_activos: usize,
}

pub fn main_test2() {
    let num_productores: usize = 4;
    let arc_estado: Arc<Mutex<Estado>> = Arc::new(Mutex::new(Estado {
        cola: VecDeque::new(),
        productores_activos: num_productores,
    }));
    let mut _hilos: Vec<JoinHandle<()>> = Vec::new();
    let handler_productores = productores(num_productores, 10, arc_estado.clone());

    let handler_consumidor = consumidor(arc_estado);
    for x in handler_productores.into_iter() {
        x.join().expect("error join")
    }
    handler_consumidor.join().expect("msg");
}

fn productores(
    num_productores: usize,
    msg: usize,
    arc_estado: Arc<Mutex<Estado>>,
) -> Vec<JoinHandle<()>> {
    let mut hilos_sincro = Vec::with_capacity(num_productores);
    for x in 0..num_productores {
        let estado_clonado = Arc::clone(&arc_estado);
        let hilo_estado = thread::spawn(move || {
            for x in 0..msg {
                let msg_envio = x.to_string();
                {
                    estado_clonado
                        .lock()
                        .expect("envenenado")
                        .cola
                        .push_back(msg_envio);
                }
            }
            {
                estado_clonado.lock().unwrap().productores_activos -= 1;
            }
        });
        hilos_sincro.push(hilo_estado);
    }
    hilos_sincro
}

fn consumidor(estado: Arc<Mutex<Estado>>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let maybe_msg = {
                let mut st = estado
                    .lock()
                    .expect("Nada que mostrar");

                if let Some(msg) = st.cola.pop_front() {
                    Some(msg)
                } else {
                    return;
                }
            };

            if let Some(m) = maybe_msg {
                println!("numero: {m}");
            }
        }
    })
}
