use crate::{
    AABB, Context, Polygon, Scene, Sprite, Vector2,
    geometry::Shape,
    physics2d::{Manifold, RigidBody, SoftBody},
    shape_type::{ShapeType, shape_type_from_shape},
};
use std::{
    any::Any,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_NODE_ID: AtomicU64 = AtomicU64::new(1);

type ReadyOption = Option<Box<dyn FnMut(&mut Node, &mut Scene, Context)>>;
type PhysicsOption = Option<Box<dyn FnMut(&mut Node, &mut Scene, f32, Context)>>;
type CollisionOption = Option<Box<dyn FnMut(&mut Node, &mut Node, Manifold, &mut Scene, Context)>>;

/// Node struct for the scene
///
/// # Examples
/// ```rust
/// use vyxen::Node;
///
/// let node = Node::new("Foo".to_string());
/// ```
pub struct Node {
    name: String,
    id: u64,
    components: Vec<Box<dyn Component>>,
    children: Vec<u64>,

    position: Vector2,
    linear_velocity: Vector2,
    rotation: f32,
    rotational_velocity: f32,

    force: Vector2,

    is_static: bool,
    nan_logged: bool,
    last_position: Vector2,
    last_linear_velocity: Vector2,
    last_rotation: f32,
    last_rotational_velocity: f32,

    pub(crate) on_ready: ReadyOption,
    pub(crate) physics_process: PhysicsOption,
    pub(crate) on_collision: CollisionOption,

    pub(crate) physics_process_default: bool,
    pub(crate) on_collision_default: bool,
}

impl Node {
    /// Gets the id of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// let id = node.get_id();
    /// ```
    pub fn get_id(&self) -> u64 {
        self.id
    }
    /// Sets the id of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.set_id(10);
    /// assert_eq!(10, node.get_id());
    /// ```
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
    /// Gets the position of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_position(), Vector2 { x: 0.0, y: 0.0 });
    /// node.move_to(Vector2 { x: 10.0, y: 10.0 });
    /// assert_eq!(node.get_position(), Vector2 { x: 10.0, y: 10.0 });
    /// ```
    pub fn get_position(&self) -> Vector2 {
        self.position
    }
    /// Gets the linear velocity of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_linear_velocity(), Vector2 { x: 0.0, y: 0.0 });
    /// ```
    pub fn get_linear_velocity(&self) -> Vector2 {
        self.linear_velocity
    }
    /// Gets the rotation of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_rotation(), 0.0);
    /// node.rotate_to(45.0);
    /// assert_eq!(node.get_rotation(), 45.0);
    /// ```
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    /// Gets the rotational velocity of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_rotational_velocity(), 0.0);
    /// ```
    pub fn get_rotational_velocity(&self) -> f32 {
        self.rotational_velocity
    }
    /// Gets the force of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_force(), Vector2 { x: 0.0, y: 0.0 });
    /// ```
    pub fn get_force(&self) -> Vector2 {
        self.force
    }

    /// Gets the name of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// assert_eq!(node.get_name(), "Foo");
    /// ```
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the children's ids of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut parent = Node::new("Parent".to_string());
    /// let child = Node::new("Child".to_string());
    /// parent.add_child(child.get_id());
    ///
    /// let ids = parent.get_children_ids();
    /// assert_eq!(ids.len(), 1);
    /// ```
    pub fn get_children_ids(&self) -> &Vec<u64> {
        &self.children
    }

    /// Returns the amount of children the node has
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut parent = Node::new("Parent".to_string());
    /// let child1 = Node::new("Child1".to_string());
    /// parent.add_child(child1.get_id());
    ///
    /// assert_eq!(parent.get_children_len(), 1);
    ///
    /// let child2 = Node::new("Child2".to_string());
    /// parent.add_child(child2.get_id());
    ///
    /// assert_eq!(parent.get_children_len(), 2);
    /// ```
    pub fn get_children_len(&self) -> usize {
        self.children.len()
    }

    /// Gets the static status of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// assert_eq!(node.is_static(), false);
    /// ```
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Sets the `on_ready` function of the node
    ///
    /// Fields are:
    ///  - `Node` (the current node)
    ///  - `Scene` (the scene that the node is in)
    ///  - `Context` (the information of the current game)
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_on_ready(|node, scene, context| {
    ///     println!("{}", node.get_name());
    /// });
    /// ```
    pub fn set_on_ready<F>(&mut self, script: F)
    where
        F: FnMut(&mut Node, &mut Scene, Context) + 'static,
    {
        self.on_ready = Some(Box::new(script));
    }

    /// Resets the `on_ready` function of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_on_ready(|node, scene, context| {
    ///     println!("{}", node.get_name());
    /// });
    /// node.reset_on_ready();
    /// ```
    pub fn reset_on_ready(&mut self) {
        self.on_ready = None;
    }

    /// Sets the `physics_process` function of the node
    ///
    /// Fields are:
    ///  - `Node` (the current node)
    ///  - `Scene` (the scene that the node is in)
    ///  - `f32` (the time between the last two frames)
    ///  - `Context` (the information of the current game)
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_on_ready(|node, scene, context| {
    ///     println!("{}", node.get_name());
    /// });
    /// ```
    pub fn set_physics_process<F>(&mut self, script: F)
    where
        F: FnMut(&mut Node, &mut Scene, f32, Context) + 'static,
    {
        self.physics_process = Some(Box::new(script));
    }

    /// Resets the `physics_process` function of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_physics_process(|node, scene, dt, context| {
    ///     println!("{} | dt: {}", node.get_name(), dt);
    /// });
    /// node.reset_physics_process();
    /// ```
    pub fn reset_physics_process(&mut self) {
        self.physics_process = None;
    }

    /// Sets the `on_collision` function of the node
    ///
    /// Fields are:
    ///  - `Node` (the current node)
    ///  - `Node` (the other node that was collided with)
    ///  - `Manifold` (all information about the collision)
    ///  -  Scene` (the scene that the node is in)
    ///  - `Context` (the information of the current game)
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_on_collision(|node, other, manifold, scene, context| {
    ///     println!("{} collided with {}", node.get_name(), other.get_name());
    /// });
    /// node.reset_on_collision();
    /// ```
    pub fn set_on_collision<F>(&mut self, script: F)
    where
        F: FnMut(&mut Node, &mut Node, Manifold, &mut Scene, Context) + 'static,
    {
        self.on_collision = Some(Box::new(script));
    }

    /// Resets the `on_collision` function of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Node".to_string());
    /// node.set_on_collision(|node, other, manifold, scene, context| {
    ///     println!("{} collided with {}", node.get_name(), other.get_name());
    /// });
    /// node.reset_on_collision();
    /// ```
    pub fn reset_on_collision(&mut self) {
        self.on_collision = None;
    }

    /// Turns on and off if the default function for physics processing should be ran, even with an override.
    pub fn set_physics_process_default(&mut self, set: bool) {
        self.physics_process_default = set;
    }

    /// Turns on and off if the default function for collision resolving should be ran, even with an override.
    pub fn set_on_collision_default(&mut self, set: bool) {
        self.on_collision_default = set;
    }

    /// Sets the name of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Foo".to_string());
    ///
    /// assert_eq!(node.get_name(), "Foo");
    ///
    /// node.set_name("Bar".to_string());
    ///
    /// assert_eq!(node.get_name(), "Bar");
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the linear velocity of the node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let start_pos = Vector2 { x: 0.0, y: 0.0 };
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.set_linear_velocity(Vector2 { x: 5.0, y: 0.0 });
    /// ```
    pub fn set_linear_velocity(&mut self, velocity: Vector2) {
        self.last_linear_velocity = self.linear_velocity;
        if !velocity.x.is_finite() || !velocity.y.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }

            if self.last_linear_velocity.x.is_finite() && self.last_linear_velocity.y.is_finite() {
                self.linear_velocity = self.last_linear_velocity;
            } else {
                self.linear_velocity = Vector2::zero();
            }
            return;
        }
        self.linear_velocity = velocity;
    }
    /// Sets the rotational velocity of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.set_rotational_velocity(45.0);
    /// ```
    pub fn set_rotational_velocity(&mut self, amount: f32) {
        self.last_rotational_velocity = self.rotational_velocity;
        if !amount.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_rotational_velocity.is_finite() {
                self.rotational_velocity = self.last_rotational_velocity;
            } else {
                self.rotational_velocity = 0.0;
            }
            return;
        }
        self.rotational_velocity = amount;
    }
    /// Sets the rotational velocity of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// assert_eq!(false, node.is_static());
    /// node.set_is_static(true);
    /// assert_eq!(true, node.is_static());
    /// ```
    pub fn set_is_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    /// Adds a child node to the current node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut parent = Node::new("Parent".to_string());
    /// let child = Node::new("Child".to_string());
    /// parent.add_child(child.get_id());
    ///
    /// assert_eq!(parent.get_children_len(), 1);
    /// ```
    pub fn add_child(&mut self, child: u64) {
        self.children.push(child);
    }

    /// Removes a child node from the current node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let mut parent = Node::new("Parent".to_string());
    /// let child = Node::new("Child".to_string());
    /// parent.add_child(child.get_id());
    ///
    /// assert_eq!(parent.get_children_len(), 1);
    ///
    /// parent.remove_child(child.get_id());
    ///
    /// assert_eq!(parent.get_children_len(), 0);
    /// ```
    pub fn remove_child(&mut self, id: u64) {
        if let Some(index) = self.children.iter().position(|c| c == &id) {
            self.children.remove(index);
        }
    }
}

impl Node {
    /// Generates a new node with the given name
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: NEXT_NODE_ID.fetch_add(1, Ordering::Relaxed),
            components: Vec::new(),
            children: Vec::new(),
            position: Vector2::zero(),
            linear_velocity: Vector2::zero(),
            rotation: 0.0,
            rotational_velocity: 0.0,
            force: Vector2::zero(),
            is_static: false,
            nan_logged: false,
            last_position: Vector2::zero(),
            last_linear_velocity: Vector2::zero(),
            last_rotation: 0.0,
            last_rotational_velocity: 0.0,
            on_ready: None,
            physics_process: None,
            on_collision: None,
            physics_process_default: true,
            on_collision_default: true,
        }
    }

    /// Add a component to this node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(2.0)));
    /// ```
    pub fn add_component<T: Component + 'static>(&mut self, comp: T) {
        self.components.push(Box::new(comp));
    }

    /// Remove the first component of type `T`.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(2.0)));
    ///
    /// node.remove_component::<Collider>();
    /// ```
    pub fn remove_component<T: 'static>(&mut self) {
        if let Some(pos) = self
            .components
            .iter()
            .position(|c| c.as_any().downcast_ref::<T>().is_some())
        {
            self.components.remove(pos);
        }
    }

    /// Gets a component of type `T`.
    ///
    /// For a mutable reference, refer to `get_component_mut()`.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle, physics2d::RigidBody};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(2.0)));
    ///
    /// let collider = node.get_component::<Collider>();
    ///
    /// assert!(collider.is_some());
    ///
    /// let rigid = node.get_component::<RigidBody>();
    ///
    /// assert!(rigid.is_none());
    /// ```
    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        for c in &self.components {
            if let Some(v) = c.as_any().downcast_ref::<T>() {
                return Some(v);
            }
        }
        None
    }

    /// Gets a component of type `T` as a a mutable reference.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle, physics2d::RigidBody};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(2.0)));
    ///
    /// let mut collider = node.get_component_mut::<Collider>();
    ///
    /// assert!(collider.is_some());
    ///
    /// let mut rigid = node.get_component_mut::<RigidBody>();
    ///
    /// assert!(rigid.is_none());
    /// ```
    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        for c in &mut self.components {
            if let Some(v) = c.as_any_mut().downcast_mut::<T>() {
                return Some(v);
            }
        }
        None
    }

    /// Moves the node by a given amount.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.move_by(Vector2 { x: 1.0, y: 1.0 });
    /// assert_eq!(node.get_position(), Vector2 { x: 1.0, y: 1.0 });
    /// ```
    pub fn move_by(&mut self, amount: Vector2) {
        if self.position.x.is_finite()
            && self.position.y.is_finite()
            && self.linear_velocity.x.is_finite()
            && self.linear_velocity.y.is_finite()
            && self.rotation.is_finite()
            && self.rotational_velocity.is_finite()
        {
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        let new_pos = self.position + amount;
        if !new_pos.x.is_finite() || !new_pos.y.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_position.x.is_finite() && self.last_position.y.is_finite() {
                self.position = self.last_position;
                self.linear_velocity = self.last_linear_velocity;
                self.rotational_velocity = self.last_rotational_velocity;
            } else {
                self.position = Vector2::zero();
                self.linear_velocity = Vector2::zero();
                self.rotational_velocity = 0.0;
            }
        } else {
            self.position = new_pos;
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        if let Some(rigid) = self.get_component_mut::<RigidBody>() {
            match rigid.get_shape_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(collider) = self.get_component_mut::<Collider>() {
            match collider.get_hitbox_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }
    }

    /// Moves the node to a given position.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.move_to(Vector2 { x: 3.0, y: 4.0 });
    /// assert_eq!(node.get_position(), Vector2 { x: 3.0, y: 4.0 });
    /// ```
    pub fn move_to(&mut self, position: Vector2) {
        if self.position.x.is_finite()
            && self.position.y.is_finite()
            && self.linear_velocity.x.is_finite()
            && self.linear_velocity.y.is_finite()
            && self.rotation.is_finite()
            && self.rotational_velocity.is_finite()
        {
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        if !position.x.is_finite() || !position.y.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_position.x.is_finite() && self.last_position.y.is_finite() {
                self.position = self.last_position;
            }
            return;
        }

        self.position = position;

        self.last_position = self.position;
        self.last_linear_velocity = self.linear_velocity;
        self.last_rotation = self.rotation;
        self.last_rotational_velocity = self.rotational_velocity;

        if let Some(rigid) = self.get_component_mut::<RigidBody>() {
            match rigid.get_shape_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(collider) = self.get_component_mut::<Collider>() {
            match collider.get_hitbox_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }
    }

    /// Rotates the node by a given amount
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.rotate_by(45.0);
    /// assert_eq!(node.get_rotation(), 45.0);
    /// ```
    pub fn rotate_by(&mut self, amount: f32) {
        if self.position.x.is_finite()
            && self.position.y.is_finite()
            && self.linear_velocity.x.is_finite()
            && self.linear_velocity.y.is_finite()
            && self.rotation.is_finite()
            && self.rotational_velocity.is_finite()
        {
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        let new_rot = self.rotation + amount;
        if !new_rot.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_rotation.is_finite() {
                self.rotation = self.last_rotation;
                self.rotational_velocity = self.last_rotational_velocity;
            } else {
                self.rotation = 0.0;
                self.rotational_velocity = 0.0;
            }
        } else {
            self.rotation = new_rot;

            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        if let Some(rigid) = self.get_component_mut::<RigidBody>() {
            match rigid.get_shape_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(collider) = self.get_component_mut::<Collider>() {
            match collider.get_hitbox_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }
    }

    /// Rotates the node to a given amount.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.rotate_to(45.0);
    /// assert_eq!(node.get_rotation(), 45.0);
    /// node.rotate_to(21.0);
    /// assert_eq!(node.get_rotation(), 21.0);
    /// ```
    pub fn rotate_to(&mut self, amount: f32) {
        if self.position.x.is_finite()
            && self.position.y.is_finite()
            && self.linear_velocity.x.is_finite()
            && self.linear_velocity.y.is_finite()
            && self.rotation.is_finite()
            && self.rotational_velocity.is_finite()
        {
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        if !amount.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_rotation.is_finite() {
                self.rotation = self.last_rotation;
                self.rotational_velocity = self.last_rotational_velocity;
            }
            return;
        }

        self.rotation = amount;

        self.last_rotation = self.rotation;
        self.last_rotational_velocity = self.rotational_velocity;

        if let Some(rigid) = self.get_component_mut::<RigidBody>() {
            match rigid.get_shape_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(collider) = self.get_component_mut::<Collider>() {
            match collider.get_hitbox_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }
    }

    /// Adds an amount to the force of the node
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let force = Vector2 { x: 5.0, y: 0.0 };
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_force(force);
    /// assert_eq!(node.get_force(), force);
    /// node.add_force(force);
    /// assert_eq!(node.get_force(), force * 2.0);
    /// ```
    pub fn add_force(&mut self, force: Vector2) {
        self.force = self.force + force;
    }

    /// Sets the force of the node to an amount
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Vector2};
    ///
    /// let force = Vector2 { x: 5.0, y: 0.0 };
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.set_force(force);
    /// assert_eq!(node.get_force(), force);
    /// node.set_force(force * 2.0);
    /// assert_eq!(node.get_force(), force * 2.0);
    /// ```
    pub fn set_force(&mut self, force: Vector2) {
        self.force = force;
    }

    fn resolve_rigid_rigid(node_a: &mut Node, node_b: &mut Node, manifold: Manifold) {
        let (inv_mass_a, inv_mass_b, inv_inertia_a, inv_inertia_b, sf, df, e) = {
            let body_a = if let Some(rigid) = node_a.get_component::<RigidBody>() {
                rigid
            } else {
                return;
            };
            let body_b = if let Some(rigid) = node_b.get_component::<RigidBody>() {
                rigid
            } else {
                return;
            };

            let sf = (body_a.get_static_friction() + body_b.get_static_friction()) / 2.0;
            let df = (body_a.get_dynamic_friction() + body_b.get_dynamic_friction()) / 2.0;
            let e = body_a.get_restitution().min(body_b.get_restitution());

            (
                if node_a.is_static() {
                    0.0
                } else {
                    body_a.get_inverse_mass()
                },
                if node_b.is_static() {
                    0.0
                } else {
                    body_b.get_inverse_mass()
                },
                if node_a.is_static() {
                    0.0
                } else {
                    body_a.get_inverse_inertia()
                },
                if node_b.is_static() {
                    0.0
                } else {
                    body_b.get_inverse_inertia()
                },
                sf,
                df,
                e,
            )
        };

        let normal = manifold.get_normal();
        let depth = manifold.get_depth();
        let contact_1 = manifold.get_contact_1();
        let contact_2 = manifold.get_contact_2();
        let contact_count = if contact_2.is_none() { 1 } else { 2 };

        let mut impulse_vec: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut friction_impulse_vec: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];

        let mut ra: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut rb: Vec<Vector2> = vec![Vector2::zero(), Vector2::zero()];
        let mut js: Vec<f32> = vec![0.0, 0.0];

        for i in 0..contact_count {
            let contact = if i == 0 { contact_1 } else { contact_2 };
            if contact.is_none() {
                continue;
            }

            ra[i] = contact.unwrap() - node_a.get_position();
            rb[i] = contact.unwrap() - node_b.get_position();

            let ra_prep = Vector2 {
                x: -ra[i].y,
                y: ra[i].x,
            };
            let rb_prep = Vector2 {
                x: -rb[i].y,
                y: rb[i].x,
            };

            let rotation_velocity_body_a = ra_prep * node_a.get_rotational_velocity();
            let rotation_velocity_body_b = rb_prep * node_b.get_rotational_velocity();

            let relative_velocity = (node_b.get_linear_velocity() + rotation_velocity_body_b)
                - (node_a.get_linear_velocity() + rotation_velocity_body_a);

            let velocity_magnitude = relative_velocity.dot(&normal);

            if velocity_magnitude > 0.0 {
                continue;
            }

            let ra_prep_dot_n = ra_prep.dot(&normal);
            let rb_prep_dot_n = rb_prep.dot(&normal);

            let denomenator = inv_mass_a
                + inv_mass_b
                + (ra_prep_dot_n * ra_prep_dot_n) * inv_inertia_a
                + (rb_prep_dot_n * rb_prep_dot_n) * inv_inertia_b;

            if !denomenator.is_finite() || denomenator == 0.0 {
                js[i] = 0.0;
                impulse_vec[i] = Vector2::zero();
                continue;
            }

            let mut j = -(1.0 + e) * velocity_magnitude;
            j /= denomenator;
            j /= contact_count as f32;

            js[i] = j;

            let impulse = normal * j;
            impulse_vec[i] = impulse
        }

        for i in 0..contact_count {
            let impulse = impulse_vec[i];
            node_a.set_linear_velocity(node_a.get_linear_velocity() + -impulse * inv_mass_a);
            node_a.set_rotational_velocity(
                node_a.get_rotational_velocity() + -ra[i].cross(&impulse) * inv_inertia_a,
            );
            node_b.set_linear_velocity(node_b.get_linear_velocity() + impulse * inv_mass_b);
            node_b.set_rotational_velocity(
                node_b.get_rotational_velocity() + rb[i].cross(&impulse) * inv_inertia_b,
            );
        }

        for i in 0..contact_count {
            let contact = if i == 0 { contact_1 } else { contact_2 };
            if contact.is_none() {
                continue;
            }

            ra[i] = contact.unwrap() - node_a.get_position();
            rb[i] = contact.unwrap() - node_b.get_position();

            let ra_prep = Vector2 {
                x: -ra[i].y,
                y: ra[i].x,
            };
            let rb_prep = Vector2 {
                x: -rb[i].y,
                y: rb[i].x,
            };

            let rotation_velocity_body_a = ra_prep * node_a.get_rotational_velocity();
            let rotation_velocity_body_b = rb_prep * node_b.get_rotational_velocity();

            let relative_velocity = (node_b.get_linear_velocity() + rotation_velocity_body_b)
                - (node_a.get_linear_velocity() + rotation_velocity_body_a);

            let tangent = relative_velocity - normal * relative_velocity.dot(&normal);
            if tangent.is_nearly_equal(&Vector2::zero()) {
                continue;
            }
            let tangent_normalized = tangent.normalize();

            let ra_prep_dot_t = ra_prep.dot(&tangent_normalized);
            let rb_prep_dot_t = rb_prep.dot(&tangent_normalized);

            let denomenator = inv_mass_a
                + inv_mass_b
                + (ra_prep_dot_t * ra_prep_dot_t) * inv_inertia_a
                + (rb_prep_dot_t * rb_prep_dot_t) * inv_inertia_b;

            let mut jt = -relative_velocity.dot(&tangent_normalized);
            if !denomenator.is_finite() || denomenator == 0.0 {
                jt = 0.0;
            } else {
                jt /= denomenator;
                jt /= contact_count as f32;
            }

            let impulse = if jt.abs() <= js[i] * sf {
                tangent_normalized * jt
            } else {
                tangent_normalized * -js[i] * df
            };

            friction_impulse_vec[i] = impulse
        }

        for i in 0..contact_count {
            let impulse = friction_impulse_vec[i];
            node_a.set_linear_velocity(node_a.get_linear_velocity() + -impulse * inv_mass_a);
            node_a.set_rotational_velocity(
                node_a.get_rotational_velocity() + -ra[i].cross(&impulse) * inv_inertia_a,
            );
            node_b.set_linear_velocity(node_b.get_linear_velocity() + impulse * inv_mass_b);
            node_b.set_rotational_velocity(
                node_b.get_rotational_velocity() + rb[i].cross(&impulse) * inv_inertia_b,
            );
        }

        let correction_mag = ((depth - 0.01).max(0.0)) * 0.2;
        let correction = normal * correction_mag;
        let total_inv_mass = inv_mass_b + inv_mass_a;

        if !total_inv_mass.is_finite() || total_inv_mass == 0.0 {
            return;
        }

        if node_a.is_static() {
            node_b.move_by(correction * inv_mass_b / total_inv_mass);
        } else if node_b.is_static() {
            node_a.move_by(-correction * inv_mass_a / total_inv_mass);
        } else {
            node_a.move_by(-correction * inv_mass_a / total_inv_mass);
            node_b.move_by(correction * inv_mass_b / total_inv_mass);
        }
    }

    fn resolve_soft_soft(node_a: &mut Node, node_b: &mut Node, manifold: Manifold) {
        let (a_inv_mass, a_inv_inertia, sf_a, df_a, e_a, count_a) =
            if let Some(s) = node_a.get_component::<SoftBody>() {
                let c = s.get_points().len() as f32;
                (
                    if node_a.is_static() {
                        0.0
                    } else {
                        s.get_inverse_mass() * c
                    },
                    if node_a.is_static() {
                        0.0
                    } else {
                        s.get_inverse_inertia()
                    },
                    s.get_static_friction(),
                    s.get_dynamic_friction(),
                    s.get_restitution(),
                    c,
                )
            } else {
                return;
            };

        let (b_inv_mass, b_inv_inertia, sf_b, df_b, e_b, count_b) =
            if let Some(s) = node_b.get_component::<SoftBody>() {
                let c = s.get_points().len() as f32;
                (
                    if node_b.is_static() {
                        0.0
                    } else {
                        s.get_inverse_mass() * c
                    },
                    if node_b.is_static() {
                        0.0
                    } else {
                        s.get_inverse_inertia()
                    },
                    s.get_static_friction(),
                    s.get_dynamic_friction(),
                    s.get_restitution(),
                    c,
                )
            } else {
                return;
            };

        let sf = (sf_a + sf_b) / 2.0;
        let df = (df_a + df_b) / 2.0;
        let e = e_a.min(e_b);

        let normal = manifold.get_normal();
        let depth = manifold.get_depth();

        let contact_count = if manifold.get_contact_2().is_some() {
            2.0
        } else {
            1.0
        };

        let a_pos = node_a.get_position();
        let a_rot = node_a.get_rotation();
        let a_cos = a_rot.cos();
        let a_sin = a_rot.sin();

        let b_pos = node_b.get_position();
        let b_rot = node_b.get_rotation();
        let b_cos = b_rot.cos();
        let b_sin = b_rot.sin();

        for contact in [manifold.get_contact_1(), manifold.get_contact_2()]
            .into_iter()
            .flatten()
        {
            let mut idx_a = 0;
            let mut min_dist_a = f32::MAX;
            if let Some(soft) = node_a.get_component::<SoftBody>() {
                for (i, p) in soft.get_points().iter().enumerate() {
                    let local_p = p.get_position();
                    let scene_pos = Vector2 {
                        x: a_pos.x + local_p.x * a_cos - local_p.y * a_sin,
                        y: a_pos.y + local_p.x * a_sin + local_p.y * a_cos,
                    };
                    let dist = scene_pos.distance_squared(&contact);
                    if dist < min_dist_a {
                        min_dist_a = dist;
                        idx_a = i;
                    }
                }
            }

            let mut idx_b = 0;
            let mut min_dist_b = f32::MAX;
            if let Some(soft) = node_b.get_component::<SoftBody>() {
                for (i, p) in soft.get_points().iter().enumerate() {
                    let local_p = p.get_position();
                    let scene_pos = Vector2 {
                        x: b_pos.x + local_p.x * b_cos - local_p.y * b_sin,
                        y: b_pos.y + local_p.x * b_sin + local_p.y * b_cos,
                    };
                    let dist = scene_pos.distance_squared(&contact);
                    if dist < min_dist_b {
                        min_dist_b = dist;
                        idx_b = i;
                    }
                }
            }

            let r_a = contact - node_a.get_position();
            let r_a_prep = Vector2 {
                x: -r_a.y,
                y: r_a.x,
            };
            let r_b = contact - node_b.get_position();
            let r_b_prep = Vector2 {
                x: -r_b.y,
                y: r_b.x,
            };

            let total_inv_mass = a_inv_mass + b_inv_mass;
            if total_inv_mass > 0.0 {
                let correction_mag = (depth - 0.01).max(0.0) * 0.2 / contact_count;
                let correction = normal * correction_mag;

                if !node_a.is_static() {
                    if let Some(soft) = node_a.get_component_mut::<SoftBody>() {
                        let point = &mut soft.get_points_mut()[idx_a];
                        let c = -correction * (a_inv_mass / total_inv_mass);
                        let local_c = Vector2 {
                            x: c.x * a_cos + c.y * a_sin,
                            y: -c.x * a_sin + c.y * a_cos,
                        };
                        point.set_position(point.get_position() + local_c);
                        node_a.move_by(c / count_a);
                    }
                }
                if !node_b.is_static() {
                    if let Some(soft) = node_b.get_component_mut::<SoftBody>() {
                        let point = &mut soft.get_points_mut()[idx_b];
                        let c = correction * (b_inv_mass / total_inv_mass);
                        let local_c = Vector2 {
                            x: c.x * b_cos + c.y * b_sin,
                            y: -c.x * b_sin + c.y * b_cos,
                        };
                        point.set_position(point.get_position() + local_c);
                        node_b.move_by(c / count_b);
                    }
                }
            }

            let vel_a = if let Some(soft) = node_a.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[idx_a].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * a_cos - p_local_vel.y * a_sin,
                    y: p_local_vel.x * a_sin + p_local_vel.y * a_cos,
                };
                node_a.get_linear_velocity()
                    + r_a_prep * node_a.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_b = if let Some(soft) = node_b.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[idx_b].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * b_cos - p_local_vel.y * b_sin,
                    y: p_local_vel.x * b_sin + p_local_vel.y * b_cos,
                };
                node_b.get_linear_velocity()
                    + r_b_prep * node_b.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_rel = vel_b - vel_a;
            let vel_along_normal = vel_rel.dot(&normal);

            if vel_along_normal > 0.0 {
                continue;
            }

            let r_a_prep_dot_n = r_a_prep.dot(&normal);
            let r_b_prep_dot_n = r_b_prep.dot(&normal);

            let denom = a_inv_mass
                + b_inv_mass
                + (r_a_prep_dot_n * r_a_prep_dot_n) * a_inv_inertia
                + (r_b_prep_dot_n * r_b_prep_dot_n) * b_inv_inertia;

            if denom == 0.0 {
                continue;
            }

            let mut restitution = e;
            if vel_along_normal.abs() < 10.0 {
                restitution = 0.0;
            }

            let j = -(1.0 + restitution) * vel_along_normal / denom / contact_count;
            let normal_impulse = normal * j;

            if !node_a.is_static() {
                if let Some(soft) = node_a.get_component_mut::<SoftBody>() {
                    let point = &mut soft.get_points_mut()[idx_a];
                    let local_impulse = Vector2 {
                        x: -normal_impulse.x * a_cos - normal_impulse.y * a_sin,
                        y: normal_impulse.x * a_sin - normal_impulse.y * a_cos,
                    };

                    point.set_velocity(point.get_velocity() + local_impulse * a_inv_mass);
                    node_a.set_linear_velocity(
                        node_a.get_linear_velocity() - normal_impulse * (a_inv_mass / count_a),
                    );
                    node_a.set_rotational_velocity(
                        node_a.get_rotational_velocity()
                            - r_a.cross(&normal_impulse) * a_inv_inertia,
                    );
                }
            }
            if !node_b.is_static() {
                if let Some(soft) = node_b.get_component_mut::<SoftBody>() {
                    let point = &mut soft.get_points_mut()[idx_b];
                    let local_impulse = Vector2 {
                        x: normal_impulse.x * b_cos + normal_impulse.y * b_sin,
                        y: -normal_impulse.x * b_sin + normal_impulse.y * b_cos,
                    };

                    point.set_velocity(point.get_velocity() + local_impulse * b_inv_mass);
                    node_b.set_linear_velocity(
                        node_b.get_linear_velocity() + normal_impulse * (b_inv_mass / count_b),
                    );
                    node_b.set_rotational_velocity(
                        node_b.get_rotational_velocity()
                            + r_b.cross(&normal_impulse) * b_inv_inertia,
                    );
                }
            }

            let vel_a_new = if let Some(soft) = node_a.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[idx_a].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * a_cos - p_local_vel.y * a_sin,
                    y: p_local_vel.x * a_sin + p_local_vel.y * a_cos,
                };
                node_a.get_linear_velocity()
                    + r_a_prep * node_a.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_b_new = if let Some(soft) = node_b.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[idx_b].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * b_cos - p_local_vel.y * b_sin,
                    y: p_local_vel.x * b_sin + p_local_vel.y * b_cos,
                };
                node_b.get_linear_velocity()
                    + r_b_prep * node_b.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_rel_new = vel_b_new - vel_a_new;
            let tangent = vel_rel_new - normal * vel_rel_new.dot(&normal);

            if tangent.length_squared() > f32::EPSILON {
                let tangent_norm = tangent.normalize();
                let r_a_prep_dot_t = r_a_prep.dot(&tangent_norm);
                let r_b_prep_dot_t = r_b_prep.dot(&tangent_norm);

                let denom_t = a_inv_mass
                    + b_inv_mass
                    + (r_a_prep_dot_t * r_a_prep_dot_t) * a_inv_inertia
                    + (r_b_prep_dot_t * r_b_prep_dot_t) * b_inv_inertia;

                if denom_t > 0.0 {
                    let jt = -vel_rel_new.dot(&tangent_norm) / denom_t / contact_count;
                    let friction_impulse = if jt.abs() <= j * sf {
                        tangent_norm * jt
                    } else {
                        tangent_norm * -j * df
                    };

                    if !node_a.is_static() {
                        if let Some(soft) = node_a.get_component_mut::<SoftBody>() {
                            let point = &mut soft.get_points_mut()[idx_a];
                            let local_f_impulse = Vector2 {
                                x: -friction_impulse.x * a_cos - friction_impulse.y * a_sin,
                                y: friction_impulse.x * a_sin - friction_impulse.y * a_cos,
                            };

                            point.set_velocity(point.get_velocity() + local_f_impulse * a_inv_mass);
                            node_a.set_linear_velocity(
                                node_a.get_linear_velocity()
                                    - friction_impulse * (a_inv_mass / count_a),
                            );
                            node_a.set_rotational_velocity(
                                node_a.get_rotational_velocity()
                                    - r_a.cross(&friction_impulse) * a_inv_inertia,
                            );
                        }
                    }
                    if !node_b.is_static() {
                        if let Some(soft) = node_b.get_component_mut::<SoftBody>() {
                            let point = &mut soft.get_points_mut()[idx_b];
                            let local_f_impulse = Vector2 {
                                x: friction_impulse.x * b_cos + friction_impulse.y * b_sin,
                                y: -friction_impulse.x * b_sin + friction_impulse.y * b_cos,
                            };

                            point.set_velocity(point.get_velocity() + local_f_impulse * b_inv_mass);
                            node_b.set_linear_velocity(
                                node_b.get_linear_velocity()
                                    + friction_impulse * (b_inv_mass / count_b),
                            );
                            node_b.set_rotational_velocity(
                                node_b.get_rotational_velocity()
                                    + r_b.cross(&friction_impulse) * b_inv_inertia,
                            );
                        }
                    }
                }
            }
        }
    }

    fn resolve_rigid_soft(
        rigid_node: &mut Node,
        soft_node: &mut Node,
        manifold: Manifold,
        is_rigid_a: bool,
    ) {
        let (r_inv_mass, r_inv_inertia, sf_r, df_r, e_r) =
            if let Some(r) = rigid_node.get_component::<RigidBody>() {
                (
                    if rigid_node.is_static() {
                        0.0
                    } else {
                        r.get_inverse_mass()
                    },
                    if rigid_node.is_static() {
                        0.0
                    } else {
                        r.get_inverse_inertia()
                    },
                    r.get_static_friction(),
                    r.get_dynamic_friction(),
                    r.get_restitution(),
                )
            } else {
                return;
            };

        let (s_inv_mass, s_inv_inertia, sf_s, df_s, e_s, point_count) =
            if let Some(s) = soft_node.get_component::<SoftBody>() {
                let count = s.get_points().len() as f32;
                (
                    if soft_node.is_static() {
                        0.0
                    } else {
                        s.get_inverse_mass() * count
                    },
                    if soft_node.is_static() {
                        0.0
                    } else {
                        s.get_inverse_inertia()
                    },
                    s.get_static_friction(),
                    s.get_dynamic_friction(),
                    s.get_restitution(),
                    count,
                )
            } else {
                return;
            };

        let sf = (sf_r + sf_s) / 2.0;
        let df = (df_r + df_s) / 2.0;
        let e = e_r.min(e_s);

        let mut normal = manifold.get_normal();

        if !is_rigid_a {
            normal = -normal;
        }

        let depth = manifold.get_depth();

        let contact_count = if manifold.get_contact_2().is_some() {
            2.0
        } else {
            1.0
        };

        let s_pos = soft_node.get_position();
        let s_rot = soft_node.get_rotation();
        let s_cos = s_rot.cos();
        let s_sin = s_rot.sin();

        for contact in [manifold.get_contact_1(), manifold.get_contact_2()]
            .into_iter()
            .flatten()
        {
            let mut closest_idx = 0;
            let mut min_dist = f32::MAX;

            if let Some(soft) = soft_node.get_component::<SoftBody>() {
                for (i, p) in soft.get_points().iter().enumerate() {
                    let local_p = p.get_position();
                    let scene_pos = Vector2 {
                        x: s_pos.x + local_p.x * s_cos - local_p.y * s_sin,
                        y: s_pos.y + local_p.x * s_sin + local_p.y * s_cos,
                    };

                    let dist = scene_pos.distance_squared(&contact);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_idx = i;
                    }
                }
            }

            let total_inv_mass = r_inv_mass + s_inv_mass;
            if total_inv_mass > 0.0 {
                let correction_mag = (depth - 0.01).max(0.0) * 0.2 / contact_count;
                let correction = normal * correction_mag;

                if !rigid_node.is_static() {
                    rigid_node.move_by(-correction * (r_inv_mass / total_inv_mass));
                }

                if !soft_node.is_static() {
                    if let Some(soft) = soft_node.get_component_mut::<SoftBody>() {
                        let point = &mut soft.get_points_mut()[closest_idx];
                        let soft_correction = correction * (s_inv_mass / total_inv_mass);

                        let local_correction = Vector2 {
                            x: soft_correction.x * s_cos + soft_correction.y * s_sin,
                            y: -soft_correction.x * s_sin + soft_correction.y * s_cos,
                        };

                        point.set_position(point.get_position() + local_correction);
                        soft_node.move_by(soft_correction / point_count);
                    }
                }
            }

            let r_rigid = contact - rigid_node.get_position();
            let r_rigid_prep = Vector2 {
                x: -r_rigid.y,
                y: r_rigid.x,
            };
            let r_soft = contact - soft_node.get_position();
            let r_soft_prep = Vector2 {
                x: -r_soft.y,
                y: r_soft.x,
            };

            let vel_rigid = rigid_node.get_linear_velocity()
                + r_rigid_prep * rigid_node.get_rotational_velocity();

            let vel_soft = if let Some(soft) = soft_node.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[closest_idx].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * s_cos - p_local_vel.y * s_sin,
                    y: p_local_vel.x * s_sin + p_local_vel.y * s_cos,
                };
                soft_node.get_linear_velocity()
                    + r_soft_prep * soft_node.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_rel = vel_soft - vel_rigid;
            let vel_along_normal = vel_rel.dot(&normal);

            if vel_along_normal > 0.0 {
                continue;
            }

            let r_rigid_prep_dot_n = r_rigid_prep.dot(&normal);
            let r_soft_prep_dot_n = r_soft_prep.dot(&normal);

            let denom = r_inv_mass
                + s_inv_mass
                + (r_rigid_prep_dot_n * r_rigid_prep_dot_n) * r_inv_inertia
                + (r_soft_prep_dot_n * r_soft_prep_dot_n) * s_inv_inertia;

            if denom == 0.0 {
                continue;
            }

            let mut restitution = e;
            if vel_along_normal.abs() < 10.0 {
                restitution = 0.0;
            }

            let j = -(1.0 + restitution) * vel_along_normal / denom / contact_count;
            let normal_impulse = normal * j;

            if !rigid_node.is_static() {
                rigid_node.set_linear_velocity(
                    rigid_node.get_linear_velocity() - normal_impulse * r_inv_mass,
                );
                rigid_node.set_rotational_velocity(
                    rigid_node.get_rotational_velocity()
                        - r_rigid.cross(&normal_impulse) * r_inv_inertia,
                );
            }

            if !soft_node.is_static() {
                if let Some(soft) = soft_node.get_component_mut::<SoftBody>() {
                    let point = &mut soft.get_points_mut()[closest_idx];

                    let local_normal_impulse = Vector2 {
                        x: normal_impulse.x * s_cos + normal_impulse.y * s_sin,
                        y: -normal_impulse.x * s_sin + normal_impulse.y * s_cos,
                    };

                    point.set_velocity(point.get_velocity() + local_normal_impulse * s_inv_mass);
                    soft_node.set_linear_velocity(
                        soft_node.get_linear_velocity()
                            + normal_impulse * (s_inv_mass / point_count),
                    );
                    soft_node.set_rotational_velocity(
                        soft_node.get_rotational_velocity()
                            + r_soft.cross(&normal_impulse) * s_inv_inertia,
                    );
                }
            }

            let vel_rigid_new = rigid_node.get_linear_velocity()
                + r_rigid_prep * rigid_node.get_rotational_velocity();
            let vel_soft_new = if let Some(soft) = soft_node.get_component::<SoftBody>() {
                let p_local_vel = soft.get_points()[closest_idx].get_velocity();
                let p_scene_vel = Vector2 {
                    x: p_local_vel.x * s_cos - p_local_vel.y * s_sin,
                    y: p_local_vel.x * s_sin + p_local_vel.y * s_cos,
                };
                soft_node.get_linear_velocity()
                    + r_soft_prep * soft_node.get_rotational_velocity()
                    + p_scene_vel
            } else {
                Vector2::zero()
            };

            let vel_rel_new = vel_soft_new - vel_rigid_new;
            let tangent = vel_rel_new - normal * vel_rel_new.dot(&normal);

            if tangent.length_squared() > f32::EPSILON {
                let tangent_norm = tangent.normalize();
                let r_rigid_prep_dot_t = r_rigid_prep.dot(&tangent_norm);
                let r_soft_prep_dot_t = r_soft_prep.dot(&tangent_norm);

                let denom_t = r_inv_mass
                    + s_inv_mass
                    + (r_rigid_prep_dot_t * r_rigid_prep_dot_t) * r_inv_inertia
                    + (r_soft_prep_dot_t * r_soft_prep_dot_t) * s_inv_inertia;

                if denom_t > 0.0 {
                    let jt = -vel_rel_new.dot(&tangent_norm) / denom_t / contact_count;
                    let friction_impulse = if jt.abs() <= j * sf {
                        tangent_norm * jt
                    } else {
                        tangent_norm * -j * df
                    };

                    if !rigid_node.is_static() {
                        rigid_node.set_linear_velocity(
                            rigid_node.get_linear_velocity() - friction_impulse * r_inv_mass,
                        );
                        rigid_node.set_rotational_velocity(
                            rigid_node.get_rotational_velocity()
                                - r_rigid.cross(&friction_impulse) * r_inv_inertia,
                        );
                    }

                    if !soft_node.is_static() {
                        if let Some(soft) = soft_node.get_component_mut::<SoftBody>() {
                            let point = &mut soft.get_points_mut()[closest_idx];

                            let local_friction_impulse = Vector2 {
                                x: friction_impulse.x * s_cos + friction_impulse.y * s_sin,
                                y: -friction_impulse.x * s_sin + friction_impulse.y * s_cos,
                            };

                            point.set_velocity(
                                point.get_velocity() + local_friction_impulse * s_inv_mass,
                            );
                            soft_node.set_linear_velocity(
                                soft_node.get_linear_velocity()
                                    + friction_impulse * (s_inv_mass / point_count),
                            );
                            soft_node.set_rotational_velocity(
                                soft_node.get_rotational_velocity()
                                    + r_soft.cross(&friction_impulse) * s_inv_inertia,
                            );
                        }
                    }
                }
            }
        }
    }

    /// The default for when the node is collided
    pub fn on_collision_default(node_a: &mut Node, node_b: &mut Node, manifold: Manifold) {
        let is_a_rigid = node_a.get_component::<RigidBody>().is_some();
        let is_b_rigid = node_b.get_component::<RigidBody>().is_some();

        let is_a_soft = node_a.get_component::<SoftBody>().is_some();
        let is_b_soft = node_b.get_component::<SoftBody>().is_some();

        if is_a_rigid && is_b_rigid {
            Self::resolve_rigid_rigid(node_a, node_b, manifold);
        } else if is_a_rigid && is_b_soft {
            Self::resolve_rigid_soft(node_a, node_b, manifold, true);
        } else if is_a_soft && is_b_rigid {
            Self::resolve_rigid_soft(node_b, node_a, manifold, false);
        } else if is_a_soft && is_b_soft {
            Self::resolve_soft_soft(node_a, node_b, manifold);
        }
    }

    /// The default for when the node is stepped
    pub fn physics_process_default(&mut self, gravity: Vector2, dt: f32) {
        if self.is_static {
            return;
        }

        if self.position.x.is_finite()
            && self.position.y.is_finite()
            && self.linear_velocity.x.is_finite()
            && self.linear_velocity.y.is_finite()
            && self.rotation.is_finite()
            && self.rotational_velocity.is_finite()
        {
            self.last_position = self.position;
            self.last_linear_velocity = self.linear_velocity;
            self.last_rotation = self.rotation;
            self.last_rotational_velocity = self.rotational_velocity;
        }

        let new_linear_velocity = self.linear_velocity + gravity * dt;
        if !new_linear_velocity.x.is_finite() || !new_linear_velocity.y.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_linear_velocity.x.is_finite() && self.last_linear_velocity.y.is_finite() {
                self.linear_velocity = self.last_linear_velocity;
            } else {
                self.linear_velocity = Vector2::zero();
            }
        } else {
            self.linear_velocity = new_linear_velocity;
            self.last_linear_velocity = self.linear_velocity;
        }

        let new_pos = self.position + self.force + self.linear_velocity * dt;
        if !new_pos.x.is_finite() || !new_pos.y.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_position.x.is_finite() && self.last_position.y.is_finite() {
                self.position = self.last_position;
                self.linear_velocity = self.last_linear_velocity;
                self.rotational_velocity = self.last_rotational_velocity;
            } else {
                self.position = Vector2::zero();
                self.linear_velocity = Vector2::zero();
                self.rotational_velocity = 0.0;
            }
        } else {
            self.position = new_pos;
            self.last_position = self.position;
        }

        let new_rot = self.rotation + self.rotational_velocity * dt;
        if !new_rot.is_finite() {
            if !self.nan_logged {
                self.nan_logged = true;
            }
            if self.last_rotation.is_finite() {
                self.rotation = self.last_rotation;
                self.rotational_velocity = self.last_rotational_velocity;
            } else {
                self.rotation = 0.0;
                self.rotational_velocity = 0.0;
            }
        } else {
            self.rotation = new_rot;
            self.last_rotation = self.rotation;
        }

        self.force = Vector2::zero();

        if let Some(rigid) = self.get_component_mut::<RigidBody>() {
            match rigid.get_shape_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(collider) = self.get_component_mut::<Collider>() {
            match collider.get_hitbox_mut() {
                ShapeType::Box(b) => b.set_transform_required(true),
                ShapeType::Polygon(p) => p.set_transform_required(true),
                ShapeType::Concave(c) => c.iter_mut().for_each(|p| p.set_transform_required(true)),
                _ => {}
            }
        }

        if let Some(soft) = self.get_component_mut::<SoftBody>() {
            soft.solve_springs(dt);

            for point in soft.get_points_mut() {
                point.step(dt);
            }

            let new_vertices: Vec<Vector2> =
                soft.get_points().iter().map(|p| p.get_position()).collect();
            if let Some(collider) = self.get_component_mut::<Collider>() {
                *collider.get_hitbox_mut() = ShapeType::Polygon(Polygon::new(&new_vertices));
                collider.set_uninitilized();
            }
            if let Some(sprite) = self.get_component_mut::<Sprite>() {
                sprite.set_shape(Polygon::new(&new_vertices));
            }
        }
    }
}

