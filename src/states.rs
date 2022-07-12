use bevy::ecs::schedule::SystemLabel;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    LoadAssets,
    MainMenu,
    MapGenerate,
    Spawning,
    PlayerTurn,
    NPCTurn,
    GameOver
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimationState {
    Idle,
    Animating
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum SetupLabel {
    CleanUp
}