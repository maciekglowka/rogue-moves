use bevy::ecs::schedule::SystemLabel;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    LoadAssets,
    MainMenu,
    MapGenerate,
    PlayerTurn,
    NPCTurn
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimationState {
    Idle,
    Animating
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum SetupLabel {
    Board,
    Units
}