/// A trait that structs can implement to be
/// added to a node
pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Allows nodes to collide with eachother.
///
/// # Examples
/// ```rust
/// use vyxen::{Node, Collider, Circle};
///
/// let mut node = Node::new("Foo".to_string());
/// node.add_component(Collider::new(Circle::new(5.0)));
/// ```
pub struct Collider {
    hitbox: ShapeType,
    aabb: AABB,
    old_pos: Vector2,
    old_rot: f32,
    aabb_initialized: bool,
}

impl Collider {
    /// Creates a collider
    ///
    /// # Examples
    /// ## Circles:
    /// ```rust
    /// use vyxen::{Node, Collider, Circle};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(5.0)));
    /// ```
    /// ## Boxes:
    /// ```rust
    /// use vyxen::{Node, Collider, Box};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Box::new(5.0, 5.0)));
    /// ```
    /// ## Polygons:
    /// ```rust
    /// use vyxen::{Node, Collider, Polygon, Vector2};
    ///
    /// let v1 = Vector2 { x: 0.0, y: 2.0 };
    /// let v2 = Vector2 { x: 2.0, y: 0.0 };
    /// let v3 = Vector2 { x: -2.0, y: 2.0 };
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Polygon::new(&[v1, v2, v3])));
    /// ```
    pub fn new<T>(hitbox: T) -> Self
    where
        T: Shape,
    {
        Self {
            hitbox: shape_type_from_shape(hitbox),
            aabb: AABB::new_from_uncalculated(f32::MAX, f32::MAX, f32::MIN, f32::MIN),
            old_pos: Vector2::zero(),
            old_rot: 0.0,
            aabb_initialized: false,
        }
    }

    /// Gets the hitbox
    ///
    /// For a mutable reference, refer to `get_hitbox_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    ///
    /// let collider = node.get_component::<Collider>().unwrap();
    ///
    /// let hitbox = collider.get_hitbox();
    /// ```
    pub fn get_hitbox(&self) -> &ShapeType {
        &self.hitbox
    }

    /// Gets the hitbox as a mutable reference
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    ///
    /// let mut collider = node.get_component_mut::<Collider>().unwrap();
    ///
    /// let hitbox = collider.get_hitbox_mut();
    /// ```
    pub fn get_hitbox_mut(&mut self) -> &mut ShapeType {
        &mut self.hitbox
    }

    /// Generates an AABB of the Collider
    ///
    /// # Examples
    /// ```rust
    /// use vyxen::{Node, Collider, Circle, Vector2};
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let collider = Collider::new(Circle::new(5.0));
    /// node.add_component(collider);
    ///
    /// let mut collider = node.get_component_mut::<Collider>().unwrap();
    ///
    /// let aabb = collider.get_aabb(Vector2 { x: 0.0, y: 0.0 }, 45.0);
    /// ```
    pub fn get_aabb(&mut self, pos: Vector2, rot: f32) -> AABB {
        if !self.aabb_initialized || self.old_pos != pos || self.old_rot != rot {
            let aabb = match &mut self.hitbox {
                ShapeType::Circle(c) => AABB::new_from_uncalculated(
                    pos.x - c.get_radius(),
                    pos.y - c.get_radius(),
                    pos.x + c.get_radius(),
                    pos.y + c.get_radius(),
                ),
                ShapeType::Box(b) => {
                    b.set_transform_required(true);
                    let vertices = b.get_transformed_vertices(pos, rot);
                    let mut min_x = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut min_y = f32::MAX;
                    let mut max_y = f32::MIN;

                    for vertex in vertices {
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

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
                ShapeType::Polygon(p) => {
                    p.set_transform_required(true);
                    let vertices = p.get_transformed_vertices(pos, rot);
                    let mut min_x = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut min_y = f32::MAX;
                    let mut max_y = f32::MIN;

                    for vertex in vertices {
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

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
                ShapeType::Concave(p) => {
                    p.iter_mut().for_each(|p| p.set_transform_required(true));
                    let mut vertices = vec![];
                    p.iter_mut().for_each(|p| {
                        p.get_transformed_vertices(pos, rot)
                            .iter()
                            .for_each(|v| vertices.push(*v))
                    });

                    let mut min_x = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut min_y = f32::MAX;
                    let mut max_y = f32::MIN;

                    for vertex in vertices {
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

                    AABB::new_from_uncalculated(min_x, min_y, max_x, max_y)
                }
            };
            self.old_pos = pos;
            self.old_rot = rot;
            self.aabb = aabb;
            self.aabb_initialized = true;
            self.aabb
        } else {
            self.aabb
        }
    }

    pub(crate) fn set_uninitilized(&mut self) {
        self.aabb_initialized = false;
    }
}

impl Component for Collider {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Circle;

    #[test]
    fn test_node_creation_and_movement() {
        let mut node = Node::new("Entity".to_string());
        assert_eq!(node.get_name(), "Entity");
        assert_eq!(node.get_position(), Vector2 { x: 0.0, y: 0.0 });

        // Test absolute movement
        node.move_to(Vector2 { x: 10.0, y: 5.0 });
        assert_eq!(node.get_position(), Vector2 { x: 10.0, y: 5.0 });

        // Test relative movement
        node.move_by(Vector2 { x: -2.0, y: 3.0 });
        assert_eq!(node.get_position(), Vector2 { x: 8.0, y: 8.0 });
    }

    #[test]
    fn test_node_rotation() {
        let mut node = Node::new("Rotator".to_string());
        assert_eq!(node.get_rotation(), 0.0);

        node.rotate_to(90.0);
        assert_eq!(node.get_rotation(), 90.0);

        node.rotate_by(45.0);
        assert_eq!(node.get_rotation(), 135.0);
    }

    #[test]
    fn test_node_hierarchy() {
        let mut parent = Node::new("Parent".to_string());
        let child1 = Node::new("Child1".to_string());
        let child2 = Node::new("Child2".to_string());

        let c1_id = child1.get_id();
        let c2_id = child2.get_id();

        parent.add_child(c1_id);
        parent.add_child(c2_id);

        assert_eq!(parent.get_children_len(), 2);

        let children_ids = parent.get_children_ids();
        assert!(children_ids.contains(&c1_id));
        assert!(children_ids.contains(&c2_id));

        parent.remove_child(c1_id);
        assert_eq!(parent.get_children_len(), 1);
        assert!(!parent.get_children_ids().contains(&c1_id));
    }

    #[test]
    fn test_component_system() {
        let mut node = Node::new("Collidable".to_string());
        let collider = Collider::new(Circle::new(5.0));

        node.add_component(collider);

        // Component should exist
        assert!(node.get_component::<Collider>().is_some());

        // Removing component
        node.remove_component::<Collider>();
        assert!(node.get_component::<Collider>().is_none());
    }

    #[test]
    fn test_node_physics_properties() {
        let mut node = Node::new("PhysicsNode".to_string());

        node.set_linear_velocity(Vector2 { x: 5.0, y: -2.0 });
        assert_eq!(node.get_linear_velocity(), Vector2 { x: 5.0, y: -2.0 });

        node.set_rotational_velocity(15.0);
        assert_eq!(node.get_rotational_velocity(), 15.0);

        let force = Vector2 { x: 10.0, y: 10.0 };
        node.add_force(force);
        assert_eq!(node.get_force(), force);

        node.set_is_static(true);
        assert!(node.is_static());
    }
}
