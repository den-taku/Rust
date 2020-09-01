use std::sync::Mutex;
use std::sync::Arc;

type PlayerId = u32;

const GAME_SIZE: usize = 8;

/// A waiting list never grows to more than GAME_SIZE players.
type WaitingList = Vec<PlayerId>;

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>
}

impl FernEmpireApp {
    /// Add a player to the waiting list for the next game.
    /// Start a new game immediately if enough players are waiting,
    fn join_waiting_list(&self, player: PlayerId) {
        // Lock the mutex and gain access to the data inside.
        // The scope of `guard` is a critical section.
        let mut guard = self.waiting_list.lock().unwrap();

        // Now do the game logic.
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            drop(guard);
            self.start_game(player);
        }
    }
}

fn main() {
    let app = Arc::new(FernEmpireApp { waiting_list: Mutex::new(vec![]) });
    println!("Hello, world!");
}
