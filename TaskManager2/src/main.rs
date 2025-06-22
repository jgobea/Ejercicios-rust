use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Estado {
    Pendiente,
    EnProgreso,
    Realizada,
}

impl Estado {
    fn como_str(&self) -> &'static str {
        match self {
            Estado::Pendiente => "pendiente",
            Estado::EnProgreso => "enprogreso",
            Estado::Realizada => "realizada",
        }
    }
    fn desde_str(s: &str) -> Estado {
        match s {
            "enprogreso" => Estado::EnProgreso,
            "realizada" => Estado::Realizada,
            _ => Estado::Pendiente,
        }
    }
}

struct Tarea {
    id: u32,
    descripcion: String,
    estado: Estado,
}

const ARCHIVO: &str = "tasks.json";

fn cargar_tareas() -> Vec<Tarea> {
    if !Path::new(ARCHIVO).exists() {
        return Vec::new();
    }
    let mut s = String::new();
    if fs::File::open(ARCHIVO).and_then(|mut f| f.read_to_string(&mut s)).is_err() {
        return Vec::new();
    }

    let mut tareas = Vec::new();
    for parte in s.split("},{") {
        let id = parte.split("\"id\":").nth(1)
        .and_then(|x| x.split(',').next())
        .and_then(|x| x.parse().ok())
        .unwrap_or(0);
        let desc = parte.split("\"descripcion\":\"").nth(1)
        .and_then(|x| x.split('"').next())
        .unwrap_or("").to_string();
        let estado = parte.split("\"estado\":\"").nth(1)
        .and_then(|x| x.split('"').next())
        .map(Estado::desde_str)
        .unwrap_or(Estado::Pendiente);
        if !desc.is_empty() {
            tareas.push(Tarea { id, descripcion: desc, estado });
        }
    }
    tareas
}

fn guardar_tareas(tareas: &Vec<Tarea>) {
    let mut s = String::from("[");
    for (i, t) in tareas.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(&format!(
            "{{\"id\":{},\"descripcion\":\"{}\",\"estado\":\"{}\"}}",
            t.id, t.descripcion.replace('"', "'"), t.estado.como_str()
        ));
    }
    s.push(']');
    let _ = fs::File::create(ARCHIVO).and_then(|mut f| f.write_all(s.as_bytes()));
}

fn siguiente_id(tareas: &Vec<Tarea>) -> u32 {
    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: {} <comando> [argumentos]", args[0]);
        println!("Comandos: agregar, eliminar, listar, marcar_enprogreso, marcar_realizada");
        return;
    }
    let mut tareas = cargar_tareas();
    match args[1].as_str() {
        "agregar" => {
            if args.len() < 3 {
                println!("Uso: agregar <descripcion>");
                return;
            }
            let desc = args[2..].join(" ");
            let id = siguiente_id(&tareas);
            tareas.push(Tarea { id, descripcion: desc, estado: Estado::Pendiente });
            guardar_tareas(&tareas);
            println!("Tarea agregada con id {}", id);
        }
        "eliminar" => {
            if args.len() < 3 {
                println!("Uso: eliminar <id>");
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            let antes = tareas.len();
            tareas.retain(|t| t.id != id);
            guardar_tareas(&tareas);
            if tareas.len() < antes {
                println!("Tarea {} eliminada", id);
            } else {
                println!("No existe la tarea");
            }
        }
        "listar" => {
            if tareas.is_empty() {
                println!("No hay tareas");
            } else {
                for t in &tareas {
                    println!("[{}] {} - {}", t.id, t.descripcion, t.estado.como_str());
                }
            }
        }
        "marcar_enprogreso" => {
            if args.len() < 3 {
                println!("Uso: marcar_enprogreso <id>");
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            for t in &mut tareas {
                if t.id == id {
                    t.estado = Estado::EnProgreso;
                }
            }
            guardar_tareas(&tareas);
            println!("Tarea {} marcada como en progreso", id);
        }
        "marcar_realizada" => {
            if args.len() < 3 {
                println!("Uso: marcar_realizada <id>");
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            for t in &mut tareas {
                if t.id == id {
                    t.estado = Estado::Realizada;
                }
            }
            guardar_tareas(&tareas);
            println!("Tarea {} marcada como realizada", id);
        }
        _ => println!("Comando no reconocido"),
    }
}
