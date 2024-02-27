struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

struct Particle {
    origin: Point3D,
    orientation: Point3D,
    scale: Point3D,
    impulse: Point3D,
    remaining_lifetime: f32,
}

impl Particle {
    fn create(x: f32, y: f32, z: f32, remaining_lifetime: f32) -> Particle {
        Particle { origin: Point3D { x: x, y: y, z: z },

    }
}

fn main() {
    f
}
