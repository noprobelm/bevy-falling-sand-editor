use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ParticleMaterialLabels {
    pub walls: Vec<String>,
    pub solids: Vec<String>,
    pub movable_solids: Vec<String>,
    pub liquids: Vec<String>,
    pub gases: Vec<String>,
    pub insects: Vec<String>,
    pub other: Vec<String>,
}

impl ParticleMaterialLabels {
    pub fn categories(&self) -> impl Iterator<Item = (&str, &Vec<String>)> {
        [
            ("Walls", &self.walls),
            ("Solids", &self.solids),
            ("Movable Solids", &self.movable_solids),
            ("Liquids", &self.liquids),
            ("Gases", &self.gases),
            ("Insects", &self.insects),
            ("Other", &self.other),
        ]
        .into_iter()
    }
}
