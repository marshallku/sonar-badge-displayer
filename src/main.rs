use env::env::Env;

mod env;

fn main() {
    let env = Env::new();
    let _ = env::state::AppState::from(env.clone());

    println!("Hello, world!");
}
