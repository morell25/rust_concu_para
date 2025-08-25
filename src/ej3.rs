/*

*/
use std::{sync::mpsc, thread};

pub fn main_ej3() {
    // enum de mensaje
    enum Mensaje {
        Dato(u32),
        Fin,
    }

    //declaramos tx y rx con tipodo de mensaje
    let (sender, receiver) = mpsc::channel::<Mensaje>();

    //clonamos el sender para no tener que estar moviendo variables
    let send1 = sender.clone();
    //Lo creo en una variables para que posteriormente le pueda hacer un join y asi esperar a que terminen todos los hilos de trabajar
    let h1 = thread::spawn(move || {
        for x in 0..20 {
            if x % 2 == 0 {
                let _ = send1.send(Mensaje::Dato(x));
            }
        }
    });

    let send2 = sender.clone();
    let h2 = thread::spawn(move || {
        for x in 0..20 {
            if x % 2 != 0 {
                let _ = send2.send(Mensaje::Dato(x));
            }
        }
    });

    //espero a que todo los hilos terminen de trabajar
    h1.join().unwrap();
    h2.join().unwrap();

    //todos los hilos han terminado, emito el mensaje de fin para asi no tener que finalizar a los hilos
    let _ = sender.send(Mensaje::Fin);
    drop(sender);

    //trabajo de hilos terminado

    for x in receiver {
        match x {
            Mensaje::Dato(n) => {
                print!("{n}")
            }
            Mensaje::Fin => {
                println!("Todo el trabajo terminado");
                break;
            }
        }
    }
}
