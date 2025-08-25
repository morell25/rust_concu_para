use std::{sync::mpsc, thread, time::Duration};
struct ABMsg {
    id: u32,
    nombre: String,
    valor: i64,
    activo: bool,
}
//Transformador
struct BCMsg {
    id: u32,
    nombre: String,
}

pub fn main_ej4_a() {
    //Pipeline lineal (1 productor → 1 transformador → 1 consumidor)
    //Productor

    //creamos los canales
    let (tx1, rx1) = mpsc::channel::<ABMsg>();
    let (tx2, rx2) = mpsc::channel::<BCMsg>();
    //fin de creacion de canales

    thread::spawn(move || {
        for x in 0..10 {
            let _ = tx1.send(ABMsg {
                id: x,
                nombre: format!("t1{x}"),
                valor: 12,
                activo: true,
            });
            thread::sleep(Duration::from_millis(200));
            println!("Dormido en hilo 1")
        }
    });

    thread::spawn(move || {
        for x in rx1 {
            if x.id % 2 == 0 {
                let _ = tx2.send(BCMsg {
                    id: x.id,
                    nombre: x.nombre,
                });

                thread::sleep(Duration::from_millis(100));
                println!("Dormido en hilo 2")
            }
        }
    });

    for x in rx2 {
        println!("id: {}, nombre {}", x.id, x.nombre)
    }
}
