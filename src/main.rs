use state::State;

mod state;
mod entities;

#[tokio::main]
async fn main() {
    let state = State::load().await;

    
}
