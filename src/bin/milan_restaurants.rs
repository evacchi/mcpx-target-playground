use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let restaurants = vec![
        "Le Bernardin (French seafood, 3 Michelin stars)",
        "Eleven Madison Park (Contemporary American, 3 Michelin stars)",
        "Per Se (French-American, 3 Michelin stars)",
        "Jean-Georges (French-Asian, 2 Michelin stars)",
        "Daniel (French, 2 Michelin stars)",
        "Atomix (Contemporary Korean)",
        "Chef's Table at Brooklyn Fare (Contemporary)",
        "Masa (Japanese sushi)",
        "Blue Hill at Stone Barns (Farm-to-table)",
        "Gramercy Tavern (Contemporary American)",
        "Cosme (Contemporary Mexican)",
        "L'Artusi (Italian)",
        "Peter Luger Steak House (Steakhouse)",
        "Estela (Contemporary American)",
        "Momofuku Ko (Contemporary Asian)",
        "Le Coucou (French)",
        "The Modern (Contemporary American)",
        "Atera (Contemporary American)",
        "Carbone (Italian-American)",
        "Crown Shy (Contemporary American)",
    ];

    let mut rng = thread_rng();
    match restaurants.choose(&mut rng) {
        Some(choice) => println!("Tonight you should try: {}", choice),
        None => println!("No restaurants available"),
    }
}
