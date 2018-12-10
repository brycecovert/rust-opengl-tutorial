use ::quad::Quad;
use component::{Sized, Position};
use ::texture_region::{TextureRegion};
use vertex::Vertex;
use world::World;
use sprite_batch::SpriteBatch;

pub fn to_quad(p: &Position, s: &Sized, t: &TextureRegion) -> Quad {
    Quad::new(
        Vertex {
            pos: (s.width + p.x, p.y + s.height, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u2, t.v2)
        },
        Vertex {
            pos: (p.x + s.width, p.y, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u2, t.v1)
        },
        Vertex {
            pos: (p.x, p.y, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u1, t.v1)
        },
        Vertex {
            pos: (p.x, p.y + s.height, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u1, t.v2)
        },
    )
}

pub fn update(gl: &gl::Gl, world: &World, sprite_batch: &mut SpriteBatch) {
    unsafe {
        gl.Clear(gl::COLOR_BUFFER_BIT);
        world.entities
            .iter()
            .for_each(|e| {
                let (position, size, texture_regioned) = (world.positions.get(e).unwrap(), world.sizes.get(e).unwrap(), world.texture_regioned.get(e).unwrap());
                sprite_batch.draw(&texture_regioned.0, &to_quad(&position, &size, &texture_regioned.0));

                
            });
    }

    sprite_batch.flush();
}

