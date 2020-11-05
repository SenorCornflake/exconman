mod config;
mod brain;
mod utilities;

fn main() {
    let registry = config::parse_registry("registry.txt");
    let modifier = config::parse_modifier("modifier.txt");

    if registry.is_err() || modifier.is_err() {
        return;
    }

    if config::check_modifier(modifier.unwrap(), "modifier.txt", &registry.unwrap()).is_err() {
        return;
    };
    
}
