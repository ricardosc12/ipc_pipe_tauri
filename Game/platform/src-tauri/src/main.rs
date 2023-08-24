// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, RwLock};

use std::thread;
mod models;
mod named_pipe;
use models::player::Player;

#[derive(Clone)]
struct AppState {
    players: Arc<RwLock<Vec<Arc<Mutex<Player>>>>>,
}

#[derive(serde::Serialize)]
struct CustomResponse {
    players: Arc<RwLock<Vec<Arc<Mutex<Player>>>>>,
    win: bool,
}

#[tauri::command]
fn greet(app_state: tauri::State<AppState>) -> CustomResponse {
    CustomResponse {
        players: app_state.players.clone(),
        win: false,
    }
}
#[derive(Copy, Clone, serde::Serialize)]
pub struct Map {
    dimensions: [i32; 2],
    goal: [i32; 2],
}

pub static MAP: Map = Map {
    dimensions: [6, 3],
    goal: [5, 1],
};

#[tauri::command]
fn init() -> Map {
    MAP
}

fn player_controller(input: String, player: Arc<Mutex<Player>>) -> String {
    let mut player_lock = player.lock().unwrap();

    println!("Input: {} from Player {}", input, player_lock.id);
    if input == "r" && player_lock.x < MAP.dimensions[0] - 1 {
        player_lock.x += 1;
    }
    if input == "l" && player_lock.x > 0 {
        player_lock.x -= 1;
    }
    if input == "u" && player_lock.y > 0 {
        player_lock.y -= 1;
    }
    if input == "d" && player_lock.y < MAP.dimensions[1] - 1 {
        player_lock.y += 1;
    }

    format!("Message from Player: {}", player_lock.id).to_string()
}

fn main() {
    let players: Arc<RwLock<Vec<Arc<Mutex<Player>>>>> = Arc::new(RwLock::new(vec![]));

    let players_clone = Arc::clone(&players);

    let handle = thread::spawn(move || {
        named_pipe::named_pipe(Arc::clone(&players), player_controller);
    });

    tauri::Builder::default()
        .manage(AppState {
            players: players_clone,
        })
        .invoke_handler(tauri::generate_handler![greet, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    handle.join().unwrap();
}
