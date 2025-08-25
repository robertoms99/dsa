mod list;

use std::{process, time::SystemTime};
use list::*;

struct Page {
  id: i32,
  url: String,
  timestamp: SystemTime
}

fn main() {
  let mut history = LinkedList::<Page>::new();
  let mut id = 0;

  loop {

    println!("\
1. Visitar página
2. Mostrar historial
3. Ir atrás
4. Ir adelante
5. Mostrar pagina actual
6. Borrar historial
7. Salir");

    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
      if let Ok(input) = input.trim().parse::<i32>() {
        match input {
            1 => {
              println!("Ingresa url de pagina: ");
              let mut url = String::new();
              if let Ok(_) = std::io::stdin().read_line(&mut url) {
                id = id + 1;
                history.add(Page { id, url, timestamp: SystemTime::now() });
              }
            },
            2=> {
              history.traverse(|page| println!("[{}]= {}",(*page).id, (*page).url));
            },
            3 => {
              if let Some(prev_entry)= history.back() {
                println!("Entrada actual: [{}]={}",(*prev_entry).id,(*prev_entry).url);
              }else {
                println!("No hay entradas previas");
              }
            },
             4 => {
              if let Some(next_entry)= history.forward() {
                println!("Entrada actual: [{}]={}",(*next_entry).id,(*next_entry).url);
              } else {
                println!("No hay entradas siguientes");
              }
            },
            5 => {
              let Some(current) = history.current() else {
                println!("No hay entradas");
                continue;
              };
                println!("Entrada actual: [{}]={}",(*current).id,(*current).url);
            },
            6 => {
              history.clear();
              println!("Historial limpio");
            },
            _ => {
              println!("chao");
              process::exit(0);
            }
        }
      }
    }
  }
}
