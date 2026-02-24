use raylib::{math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D};

pub enum EnemyState {
    
    //enemy owns hand and boxes
    
    StartTurn,
    RollingDice,
    ChoosingDice,
    TallyingTotal,
    Acting,
    WaitingForPlayer,
    Resetting,
}

pub struct EnemyData {
    pub health: i64,
    pub pos: Vector2,
    pub state: EnemyState,
}
