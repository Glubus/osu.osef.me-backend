use rosu_v2::model::GameMode;

pub async fn is_allowed_beatmap(mode: GameMode, cs: f32) -> bool {
    if mode != GameMode::Mania {
        return false;
    }

    if cs != 4.0 {
        return false;
    }

    true
}
