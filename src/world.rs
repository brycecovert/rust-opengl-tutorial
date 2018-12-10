use hashbrown::HashMap;
use entity::Entity;
use component::{Position, Velocity, TextureRegioned, Sized};

pub struct World<'a> {
    pub last_id: u32,
    pub entities: Vec<Entity>,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub sizes: HashMap<Entity, Sized>,
    pub texture_regioned: HashMap<Entity, TextureRegioned<'a>>,
}

impl <'a> World <'a> {
    pub fn new() -> World<'a> {
        World {
            last_id: 0,
            entities: Vec::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sizes: HashMap::new(),
            texture_regioned: HashMap::new(),
        }
    }
    pub fn create_entity(&mut self, p: Position, v: Velocity, s: Sized, t: TextureRegioned<'a>)  -> Entity {
        self.last_id += 1;
        let id = self.last_id;;
        let e = Entity(id);
        self.positions.insert(e, p);
        self.velocities.insert(e, v);
        self.sizes.insert(e, s);
        self.texture_regioned.insert(e, t);
        self.entities.push(e);
        e
    }
}

