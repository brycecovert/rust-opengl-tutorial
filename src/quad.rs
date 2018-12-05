use vertex::Vertex;
#[derive(Debug, Clone)]
pub struct Quad (pub Vertex, pub Vertex, pub Vertex, pub Vertex);
impl Quad {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex, v4:Vertex) -> Quad {
        Quad(v1, v2, v3, v4)
    }
    pub fn to_vertex_data(&self) -> Vec<f32> {
        vec![
            self.0.pos.0, self.0.pos.1, self.0.pos.2,
            self.0.color.0, self.0.color.1, self.0.color.2, self.0.color.3,
            self.0.uv.0, self.0.uv.1,

            self.1.pos.0, self.1.pos.1, self.1.pos.2,
            self.1.color.0, self.1.color.1, self.1.color.2, self.1.color.3,
            self.1.uv.0, self.1.uv.1,

            self.2.pos.0, self.2.pos.1, self.2.pos.2,
            self.2.color.0, self.2.color.1, self.2.color.2, self.2.color.3,
            self.2.uv.0, self.2.uv.1,

            self.3.pos.0, self.3.pos.1, self.3.pos.2,
            self.3.color.0, self.3.color.1, self.3.color.2, self.3.color.3,
            self.3.uv.0, self.3.uv.1,
        ]
    }

    pub fn add(&self, x: f32,  y: f32) -> Quad {
        let mut n = self.clone();
        n.0.pos.0 += x;
        n.1.pos.0 += x;
        n.2.pos.0 += x;
        n.3.pos.0 += x;

        n.0.pos.1 += y;
        n.1.pos.1 += y;
        n.2.pos.1 += y;
        n.3.pos.1 += y;
        n
    }
}

