use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let restaurants = vec![
        "Enrico Bartolini al Mudec",
        "Seta by Antonio Guida",
        "Vun Andrea Aprea",
        "D'O by Davide Oldani",
        "Cracco in Galleria",
        "Il Luogo di Aimo e Nadia",
        "Joia",
        "Berton",
        "L'Alchimia",
        "Contraste",
        "Essenza",
        "Il Ristorante Trussardi alla Scala",
        "Tano Passami l'Olio",
        "Alice Ristorante",
        "Innocenti Evasioni",
        "Sadler",
        "Torre di Pisa",
        "Al Pont de Ferr",
        "Langosteria",
        "Giacomo Bistrot",
    ];

    let mut rng = thread_rng();
    match restaurants.choose(&mut rng) {
        Some(choice) => println!("Tonight you should try: {}", choice),
        None => println!("No restaurants available"),
    }
}
