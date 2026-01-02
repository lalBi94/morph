use crate::coords::Coords;

pub type MonsterCampId = String;

pub struct MonsterCamp {
    id: MonsterCampId,
    coords: Coords,
    is_finished: bool,
}