struct Object {
    pos : Vector3<f32>,
    prev_pos : Vector3<f32>,
    mass : f32,
    radius : f32,
    force : Vector3<f32>,
    render_object : CircleShape<'static>,
}

impl Object{
    fn get_pos_matrix(&self) -> Vector4<f32> {
        return Vector4::new(self.pos.x, self.pos.y, self.pos.z, 1.0);
    }

    fn calc_physics(&mut self, dt: f32) {
        let temp_pos = self.pos;
        self.pos = self.pos * 2.0 - self.prev_pos + ((GRAVITY + self.force) / self.mass) * dt * dt;
        self.prev_pos = temp_pos;

        // box that bitch in
        let b = 15.0;
        let p = self.pos;
        let r = self.radius;
        self.pos = Vector3::new(p.x.clamp(r - b, b - r), p.y.clamp(r - b, b - r), p.z.clamp(r - b, b - r));
        self.force = Vector3::zero();
    }

    fn calc_constraint(&mut self, other : &mut Object, distance : f32) {
        let mut offset = other.pos - self.pos;
        let magnitude = offset.magnitude();
        if magnitude == 0.0 {
            return;
        }
        offset = offset.normalize_to((distance - magnitude) / 2.0);
        self.pos = -offset + self.pos;
        other.pos = offset + other.pos;
    }

    fn new(pos : Vector3<f32>, mass : f32, radius : f32) -> Object {
        return Object {
            pos,
            prev_pos : pos,
            mass,
            radius,
            force : Vector3::zero(),
            render_object : CircleShape::new(radius as f32, 16),
        }
    }
}