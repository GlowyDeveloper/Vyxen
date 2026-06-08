use vyxen_geometry::{aabb::AABB, shapes::{Box, Circle, Polygon, Shape}};
use vyxen_math::Vector2;

/// An enum representing the type of a rigid body.
#[derive(Debug, Clone, PartialEq)]
pub enum RigidType {
    Circle(Circle),
    Box(Box),
    Polygon(Polygon),
    Concave(Vec<Polygon>)
}

/// A struct representing a rigid body in the physics simulation.
/// 
/// # Examples
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::bodies::{Rigid, RigidType};
/// use vyxen_geometry::shapes::Circle;
/// 
/// let circle = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
/// assert!(circle.get_circle().is_some());
/// assert_eq!(circle.get_circle().unwrap().get_radius(), 1.0);
/// assert_eq!(circle.get_density(), 1.0);
/// assert_eq!(circle.get_restitution(), 0.5);
/// assert_eq!(circle.get_static_friction(), 0.6);
/// assert_eq!(circle.get_dynamic_friction(), 0.4);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Rigid {
    density: f32,
    mass: f32,
    inverse_mass: f32,
    restitution: f32,
    area: f32,

    inertia: f32,
    inverse_inertia: f32,

    static_friction: f32,
    dynamic_friction: f32,

    shape: RigidType,

    aabb: AABB,
    aabb_required: bool,
}

