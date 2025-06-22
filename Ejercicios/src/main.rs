
// Esta función calcula la potencia al cuadrado de un número entero
// Recibe un número entero base y un exponente de tipo u32
// Retorna el resultado de elevar la base al exponente
fn potencia_al_cuadrado(base: i32) -> i32 {
    let mut resultado = base * base;
    resultado
}

// Estructura Libro que representa un libro con título, autor y año de publicación
// Cada campo es de tipo String para el título y autor, y u32 para el año

struct Libro {
    titulo: String,
    autor: String,
    ano: u32,
}

// Implementación de la estructura Libro
// Aquí se define un método para crear y resumir un libro
impl Libro {
    fn nuevo(titulo: String, autor: String, ano: u32) -> Libro {
        Libro {
            titulo,
            autor,
            ano,
        }
    }

    // Método para resumir información del libro
    fn resumen(&self) {
        println!("Título: {}, Autor: {}, Año: {}", self.titulo, self.autor, self.ano);
    }
}

fn main() {
    // Llamada a la función potencia_al_cuadrado con un número y un exponente
    let resultado = potencia_al_cuadrado(3);
    // Imprime el resultado de la potencia al cuadrado
    println!("El resultado de elevar al cuadrado es: {}", resultado);

    // Creación de un nuevo libro
    let libro = Libro::nuevo(String::from("La Odisea"), String::from("Homero"), 50);
    // Llamada al método resumen del libro para imprimir sus detalles
    libro.resumen();
}