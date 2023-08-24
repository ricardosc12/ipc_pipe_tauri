use std::sync::{Arc, Mutex};

struct Player {
    id: usize,
    // Outras propriedades do jogador
}

fn main() {
    // Crie o dado original
    let data: Arc<Mutex<Vec<Arc<Mutex<Player>>>>> = Arc::new(Mutex::new(vec![
        Arc::new(Mutex::new(Player { id: 1 })),
        Arc::new(Mutex::new(Player { id: 2 })),
        // ... mais jogadores aqui ...
    ]));

    // Bloqueie o mutex principal para acessar os dados
    let data_guard = data.lock().unwrap();

    // Itere sobre os jogadores (Mutex<Arc<Mutex<Player>>>) no vetor
    for player_mutex in &*data_guard {
        // Bloqueie o mutex do jogador para acessar os dados internos
        let mut player = player_mutex.lock().unwrap();
        player.id = 120;
        // Agora você pode acessar e modificar as propriedades do jogador
        // Neste exemplo, estamos apenas alterando o ID
        // player.id = novo_id;

        // ... faça as modificações necessárias ...

        // O mutex do jogador será desbloqueado automaticamente ao sair deste escopo
    }

    for player_mutex in &*data_guard {
        println!("Player {}", player_mutex.lock().unwrap().id);
    }

    // O mutex principal será desbloqueado automaticamente ao sair deste escopo
}
