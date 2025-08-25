use std::{
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
    time::Duration,
    vec,
};
#[derive(Debug)]
struct ABMsg {
    id: u32,
    nombre: String,
    valor: i64,
    activo: bool,
}
//Transformador
#[derive(Debug)]
struct BCMsg {
    id: u32,
    nombre: String,
    worker_id: usize,
}

pub fn main_ej4_b() {
    let workers = 4;
    let totales = 500;

    //canales de envio y reception principales
    let (c1t, c1r) = mpsc::channel::<ABMsg>();
    let (c2t, c2r) = mpsc::channel::<BCMsg>();

    //canal router
    let mut t_trabajadores: Vec<mpsc::Sender<ABMsg>> = Vec::with_capacity(workers);
    let mut s_trabajadores: Vec<mpsc::Receiver<ABMsg>> = Vec::with_capacity(workers);

    //crear los trabajadores de forma dinamica
    for _ in 0..workers {
        let (t_tra, s_tra) = mpsc::channel::<ABMsg>();
        t_trabajadores.push(t_tra);
        s_trabajadores.push(s_tra);
    }

    let _distribuidor_de_trabajadores = invocar_trab(s_trabajadores, c2t.clone());
    drop(c2t);
    let _distribuidor_router = invocar_router(c1r, t_trabajadores);
    let _distribuidor_productor = invocar_productor(c1t, totales);

    for x in c2r {
        println!("{:?}", x)
    }
}
fn invocar_trab(
    trabajadores: Vec<mpsc::Receiver<ABMsg>>,
    enviador: mpsc::Sender<BCMsg>,
) -> Vec<JoinHandle<()>> {
    trabajadores
        .into_iter()
        .enumerate()
        .map(|(worker_id, rx)| {
            let enviador_del_hilo = enviador.clone();
            thread::spawn(move || {
                for msg in rx {
                    if msg.id % 2 == 0 {
                        let msg_a_enviar = BCMsg {
                            id: msg.id,
                            worker_id,
                            nombre: msg.nombre,
                        };
                        if enviador_del_hilo.send(msg_a_enviar).is_err() {
                            //Esto se hace para que en caso de error este hilo se interrumpa y no se envie el mensaje
                            break;
                        }
                    }
                    println!("workers {worker_id}")
                }
            })
        })
        .collect()
}

fn invocar_router(recividor_msg_a: mpsc::Receiver<ABMsg>, trabajadores: Vec<Sender<ABMsg>>) {
    thread::spawn(move || {
        let trabajadores_totales = trabajadores.len();
        let mut idx = 0;
        //por cada mensaje que recivo se lo envio al trabajadores de forma dinamica
        for x in recividor_msg_a {
            let _ = trabajadores[idx].send(x);
            idx = (idx + 1) % trabajadores_totales;
        }
    });
}

fn invocar_productor(enviador_msg_a: mpsc::Sender<ABMsg>, totales: u32) {
    thread::spawn(move || {
        for x in 0..totales {
            let msg_a = enviador_msg_a.send(ABMsg {
                id: x,
                nombre: format!("envo: {x}"),
                valor: 12,
                activo: true,
            });
        }
    });
}
