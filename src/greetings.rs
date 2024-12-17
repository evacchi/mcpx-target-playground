// A collection of friendly greetings in different languages

pub fn hello_world() -> &'static str {
    "Hello, World!"
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to our playground!", name)
}

pub fn greet_in_languages(name: &str) -> Vec<String> {
    vec![
        format!("🇬🇧 Hello, {}", name),
        format!("🇮🇹 Ciao, {}", name),
        format!("🇯🇵 こんにちは, {}", name),
        format!("🇪🇸 ¡Hola, {}!", name),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, World!");
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Claude"), "Hello, Claude! Welcome to our playground!");
    }

    #[test]
    fn test_greet_in_languages() {
        let greetings = greet_in_languages("Claude");
        assert_eq!(greetings.len(), 4);
        assert!(greetings.iter().all(|g| g.contains("Claude")));
    }
}