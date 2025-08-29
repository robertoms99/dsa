mod package;
pub mod util;

use std::{any::Any, error::Error, os::linux::raw::stat, process};

use queue_lib::{FixedQueue, QueueTrait};

use package::Package;

struct Stadistics {
  processed: i32,
  received: i32,
  ignored: i32
}

fn run_insert_package<const N: usize>(routing_queue: &mut FixedQueue<Package, N>, stadistics: &mut Stadistics){
  if let Ok(input) = util::get_user_input() {
    let input_args: Vec<&str> = input.split(';').collect();

    match Package::build(input_args.as_slice()) {
      Ok(package) =>  {
        stadistics.received += 1;
        if !routing_queue.is_full() {
          routing_queue.enqueue(package);
          return;
        }
        stadistics.ignored += 1;
        println!("Buffer lleno. Paquete [{}] descartado.", &package.id);
      },
      Err(e) => {
        println!("{}" ,&e);
      }
    }
  }
}

fn run_process_package<const N: usize>(routing_queue: &mut FixedQueue<Package, N>, stadistics: &mut Stadistics){
  if !routing_queue.is_empty() {
    routing_queue.dequeue();
    stadistics.processed += 1;
    return;
  }
  println!("No hay paquetes por procesar");
}

fn display_buffer<const N: usize>(routing_queue: &FixedQueue<Package, N>) {
    if routing_queue.is_empty() {
      println!("No hay entradas en el buffer");
      return;
    }

    println!("Buffer actual");
    routing_queue.traverse(|package| {
      println!("[{}] {} -> {} ({} bytes)", package.id,package.source_ip, package.destine_ip,package.size);
    });
}

fn display_stadistics(stadistics: &Stadistics) {
  println!("Estadisticas:");
  println!("\
Paquetes recibidos: {}
Paquetes procesados: {}
Paquetes descartados: {}
", stadistics.received,stadistics.processed, stadistics.ignored);
}

fn main(){
  let mut stadistics = Stadistics {
    ignored: 0,
    processed: 0,
    received: 0
  };

  let mut routing_queue: FixedQueue<Package, 10> = FixedQueue::new();

  loop {
    println!("\
1. Insertar paquete
2. Procesar paquete
3. Mostrar buffer
4. Mostrar estadísticas
5. Salir");

    if let Ok(input) = util::get_user_input() && let Ok(n) = input.trim().parse::<i32>() {
        match n {
            1 => {
              println!("Usa el formato <ID>;<SRC_IP>;<DEST_IP>;<S_BYTES>");
              run_insert_package(&mut routing_queue,&mut stadistics);
            },
            2=> { run_process_package(&mut routing_queue,&mut stadistics); },
            3 => { display_buffer(&routing_queue) },
            4 => { display_stadistics(&stadistics)},
            5 => {
              println!("chao");
              process::exit(0);
            },
            _ => continue
        }
    } else {
      println!("Error digitando las opciones");
      continue;
    }
  }
}
