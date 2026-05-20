use vyxen_math::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RigidType {
    Circle,
    Box
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rigid {
    position: Vector2,
    linear_velocity: Vector2,
    rotation: f32,
    rotational_velocity: f32,

    density: f32,
    mass: f32,
    restitution: f32,
    area: f32,

    is_static: bool,

    shape_type: RigidType,
    radius: f32,
    width: f32,
    height: f32,
}

impl Rigid {
    pub fn get_position(&self) -> Vector2 {
        self.position
    }
    pub fn get_linear_velocity(&self) -> Vector2 {
        self.linear_velocity
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_rotational_velocity(&self) -> f32 {
        self.rotational_velocity
    }
    pub fn get_density(&self) -> f32 {
        self.density
    }
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }
    pub fn get_area(&self) -> f32 {
        self.area
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    pub fn get_shape_type(&self) -> RigidType {
        self.shape_type
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }
}

impl Rigid {
    fn new(position: Vector2, density: f32, mass: f32, restitution: f32, area: f32, is_static: bool, shape_type: RigidType, radius: f32, width: f32, height: f32) -> Self {
        Rigid {
            position,
            linear_velocity: Vector2::zero(),
            rotation: 0.0,
            rotational_velocity: 0.0,
            density,
            mass,
            restitution,
            area,
            is_static,
            shape_type,
            radius,
            width,
            height,
        }
    }

    pub fn new_circle(radius: f32, position: Vector2, density: f32, is_static: bool, restitution: f32) -> Self {
        let area = std::f32::consts::PI * radius * radius;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Circle,
            radius,
            0.0,
            0.0,
        )
    }

    pub fn new_box(width: f32, height: f32, position: Vector2, density: f32, is_static: bool, restitution: f32) -> Self {
        let area = width * height;

        Rigid::new(
            position,
            density,
            area * density,
            restitution.clamp(0.0, 1.0),
            area,
            is_static,
            RigidType::Box,
            0.0,
            width,
            height,
        )
    }

    pub fn move_by(&mut self, amount: Vector2) {
        self.position = self.position + amount;
    }

    pub fn move_to(&mut self, position: Vector2) {
        self.position = position;
    }
}