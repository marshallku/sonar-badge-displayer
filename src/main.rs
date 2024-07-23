mod env;

fn main() {
    let _ = env::state::AppState::from_env();

    println!("Hello, world!");
}