impl Rigid {
    /// A getter for the density of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(rigid.get_density(), 1.0);
    /// ```
    pub fn get_density(&self) -> f32 {
        self.density
    }
    /// A getter for the mass of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// 
    /// assert_eq!(rigid.get_mass(), expected_mass);
    /// ```
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    /// A getter for the inverted mass of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let area = std::f32::consts::PI;
    /// let expected_mass = area * 1.0; // area * density
    /// let expected_inverted_mass = 1.0 / expected_mass;
    /// 
    /// assert_eq!(rigid.get_inverse_mass(), expected_inverted_mass);
    /// ```
    pub fn get_inverse_mass(&self) -> f32 {
        self.inverse_mass
    }
    /// A getter for the restitution of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(rigid.get_restitution(), 0.5);
    /// ```
    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }
    /// A getter for the area of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(rigid.get_area(), std::f32::consts::PI);
    /// ```
    pub fn get_area(&self) -> f32 {
        self.area
    }
    /// A getter for the rotational inertia of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let expected_mass = std::f32::consts::PI * 1.0; // area * density
    /// let expected_inertia = (1.0 / 2.0) * expected_mass * 1.0 * 1.0; // (1/2) * mass * radius * radius
    /// 
    /// assert_eq!(rigid.get_inertia(), expected_inertia);
    /// ```
    pub fn get_inertia(&self) -> f32 {
        self.inertia
    }
    /// A getter for the inverted rotational inertia of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let expected_mass = std::f32::consts::PI * 1.0; // area * density
    /// let expected_inertia = (1.0 / 2.0) * expected_mass * 1.0 * 1.0; // (1/2) * mass * radius * radius
    /// 
    /// assert_eq!(rigid.get_inertia(), expected_inertia);
    /// 
    /// let expected_inverted_inertia = 1.0 / expected_inertia;
    /// 
    /// assert_eq!(rigid.get_inverse_inertia(), expected_inverted_inertia);
    /// ```
    pub fn get_inverse_inertia(&self) -> f32 {
        self.inverse_inertia
    }
    /// A getter for the shape of the rigid body.
    /// 
    /// If you want the mutable version, refer to `get_shape_mut()`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let shape = rigid.get_shape();
    /// let expected = RigidType::Circle(Circle::new(1.0));
    /// assert!(shape == expected);
    /// ```
    pub fn get_shape(&self) -> RigidType {
        self.shape.clone()
    }
    /// A getter for the shape of the rigid body.
    /// 
    /// If you want the mutable version, refer to `get_shape_mut()`
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let mut rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// let mut shape = rigid.get_shape_mut();
    /// let mut expected = RigidType::Circle(Circle::new(1.0));
    /// assert!(shape == &mut expected);
    /// ```
    pub fn get_shape_mut(&mut self) -> &mut RigidType {
        &mut self.shape
    }
    /// Returns `None` if the shape is a box or polygon, return `Some(Circle)` if the shape is a circle
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::{Box, Circle};
    /// 
    /// let circle = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert!(circle.get_circle().is_some());
    /// 
    /// let bx = Rigid::new(1.0, false, 0.5, Box::new(1.0, 1.0), 0.6, 0.4);
    /// assert!(bx.get_circle().is_none());
    /// ```
    pub fn get_circle(&self) -> Option<Circle> {
        match self.shape {
            RigidType::Circle(c) => Some(c),
            _ => None,
        }
    }
    /// Returns `None` if the shape is a circle or polygon, return `Some(Box)` if the shape is a box
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::{Box, Circle};
    /// 
    /// let bx = Rigid::new(1.0, false, 0.5, Box::new(1.0, 1.0), 0.6, 0.4);
    /// assert!(bx.get_box().is_some());
    /// 
    /// let circle = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert!(circle.get_box().is_none());
    /// ```
    pub fn get_box(&self) -> Option<Box> {
        match self.shape {
            RigidType::Box(b) => Some(b),
            _ => None,
        }
    }
    /// Returns `None` if the shape is a circle or box, return `Some(Polygon)` if the shape is a **convex** polygon
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::{Polygon, Circle};
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// 
    /// let convex = Rigid::new(1.0, false, 0.5, Polygon::new(&[v1, v2, v3]), 0.6, 0.4);
    /// assert!(convex.get_convex_polygon().is_some());
    /// 
    /// let v1 = Vector2 { x: 2.0, y: -2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: -2.0, y: -2.0 };
    /// let v5 = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let concave = Rigid::new(1.0, false, 0.5, Polygon::new(&[v1, v2, v3, v4, v5]), 0.6, 0.4);
    /// assert!(concave.get_convex_polygon().is_none());
    /// ```
    pub fn get_convex_polygon(&self) -> Option<Polygon> {
        match &self.shape {
            RigidType::Polygon(p) => Some(p.clone()),
            _ => None,
        }
    }
    /// Returns `None` if the shape is a circle or box or **convex** polygon, return `Some(Polygon)` if the shape is a **concave** polygon
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::{Polygon, Circle};
    /// 
    /// let v1 = Vector2 { x: 2.0, y: -2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 2.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// let v4 = Vector2 { x: -2.0, y: -2.0 };
    /// let v5 = Vector2 { x: 0.0, y: 0.0 };
    /// 
    /// let concave = Rigid::new(1.0, false, 0.5, Polygon::new(&[v1, v2, v3, v4, v5]), 0.6, 0.4);
    /// assert!(concave.get_concave_polygon().is_some());
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// 
    /// let convex = Rigid::new(1.0, false, 0.5, Polygon::new(&[v1, v2, v3]), 0.6, 0.4);
    /// assert!(convex.get_concave_polygon().is_none());
    /// ```
    pub fn get_concave_polygon(&self) -> Option<Vec<Polygon>> {
        match &self.shape {
            RigidType::Concave(p) => Some(p.clone()),
            _ => None,
        }
    }
    /// A getter for the static friction of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(rigid.get_static_friction(), 0.6);
    /// ```
    pub fn get_static_friction(&self) -> f32 {
        self.static_friction
    }
    /// A getter for the dynamic friction of the rigid body.
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::{Rigid, RigidType};
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let rigid = Rigid::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4);
    /// assert_eq!(rigid.get_dynamic_friction(), 0.4);
    /// ```
    pub fn get_dynamic_friction(&self) -> f32 {
        self.dynamic_friction
    }
}

