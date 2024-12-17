use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

struct CityRestaurants {
    name: &'static str,
    restaurants: Vec<&'static str>,
}

fn get_cities() -> Vec<CityRestaurants> {
    vec![
        CityRestaurants {
            name: "Milan",
            restaurants: vec![
                "Enrico Bartolini al Mudec (3 Michelin stars)",
                "Seta by Antonio Guida (2 Michelin stars)",
                "Vun Andrea Aprea (2 Michelin stars)",
                "D'O by Davide Oldani",
                "Cracco in Galleria",
                "Il Luogo di Aimo e Nadia (2 Michelin stars)",
                "Joia (vegetarian, 1 Michelin star)",
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
            ],
        },
        CityRestaurants {
            name: "New York",
            restaurants: vec![
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
            ],
        },
    ]
}

fn print_menu(cities: &[CityRestaurants]) {
    println!("\nAvailable cities:");
    for (i, city) in cities.iter().enumerate() {
        println!("{}. {}", i + 1, city.name);
    }
}

fn get_user_choice(max: usize) -> usize {
    loop {
        print!("Enter your choice (1-{}): ", max);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= max => return num - 1,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn main() {
    let cities = get_cities();
    println!("Welcome to the Restaurant Picker!");
    print_menu(&cities);

    let choice = get_user_choice(cities.len());
    let city = &cities[choice];

    let mut rng = thread_rng();
    match city.restaurants.choose(&mut rng) {
        Some(restaurant) => println!("\nIn {}, you should try: {}", city.name, restaurant),
        None => println!("No restaurants available in {}", city.name),
    }
}
