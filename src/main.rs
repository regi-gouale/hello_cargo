use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Déviner le nombre !");

    // let nombre_secret:u32 = rand::random::<u32>() % 100 + 1;
    let nombre: u32 = rand::thread_rng().gen_range(1..101);

    println!("Le nombre secret est : {}", nombre);

    loop {
        println!("Veuillez entrer un nombre :");

        let mut supposition: String = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Vous avez deviné : {}", supposition);

        match supposition.cmp(&nombre) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
