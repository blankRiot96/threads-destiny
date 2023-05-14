use raylib::prelude::*;

#[derive(Debug)]
pub struct World {
    health_components: Vec<Option<i32>>,
    dmg_components: Vec<Option<i32>>,
    pos_components: Vec<Option<Vector2>>,
    name_components: Vec<Option<String>>,
}

impl World {
    pub fn new() -> Self {
        World {
            health_components: Vec::new(),
            dmg_components: Vec::new(),
            pos_components: Vec::new(),
            name_components: Vec::new(),
        }
    }

    pub fn add_entity(
        &mut self,
        health: Option<i32>,
        dmg: Option<i32>,
        pos: Option<Vector2>,
        name: Option<String>,
    ) {
        self.health_components.push(health);
        self.dmg_components.push(dmg);
        self.pos_components.push(pos);
        self.name_components.push(name);
    }

    pub fn get_entity_by_name(&self, target_name: String) -> Vec<(i32, i32, Vector2)> {
        let mut entities = Vec::new();
        let mut index: usize = 0;
        for name in &self.name_components {
            if name.as_ref().unwrap() == &target_name {
                let entity_collection: (i32, i32, Vector2) = (
                    self.health_components[index].unwrap_or_default(),
                    self.dmg_components[index].unwrap_or_default(),
                    self.pos_components[index].unwrap_or_default(),
                );
                entities.push(entity_collection);
            }
            index += 1;
        }

        entities
    }
}
