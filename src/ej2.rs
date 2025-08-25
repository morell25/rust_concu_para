/*
Contador simple
Un hilo cuenta del 1 al 10, el otro del 11 al 20.
Objetivo: entender ejecución intercalada.
*/

use std::{thread, time::Duration};
pub fn main_ej2() {
    let mut contador = 0;
    let handler = thread::spawn(move || {
        for _ in 0..10 {
            contador = contador + 1;
            println!("a dormir 200ms");
            thread::sleep(Duration::from_millis(200));
        }
        return contador;
    });
    let handler2 = thread::spawn(move || {
        for _ in 0..10 {
            contador = contador + 1;
            println!("a dormir 400ms");
            thread::sleep(Duration::from_millis(400));
        }
        return contador;
    });

    let resultado1 = handler.join().unwrap();
    let resultado2 = handler2.join().unwrap();
    print!("{:?}", resultado1 + resultado2);
}