impl Rigid {
    /// A constructor for a rigid body.
    /// 
    /// # Examples
    /// 
    /// ## Circle
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Circle;
    /// 
    /// let radius = 1.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    /// 
    /// let rigid = Rigid::new(density, is_static, restitution, Circle::new(radius), static_friction, dynamic_friction);
    /// ```
    /// 
    /// ## Box
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Box;
    /// 
    /// let width = 1.0;
    /// let height = 2.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    /// 
    /// let rigid = Rigid::new(density, is_static, restitution, Box::new(width, height), static_friction, dynamic_friction);
    /// ```
    /// 
    /// ## Polygon
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Polygon;
    /// 
    /// let width = 1.0;
    /// let height = 2.0;
    /// let density = 1.0;
    /// let is_static = false;
    /// let restitution = 0.5;
    /// let static_friction = 0.6;
    /// let dynamic_friction = 0.4;
    /// 
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    /// 
    /// let rigid = Rigid::new(density, is_static, restitution, Polygon::new(&[v1, v2, v3]), static_friction, dynamic_friction);
    /// ```
    pub fn new<T>(density: f32, is_static: bool, restitution: f32, shape: T, static_friction: f32, dynamic_friction: f32) -> Self
    where 
        T: Shape
    {
        let shape_type = Self::generate_shape_type_from_shape(shape.clone());

        let area = match () {
            _ if shape.as_any().is::<Circle>() => {
                if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
                    std::f32::consts::PI * circle.get_radius() * circle.get_radius()
                } else {
                    std::f32::consts::PI
                }
            }
            _ if shape.as_any().is::<Box>() => {
                if let Some(bx) = shape.as_any().downcast_ref::<Box>() {
                    bx.get_width() * bx.get_height()
                } else {
                    std::f32::consts::PI
                }
            }
            _ if shape.as_any().is::<Polygon>() => {
                if let Some(polygon) = shape.as_any().downcast_ref::<Polygon>() {
                    let vertices = polygon.get_vertices();

                    let mut products_1 = 0.0_f32;
                    let mut products_2 = 0.0_f32;

                    for i in 0..vertices.len() {
                        let current = vertices[i];
                        let next = vertices[(i + 1) % vertices.len()];

                        products_1 += current.x * next.y;
                        products_2 += current.y * next.x;
                    }

                    (products_1 - products_2).abs() * 0.5
                } else {
                    std::f32::consts::PI
                }
            }
            _ => std::f32::consts::PI
        };

        let mass =  area * if density <= 0.0 { 0.00001 } else { density };

        let inertia = match &shape_type {
            RigidType::Circle(c) => c.rotational_inertia(mass),
            RigidType::Box(b) => b.rotational_inertia(mass),
            RigidType::Polygon(p) => p.rotational_inertia(mass),
            RigidType::Concave(v) => {
                let mut added = vec![];
                v.iter().for_each(|p| added.push(p.rotational_inertia(mass)));

                added.iter().sum::<f32>() / added.len() as f32
            }
        };

