// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use std::thread;
mod named_pipe;

#[derive(Clone)]
struct AppState {
    var_global: Arc<Mutex<i32>>,
}

#[derive(serde::Serialize)]
struct Player {
    nome: String,
}

#[derive(serde::Serialize)]
struct CustomResponse {
    players: Vec<Player>,
}

#[tauri::command]
fn greet(app_state: tauri::State<AppState>, teste: String) -> String {
    let mut player = 0;
    if let Ok(guard) = app_state.var_global.lock() {
        // Faça algo com a variável global aqui
        player = *guard;
    }
    format!("Players {}", player)
    // let player: Player = Player {
    //     nome: "Rust Player 1".to_string(),
    // };

    // let player2: Player = Player {
    //     nome: "Rust Player 2".to_string(),
    // };

    // println!("Message from front {}", teste);

    // let players: Vec<Player> = vec![player, player2];

    // CustomResponse { players: players }
}

#[tauri::command]
fn init(app_state: tauri::State<AppState>) -> Vec<i32> {
    vec![6, 3]
}

fn received(message: String){
    println!("Message: {}", message);
}



fn main() {
    let var_global = Arc::new(Mutex::new(0));

    let var_global_clone = Arc::clone(&var_global);

    let handle = thread::spawn(move || {
        named_pipe::named_pipe(Arc::clone(&var_global), received);
    });

    tauri::Builder::default()
        .manage(AppState {
            var_global: var_global_clone,
        })
        .invoke_handler(tauri::generate_handler![greet, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    handle.join().unwrap();
}
