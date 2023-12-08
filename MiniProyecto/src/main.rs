use std::fs::File; 
use std::io::{self, BufRead, BufReader, Write}; 
 
struct Transaccion { 
    tipo: String, 
    cantidad: f64, 
    fecha: String, 
    descripcion: String, 
} 
 
fn agregar_transaccion(transaccion: &Transaccion, file_name: &str) -> io::Result<()> { 
    let mut file = File::create(file_name)?; 
 
    writeln!(file, "{},{},{},{}", transaccion.tipo, transaccion.cantidad, transaccion.fecha, transaccion.descripcion)?; 
 
    Ok(()) 
} 
 
fn mostrar_resumen(file_name: &str) -> io::Result<f64> { 
    let file = File::open(file_name)?; 
    let reader = BufReader::new(file); 
 
    let mut saldo = 0.0; 
 
    println!("Lista de transacciones:"); 
    for line in reader.lines() { 
        if let Ok(transaction) = line { 
            let detalles: Vec<&str> = transaction.split(',').collect(); 
            if detalles.len() >= 2 { 
                let tipo = detalles[0]; 
                let cantidad: f64 = detalles[1].parse().unwrap_or(0.0); 
 
                println!("Tipo: {} - Cantidad: {} - Fecha: {} - Descripción: {}", tipo, cantidad, detalles.get(2).unwrap_or(&""), detalles.get(3).unwrap_or(&"")); 
 
                if tipo == "Ingreso" { 
                    saldo += cantidad; 
                } else { 
                    saldo -= cantidad; 
                } 
            } 
        } 
    } 
 
    Ok(saldo) 
} 
 
fn main() { 
    loop { 
        println!("\n### Gestión de presupuesto personal ###"); 
        println!("1. Agregar transacción"); 
        println!("2. Mostrar resumen del presupuesto"); 
        println!("3. Ver saldo actual"); 
        println!("4. Salir"); 
 
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
 
        match input.trim() { 
            "1" => { 
                let mut tipo = String::new(); 
                println!("Ingrese el tipo de transacción (Ingreso/Gasto):"); 
                io::stdin().read_line(&mut tipo).expect("Failed to read line"); 
 
                let mut cantidad = String::new(); 
                println!("Ingrese la cantidad:"); 
                io::stdin().read_line(&mut cantidad).expect("Failed to read line"); 
                let cantidad: f64 = cantidad.trim().parse().unwrap_or(0.0); 
 
                let mut fecha = String::new(); 
                println!("Ingrese la fecha (YYYY-MM-DD):"); 
                io::stdin().read_line(&mut fecha).expect("Failed to read line"); 
 
                let mut descripcion = String::new(); 
                println!("Ingrese una breve descripción:"); 
                io::stdin().read_line(&mut descripcion).expect("Failed to read line"); 
 
                let transaccion = Transaccion { 
                    tipo: tipo.trim().to_string(), 
                    cantidad, 
                    fecha: fecha.trim().to_string(), 
                    descripcion: descripcion.trim().to_string(), 
                }; 
 
                if let Err(e) = agregar_transaccion(&transaccion, "registro_transacciones.txt") { 
                    println!("Error al agregar la transacción: {}", e); 
                } 
            } 
            "2" => { 
                if let Err(e) = mostrar_resumen("registro_transacciones.txt") { 
                    println!("Error al mostrar el resumen: {}", e); 
                } 
            } 
            "3" => { 
                if let Ok(saldo) = mostrar_resumen("registro_transacciones.txt") { 
                    println!("Saldo actual: {}", saldo); 
                } else { 
                    println!("Error al mostrar el saldo."); 
                } 
            } 
            "4" => { 
                println!("¡Hasta luego!"); 
                break; 
            } 
            _ => println!("Opción inválida. Por favor, seleccione una opción válida."), 
        } 
    } 
}