//¡Vamos con el Ejercicio 3: Transferencias bancarias sin deadlock!
/*
Simular transferencias de dinero entre varias cuentas, ejecutadas por múltiples hilos, garantizando que:
Nunca haya deadlock (bloqueo mutuo).
Nunca haya saldo negativo en una cuenta.
La suma total de saldos se mantenga constante.
*/

use std::{
    cmp::{max, min},
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

struct Cuenta {
    id: usize,
    saldo: i64,
}

pub fn main_test3() {
    let total_cuentas = 4;
    let mut cuentas: Vec<Arc<Mutex<Cuenta>>> = Vec::with_capacity(total_cuentas);
    for x in 0..total_cuentas {
        cuentas.push(Arc::new(Mutex::new(Cuenta {
            id: x,
            saldo: 100000,
        })));
    }
}

fn total_sistema(cuentas: Vec<Arc<Mutex<Cuenta>>>) -> JoinHandle<i64> {
    thread::spawn(move || {
        let mut total: i64 = 0;
        for c in cuentas {
            // Lock CORTO para leer el saldo y listo
            let saldo = c.lock().expect("lock saldo").saldo as i64;
            total += saldo;
        }
        total
    })
}

fn transferencia(
    cue1: Arc<Mutex<Cuenta>>,
    cue2: Arc<Mutex<Cuenta>>,
    monto: i64,
) -> Result<(), &'static str> {
    if Arc::ptr_eq(&cue1, &cue2) {};

    let (primero, segundo) = if (Arc::as_ptr(&cue1) as usize) <= (Arc::as_ptr(&cue2) as usize) {
        (cue1.clone(), cue2.clone())
    } else {
        (cue2.clone(), cue1.clone())
    };

    let mut g_cue1 = primero.lock().map_err(|_| "Mutex poi cue1")?;
    let mut g_cue2 = segundo.lock().map_err(|_| "Mutex poi cue2")?;
    if g_cue1.saldo < monto {
        g_cue1.saldo -= monto;
        g_cue2.saldo += monto;
    }
    Ok(())
}
