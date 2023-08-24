#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct Player {
    pub nome: String,
    pub id: i32,
    pub x: i32,
    pub y: i32
}
impl Player {
    pub fn init(num: i32) -> Player {
        Player {
            nome: format!("Player {}", num).to_string(),
            id: num,
            x: 0,
            y: 0
        }
    }
}
