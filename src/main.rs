use tarea_1::stack::Stack;
use tarea_1::queue::Queue;
use std::collections::HashMap;

fn main() {
    // Stack Operaciones 
    println!("Inicializamos el stack!");
    let mut stack = Stack::new();
    
    println!("Stack vacío: {}", stack.is_empty());
    println!("Metemos un 1 y 2 al stack");
    stack.push(1);
    stack.push(2);

    println!("Stack tamaño: {}", stack.size());
    println!("Peek: {:?}", stack.peek());


    println!("Pop: {:?}", stack.pop());
    println!("Stack tamaño después del pop: {}", stack.size());

    // Queue Operaciones 
    println!("\nInicializamos el queue!");
    let mut queue = Queue::new();

    println!("Queue vacío: {}", queue.is_empty());
    println!("Metemos un 1 y 2 al queue");
    queue.enqueue(1);
    queue.enqueue(2);

    println!("Queue tamaño: {}", queue.size());
    println!("Peek: {:?}", queue.peek());

    println!("Dequeue: {:?}", queue.dequeue());
    println!("Queue tamaño después del dequeue: {}", queue.size());

    // HashMap Operaciones 
    println!("\nInicializamos el hashtable!");
    let mut hashtable = HashMap::new();

    // Insertamos valores
    println!("Insertamos un Max(Agresivo) y Lucas(Calmado)");
    hashtable.insert("Max", "Agresivo");
    hashtable.insert("Lucas", "Calmado");

    println!("Get Max: {:?}", hashtable.get("Max"));
    println!("Get Lucas: {:?}", hashtable.get("Lucas"));

    // Borramos valores
    println!("Borramos Max");
    hashtable.remove("Max");

    println!("Get Max: {:?}", hashtable.get("Max"));
}