        Rigid {
            density,
            mass,
            inverse_mass: if is_static { 0.0 } else { 1.0 / mass },
            restitution: restitution.clamp(0.0, 1.0),
            area,
            shape: shape_type,
            aabb: AABB::new_from_uncalculated(std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN),
            aabb_required: true,
            inertia,
            inverse_inertia: if is_static { 0.0 } else { 1.0 / inertia },
            static_friction,
            dynamic_friction
        }
    }

    /// Generates a rigid type from shape
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_geometry::shapes::Circle;
    /// use vyxen_physics2d::bodies::Rigid;
    /// 
    /// let rigid_type = Rigid::generate_shape_type_from_shape(Circle::new(2.0));
    /// ```
    pub fn generate_shape_type_from_shape<T>(shape: T) -> RigidType
    where 
        T: Shape
    {
        match () {
            _ if shape.as_any().is::<Circle>() => {
                if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
                    RigidType::Circle(*circle)
                } else {
                    RigidType::Circle(Circle::new(1.0))
                }
            }
            _ if shape.as_any().is::<Box>() => {
                if let Some(bx) = shape.as_any().downcast_ref::<Box>() {
                    RigidType::Box(*bx)
                } else {
                    RigidType::Circle(Circle::new(1.0))
                }
            }
            _ if shape.as_any().is::<Polygon>() => {
                if let Some(polygon) = shape.as_any().downcast_ref::<Polygon>() {
                    if polygon.is_convex() {
                        RigidType::Polygon(polygon.clone())
                    } else {
                        RigidType::Concave(Polygon::triangulate(polygon.get_vertices()))
                    }
                } else {
                    RigidType::Circle(Circle::new(1.0))
                }
            }
            _ => RigidType::Circle(Circle::new(1.0))
        }
    }

    /// Sets if another AABB calculation is required
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Box;
    /// 
    /// let mut rigid = Rigid::new(10.0, false, 1.0, Box::new(5.0, 5.0), 0.6, 0.4);
    /// 
    /// rigid.set_aabb_required(true);
    /// ```
    pub fn set_aabb_required(&mut self, required: bool) {
        self.aabb_required = required;
    }

    /// Sets if another transform is required
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Box;
    /// 
    /// let mut rigid = Rigid::new(10.0, false, 1.0, Box::new(5.0, 5.0), 0.6, 0.4);
    /// 
    /// rigid.set_transform_required(true);
    /// ```
    pub fn set_transform_required(&mut self, required: bool) {
        match self.get_shape_mut() {
            RigidType::Box(b) => b.set_transform_required(required),
            RigidType::Polygon(p) => p.set_transform_required(required),
            RigidType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(required)),
            _ => {}
        }
    }

    /// Gets the AABB for a rigid body.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::bodies::Rigid;
    /// use vyxen_geometry::shapes::Box;
    /// 
    /// let mut rigid = Rigid::new(1.0, false, 0.5, Box::new(1.0, 1.0), 0.6, 0.4);
    /// let aabb = rigid.get_aabb(Vector2 { x: 0.0, y: 0.0 }, 0.0);
    /// 
    /// assert_eq!(aabb.get_max(), Vector2 { x: 0.5, y: 0.5 });
    /// assert_eq!(aabb.get_min(), Vector2 { x: -0.5, y: -0.5 });
    /// ```
    pub fn get_aabb(&mut self, pos: Vector2, rot: f32) -> AABB {
        if self.aabb_required {
            match self.get_shape_mut() {
                RigidType::Circle(c) => {
                    self.aabb = AABB::new_from_uncalculated(
                        pos.x - c.get_radius(),
                        pos.y - c.get_radius(),
                        pos.x + c.get_radius(),
                        pos.y + c.get_radius(),
                    );
                }
                RigidType::Box(b) => {
                    let vertices = b.get_transformed_vertices(pos, rot);
                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    self.aabb = AABB::new_from_uncalculated(min_x, min_y, max_x, max_y);
                }
                RigidType::Polygon(p) => {
                    let vertices = p.get_transformed_vertices(pos, rot);
                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    self.aabb = AABB::new_from_uncalculated(min_x, min_y, max_x, max_y);
                }
                RigidType::Concave(p) => {
                    let mut vertices = vec![];
                    p.iter_mut().for_each(|p| p.get_transformed_vertices(pos, rot).iter().for_each(|v| vertices.push(v)));

                    let mut min_x = std::f32::MAX;
                    let mut max_x = std::f32::MIN;
                    let mut min_y = std::f32::MAX;
                    let mut max_y = std::f32::MIN;

                    for i in 0..vertices.len() {
                        let vertex = vertices[i];
                        if vertex.x < min_x {
                            min_x = vertex.x;
                        }
                        if vertex.x > max_x {
                            max_x = vertex.x;
                        }
                        if vertex.y < min_y {
                            min_y = vertex.y;
                        }
                        if vertex.y > max_y {
                            max_y = vertex.y;
                        }
                    }

                    self.aabb = AABB::new_from_uncalculated(min_x, min_y, max_x, max_y);
                }
                
            } 

            self.aabb_required = false;
        }

        self.aabb
    }
}