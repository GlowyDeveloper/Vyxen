#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{any::Any, collections::HashMap, path::PathBuf, sync::Arc};

use vyxen_geometry::{AABB, Polygon, Shape, ShapeType, shape_type_from_shape};
use vyxen_input::{Inputs, KeyCode, KeyState, MouseInput, TouchPhase};
use vyxen_math::{Random, Vector2};
use vyxen_physics2d::{Collision, ContactPoints, Manifold, RigidBody, SoftBody};
use vyxen_renderer::{Sprite, WindowConfig, WindowEvent, backend::State};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::WindowId,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

/// Scene to hold nodes in the game
///
/// # Examples
/// ```rust
/// use vyxen_core::{Scene, Node, Game};
/// use vyxen_math::Vector2;
/// use vyxen_physics2d::RigidBody;
/// use vyxen_geometry::Circle;
///
/// let mut scene = Scene::new();
///
/// let mut node = Node::new("Foo".to_string());
/// let id = node.get_id();
/// node.add_component(RigidBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4));
///
/// scene.add_node(node);
///
/// assert_eq!(2, scene.get_nodes_len());
///
/// scene.remove_node_by_id(id);
///
/// assert_eq!(1, scene.get_nodes_len());
///
/// let mut game = Game::new();
///
/// game.load_scene(scene);
/// ```
pub struct Scene {
    nodes: HashMap<u64, Node>,
    contact_pairs: Vec<(usize, usize)>,
    manifolds: Vec<Manifold>,
    gravity: Vector2,
    iterations: usize,
    aabbs: Vec<AABB>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// Generates a new scene
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::RigidBody;
    /// use vyxen_geometry::Circle;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let id = node.get_id();
    /// node.add_component(RigidBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4));
    ///
    /// scene.add_node(node);
    ///
    /// assert_eq!(2, scene.get_nodes_len());
    ///
    /// scene.remove_node_by_id(id);
    ///
    /// assert_eq!(1, scene.get_nodes_len());
    /// ```
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let mut root = Node::new("Root".to_string());
        root.set_id(0);
        nodes.insert(0, root);

        Self {
            nodes,
            contact_pairs: Vec::new(),
            manifolds: Vec::new(),
            gravity: Vector2 { x: 0.0, y: -9.81 },
            iterations: 10,
            aabbs: Vec::new(),
        }
    }

    /// Gets the scene root as a mutable reference
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let root = scene.get_root_mut();
    /// ```
    pub fn get_root_mut(&mut self) -> &mut Node {
        self.nodes.get_mut(&0).unwrap()
    }

    /// Gets the scene root as a reference
    ///
    /// For a mutable reference, refer to `get_root_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let scene = Scene::new();
    ///
    /// let root = scene.get_root();
    /// ```
    pub fn get_root(&self) -> &Node {
        self.nodes.get(&0).unwrap()
    }

    /// Gets the nodes of the scene as a reference
    ///
    /// For a mutable reference, refer to `get_nodes_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let scene = Scene::new();
    ///
    /// let nodes = scene.get_nodes();
    ///
    /// assert_eq!(1, nodes.len());
    /// ```
    pub fn get_nodes(&self) -> &HashMap<u64, Node> {
        &self.nodes
    }

    /// Gets the nodes of the scene as a mutable reference
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let nodes = scene.get_nodes_mut();
    ///
    /// assert_eq!(1, nodes.len());
    /// ```
    pub fn get_nodes_mut(&mut self) -> &mut HashMap<u64, Node> {
        &mut self.nodes
    }

    /// Gets a node from the scene by id
    ///
    /// For a mutable reference, refer to `get_node_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let node_id = node.get_id();
    /// scene.add_node(node);
    ///
    /// let node = scene.get_node(node_id).unwrap();
    ///
    /// assert_eq!(node.get_id(), node_id);
    /// ```
    pub fn get_node(&self, id: u64) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Gets a node from the scene by id as a mutable reference
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let node_id = node.get_id();
    /// scene.add_node(node);
    ///
    /// let node = scene.get_node_mut(node_id).unwrap();
    ///
    /// assert_eq!(node.get_id(), node_id);
    /// ```
    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Add a node as a child of the root.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::Circle;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// scene.add_node(node);
    /// ```
    pub fn add_node(&mut self, node: Node) {
        let id = node.get_id();
        self.nodes.insert(id, node);
        if let Some(root) = self.nodes.get_mut(&0) {
            root.add_child(id);
        }
    }

    /// Removes the node from the scene with all of its children
    ///
    /// Wrapper for `remove_node_by_id()`.
    pub fn remove_node(&mut self, node: &Node) -> anyhow::Result<()> {
        self.remove_node_by_id(node.get_id())
    }

    /// Removes the node from the scene by id with all of its children
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::Circle;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node1 = Node::new("Foo".to_string());
    /// let node1_id = node1.get_id();
    ///
    /// let mut node2 = Node::new("Bar".to_string());
    /// let node2_id = node2.get_id();
    ///
    /// scene.add_node(node1);
    /// scene.add_node(node2);
    ///
    /// {
    ///     let node1_copy = scene.get_node_mut(node1_id).unwrap();
    ///     node1_copy.add_child(node2_id);
    /// }
    ///
    /// assert_eq!(3, scene.get_nodes_len());
    ///
    /// scene.remove_node_by_id(node1_id);
    ///
    /// assert_eq!(1, scene.get_nodes_len());
    /// ```
    pub fn remove_node_by_id(&mut self, id: u64) -> anyhow::Result<()> {
        if id == 0 {
            anyhow::bail!("Root node cannot be removed.")
        }

        if let Some(node) = self.nodes.remove(&id) {
            let child_ids: Vec<u64> = node.get_children_ids().to_vec();
            for child_id in child_ids {
                self.remove_node_by_id(child_id)?;
            }
        }

        Ok(())
    }

    /// Gets the len of the amount of nodes in the scene.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node};
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::Circle;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node1 = Node::new("Foo".to_string());
    /// let node1_id = node1.get_id();
    ///
    /// let mut node2 = Node::new("Bar".to_string());
    /// let node2_id = node2.get_id();
    ///
    /// scene.add_node(node1);
    /// scene.add_node(node2);
    ///
    /// {
    ///     let node1_copy = scene.get_node_mut(node1_id).unwrap();
    ///     node1_copy.add_child(node2_id);
    /// }
    ///
    /// assert_eq!(3, scene.get_nodes_len());
    /// ```
    pub fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the gravity of the scene
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    /// use vyxen_math::Vector2;
    ///
    /// let scene = Scene::new();
    ///
    /// assert_eq!(Vector2 { x: 0.0, y: -9.81 }, scene.get_gravity());
    /// ```
    pub fn get_gravity(&self) -> Vector2 {
        self.gravity
    }

    /// Sets the gravity of the scene
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    /// use vyxen_math::Vector2;
    ///
    /// let mut scene = Scene::new();
    ///
    /// assert_eq!(Vector2 { x: 0.0, y: -9.81 }, scene.get_gravity());
    ///
    /// scene.set_gravity(Vector2 { x: 0.0, y: 9.81 });
    ///
    /// assert_eq!(Vector2 { x: 0.0, y: 9.81 }, scene.get_gravity());
    /// ```
    pub fn set_gravity(&mut self, g: Vector2) {
        self.gravity = g;
    }

    /// Gets the iterations of the scene
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let mut scene = Scene::new();
    ///
    /// assert_eq!(10, scene.get_iterations());
    ///
    /// scene.set_iterations(20);
    ///
    /// assert_eq!(20, scene.get_iterations());
    /// ```
    pub fn get_iterations(&self) -> usize {
        self.iterations
    }

    /// Sets the iterations of the scene
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Scene;
    ///
    /// let mut scene = Scene::new();
    ///
    /// assert_eq!(10, scene.get_iterations());
    ///
    /// scene.set_iterations(20);
    ///
    /// assert_eq!(20, scene.get_iterations());
    /// ```
    pub fn set_iterations(&mut self, iterations: usize) {
        self.iterations = iterations;
    }

    /// Calculates a single game step
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Node, Context};
    /// use vyxen_math::Vector2;
    /// use vyxen_physics2d::RigidBody;
    /// use vyxen_geometry::Circle;
    /// use vyxen_input::Inputs;
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(RigidBody::new(1.0, false, 0.5, Circle::new(1.0), 0.6, 0.4));
    /// scene.add_node(node);
    ///
    /// scene.step(
    ///     0.1,
    ///     Context {
    ///         inputs: Inputs::new(),
    ///         cursor_pos: Vector2::zero(),
    ///         config: WindowConfig::new()
    ///     }
    /// );
    /// ```
    pub fn step(&mut self, dt: f32, ctx: Context) {
        let ids_snapshot: Vec<u64> = self.nodes.keys().cloned().collect();

        for id in ids_snapshot.iter() {
            if let Some(mut node) = self.nodes.remove(id) {
                if let Some(mut callback) = node.physics_process.take() {
                    callback(&mut node, self, dt, ctx.clone());
                    node.physics_process = Some(callback);
                    if node.physics_process_default {
                        node.physics_process_default(self.gravity, dt);
                    }
                } else {
                    node.physics_process_default(self.gravity, dt);
                }
                self.nodes.insert(*id, node);
            }
        }

        let node_ids: Vec<u64> = self.nodes.keys().cloned().collect();

        self.contact_pairs.clear();
        self.manifolds.clear();
        self.broad_phase(&node_ids);

        self.narrow_phase(&node_ids);

        let manifolds_snapshot = self.manifolds.clone();

        for _ in 0..self.iterations {
            for manifold in manifolds_snapshot.iter().copied() {
                let a_idx = manifold.get_body_a_index();
                let b_idx = manifold.get_body_b_index();

                if a_idx >= node_ids.len() || b_idx >= node_ids.len() {
                    continue;
                }

                let id_a = node_ids[a_idx];
                let id_b = node_ids[b_idx];

                let mut node_a = match self.nodes.remove(&id_a) {
                    Some(n) => n,
                    None => continue,
                };

                let mut node_b = match self.nodes.remove(&id_b) {
                    Some(n) => n,
                    None => {
                        self.nodes.insert(id_a, node_a);
                        continue;
                    }
                };

                let mut called = false;

                if let Some(mut callback) = node_a.on_collision.take() {
                    called = true;
                    callback(&mut node_a, &mut node_b, manifold, self, ctx.clone());
                    node_a.on_collision = Some(callback);
                }

                if let Some(mut callback) = node_b.on_collision.take() {
                    called = true;
                    callback(&mut node_b, &mut node_a, manifold, self, ctx.clone());
                    node_b.on_collision = Some(callback);
                }

                if !called || (node_a.on_collision_default && node_b.on_collision_default) {
                    Node::on_collision_default(&mut node_a, &mut node_b, manifold);
                }
            }
        }
    }

    fn broad_phase(&mut self, node_ids: &[u64]) {
        self.aabbs.clear();
        self.aabbs.reserve(node_ids.len());

        for id in node_ids.iter() {
            if let Some(node) = self.nodes.get_mut(id) {
                let pos = node.get_position();
                let rot = node.get_rotation();

                let aabb = if let Some(collider) = node.get_component_mut::<Collider>() {
                    collider.get_aabb(pos, rot)
                } else {
                    AABB::new_from_uncalculated(f32::MAX, f32::MAX, f32::MIN, f32::MIN)
                };

                let min = aabb.get_min();
                let max = aabb.get_max();
                let sanitized = if !(min.x.is_finite()
                    && min.y.is_finite()
                    && max.x.is_finite()
                    && max.y.is_finite())
                {
                    let eps = 0.001;
                    AABB::new_from_uncalculated(pos.x - eps, pos.y - eps, pos.x + eps, pos.y + eps)
                } else {
                    aabb
                };

                self.aabbs.push(sanitized);
            } else {
                self.aabbs.push(AABB::new_from_uncalculated(
                    f32::MAX,
                    f32::MAX,
                    f32::MIN,
                    f32::MIN,
                ));
            }
        }

        let mut indices: Vec<usize> = (0..self.aabbs.len()).collect();
        indices.sort_unstable_by(|&i, &j| {
            self.aabbs[i]
                .get_min()
                .x
                .total_cmp(&self.aabbs[j].get_min().x)
        });

        for s in 0..indices.len() {
            let i = indices[s];
            let max_x = self.aabbs[i].get_max().x;

            for j in indices.iter().skip(s + 1) {
                if self.aabbs[*j].get_min().x > max_x {
                    break;
                }
                if AABB::intersect_aabb(self.aabbs[i], self.aabbs[*j]) {
                    self.contact_pairs.push((i, *j));
                }
            }
        }
    }

    fn narrow_phase(&mut self, node_ids: &[u64]) {
        let pairs = std::mem::take(&mut self.contact_pairs);
        self.manifolds.clear();

        for (ia, ib) in pairs {
            if ia >= node_ids.len() || ib >= node_ids.len() {
                continue;
            }

            let id_a = node_ids[ia];
            let id_b = node_ids[ib];

            let node_a_opt = self.nodes.remove(&id_a);
            let node_b_opt = self.nodes.remove(&id_b);

            if node_a_opt.is_none() || node_b_opt.is_none() {
                if let Some(n) = node_a_opt {
                    self.nodes.insert(id_a, n);
                }
                if let Some(n) = node_b_opt {
                    self.nodes.insert(id_b, n);
                }
                continue;
            }

            let mut node_a = node_a_opt.unwrap();
            let mut node_b = node_b_opt.unwrap();

            let pos_a = node_a.get_position();
            let rot_a = node_a.get_rotation();
            let pos_b = node_b.get_position();
            let rot_b = node_b.get_rotation();

            let collider_a = if let Some(c) = node_a.get_component_mut::<Collider>() {
                c
            } else {
                self.nodes.insert(id_a, node_a);
                self.nodes.insert(id_b, node_b);
                continue;
            };

            let collider_b = if let Some(c) = node_b.get_component_mut::<Collider>() {
                c
            } else {
                self.nodes.insert(id_a, node_a);
                self.nodes.insert(id_b, node_b);
                continue;
            };

            let collisions = Collision::collide(
                collider_a.get_hitbox_mut(),
                pos_a,
                rot_a,
                collider_b.get_hitbox_mut(),
                pos_b,
                rot_b,
            );

            for collision in collisions {
                let contacts = ContactPoints::find_contact_points(
                    collider_a.get_hitbox_mut(),
                    pos_a,
                    rot_a,
                    collider_b.get_hitbox_mut(),
                    pos_b,
                    rot_b,
                );

                self.manifolds.push(Manifold::new(
                    ia,
                    ib,
                    collision.normal,
                    collision.depth,
                    contacts.contact_1,
                    contacts.contact_2,
                ));
            }

            self.nodes.insert(id_a, node_a);
            self.nodes.insert(id_b, node_b);
        }
    }
}

type Callback = Box<dyn FnMut(&mut Game, &ActiveEventLoop, Event)>;

/// Game struct to hold everything
///
/// # Examples
/// ```rust
/// use vyxen_core::{Scene, Game};
///
/// let scene = Scene::new();
///
/// let mut game = Game::new();
///
/// game.load_scene(scene);
/// ```
pub struct Game {
    loaded_scene: Option<Scene>,
    state: Option<State>,
    callback: Option<Callback>,
    ctx: Context,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Game struct to hold everything
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    /// ```
    pub fn new() -> Self {
        Self {
            loaded_scene: None,
            state: None,
            callback: None,
            ctx: Context {
                inputs: Inputs::new(),
                cursor_pos: Vector2::zero(),
                config: WindowConfig::new(),
            },
        }
    }

    /// Steps the scene.
    ///
    /// Ran automatically in `run()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.step(0.1);
    /// ```
    pub fn step(&mut self, dt: f32) {
        if let Some(scene) = &mut self.loaded_scene {
            scene.step(dt, self.ctx.clone());
        }
    }

    /// Gets the loaded scene.
    ///
    /// If you want the mutable version, refer to `get_scene_mut()`
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// let loaded = game.get_scene().unwrap();
    /// ```
    pub fn get_scene(&self) -> Option<&Scene> {
        self.loaded_scene.as_ref()
    }

    /// Gets the loaded scene mutably.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// let mut loaded = game.get_scene_mut().unwrap();
    /// ```
    pub fn get_scene_mut(&mut self) -> Option<&mut Scene> {
        self.loaded_scene.as_mut()
    }

    /// Loads a new scene.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    /// ```
    pub fn load_scene(&mut self, scene: Scene) {
        self.loaded_scene = Some(scene);
    }

    /// Sets a new config
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Scene, Game};
    /// use vyxen_renderer::WindowConfig;
    ///
    /// let mut game = Game::new();
    ///
    /// let mut conf = WindowConfig::new();
    /// conf.set_title("Foobar".to_string());
    ///
    /// game.set_config(conf);
    /// ```
    pub fn set_config(&mut self, config: WindowConfig) {
        self.ctx.config = config;
    }

    /// Updates the sprites for the renderer.
    ///
    /// Called automatically by `run()`.
    pub fn update_sprites(&mut self) {
        if let Some(scene) = &mut self.loaded_scene {
            let sprites = scene
                .get_nodes_mut()
                .iter_mut()
                .filter_map(|(_, node)| {
                    let pos = node.get_position();
                    let rot = node.get_rotation();

                    if let Some(sprite) = node.get_component_mut::<Sprite>() {
                        sprite.set_position(pos);
                        sprite.set_rotation(rot);
                        Some(sprite.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Sprite>>();

            if let Some(state) = self.state.as_mut() {
                state.set_sprites(sprites);
            }
        }
    }

    /// Runs the game.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.run(|_, _, _| {
    ///     println!("callback"); // Called every frame
    /// });
    /// ```
    pub fn run<F>(mut self, callback: F) -> anyhow::Result<()>
    where
        F: FnMut(&mut Game, &ActiveEventLoop, Event) + 'static,
    {
        let event_loop = EventLoop::new()?;
        self.callback = Some(Box::new(callback));

        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut self)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.spawn_app(self);
        }

        Ok(())
    }

    /// Runs the game without a callback.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use vyxen_core::{Scene, Game};
    ///
    /// let scene = Scene::new();
    ///
    /// let mut game = Game::new();
    ///
    /// game.load_scene(scene);
    ///
    /// game.run_without_callback();
    /// ```
    #[allow(unused_mut)]
    pub fn run_without_callback(mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut self)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.spawn_app(self);
        }

        Ok(())
    }

    /// If a key has been pressed between the current frame and the last.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::{KeyCode, Inputs};
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_just_pressed(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_just_pressed` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_just_pressed(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.just_pressed(keycode)
    }

    /// If a key has been released between the current frame and the last.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::KeyCode;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_just_released(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_just_released` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_just_released(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.just_released(keycode)
    }

    /// If a key is currently held.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_input::KeyCode;
    /// use vyxen_core::Game;
    ///
    /// let mut game = Game::new();
    ///
    /// assert!(!game.is_held(KeyCode::KeyH));
    /// ```
    ///
    /// # Note
    ///
    /// For `is_held` to be processed correctly, the game must be first ran from `run` or `run_without_callback`.
    pub fn is_held(&self, keycode: KeyCode) -> bool {
        self.ctx.inputs.held(keycode)
    }

    /// The current mouse position.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::Game;
    /// use vyxen_math::Vector2;
    ///
    /// let mut game = Game::new();
    ///
    /// assert_eq!(game.get_mouse_position(), Vector2::zero());
    /// ```
    ///
    /// # Note
    ///
    /// For `get_mouse_position` to be processed correctly,
    ///  - the game must be first ran from `run` or `run_without_callback`. If not, `Vector2::zero()` will be returned.
    ///  - the cursor position will only be updated when the cursor is in the window. If not, the most recent reported mouse position will be returned.
    pub fn get_mouse_position(&self) -> Vector2 {
        self.ctx.cursor_pos
    }
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.ctx.config.clone().into())
                .unwrap(),
        );

        let mut state = pollster::block_on(State::new(window, self.ctx.config.clone())).unwrap();

        state.resize(
            state.get_window().inner_size().width,
            state.get_window().inner_size().height,
        );

        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(mut callback) = self.callback.take() {
            let into: Event = match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    Event::MouseInput(button.into(), state.into(), self.ctx.cursor_pos)
                }
                _ => event.clone().into(),
            };
            if into != Event::Unknown {
                callback(self, event_loop, into);
            }

            self.callback = Some(callback);
        }

        match event {
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = &mut self.state {
                    state.resize(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                self.step(1.0 / 60.0);
                self.update_sprites();

                if let Some(state) = &mut self.state {
                    state.set_config(self.ctx.config.clone());
                    state.update();
                    state.render().unwrap();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => self.ctx.inputs.key_pressed(code.into()),
                        ElementState::Released => self.ctx.inputs.key_released(code.into()),
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.ctx.cursor_pos = Vector2 {
                    x: position.x as f32,
                    y: position.y as f32,
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.ctx.inputs.begin_frame();

        if let Some(state) = &self.state {
            state.get_window().request_redraw();
        }
    }
}

type ReadyOption = Option<Box<dyn FnMut(&mut Node, &mut Scene, Context)>>;
type PhysicsOption = Option<Box<dyn FnMut(&mut Node, &mut Scene, f32, Context)>>;
type CollisionOption = Option<Box<dyn FnMut(&mut Node, &mut Node, Manifold, &mut Scene, Context)>>;

/// Node struct for the scene
///
/// # Examples
/// ```rust
/// use vyxen_core::Node;
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

    on_ready: ReadyOption,
    physics_process: PhysicsOption,
    on_collision: CollisionOption,

    physics_process_default: bool,
    on_collision_default: bool,
}

impl Node {
    /// Gets the id of the node.
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::Circle;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
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
    /// use vyxen_core::Node;
    ///
    /// let node = Node::new("Foo".to_string());
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: Random::from_time().next_u64(),
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
    /// use vyxen_physics2d::RigidBody;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
    /// use vyxen_physics2d::RigidBody;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
    /// use vyxen_math::Vector2;
    /// use vyxen_core::Node;
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
                body_a.get_inverse_mass(),
                body_b.get_inverse_mass(),
                body_a.get_inverse_inertia(),
                body_b.get_inverse_inertia(),
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
                    s.get_inverse_mass() * c,
                    s.get_inverse_inertia(),
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
                    s.get_inverse_mass() * c,
                    s.get_inverse_inertia(),
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
                    r.get_inverse_mass(),
                    r.get_inverse_inertia(),
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
                    s.get_inverse_mass() * count,
                    s.get_inverse_inertia(),
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
        }
    }
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Component for RigidBody {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Component for SoftBody {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Allows nodes to collide with eachother.
///
/// # Examples
/// ```rust
/// use vyxen_core::{Node, Collider};
/// use vyxen_geometry::Circle;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Circle::new(5.0)));
    /// ```
    /// ## Boxes:
    /// ```rust
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Box;
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(Collider::new(Box::new(5.0, 5.0)));
    /// ```
    /// ## Polygons:
    /// ```rust
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Polygon;
    /// use vyxen_math::Vector2;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
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
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
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

    /// Gets the hitbox as a mutable reference
    ///
    /// # Examples
    /// ```rust
    /// use vyxen_core::{Node, Collider};
    /// use vyxen_geometry::Circle;
    /// use vyxen_math::Vector2;
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

    /// Sets if the aabb needs to be reinitialized
    pub fn set_uninitilized(&mut self) {
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

/// Window Events.
/// Retuned from a callback in `Game`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Event {
    /// Change of window size
    Resized(Vector2),
    /// Window movement
    Moved(Vector2),
    /// Close Request
    CloseRequested,
    /// Window closing
    Destroyed,
    /// A file dropped on the window
    DroppedFile(PathBuf),
    /// A file hovered on the window
    HoveredFile(PathBuf),
    /// A file was hovered, and moved from the window
    HoveredFileCancelled,
    /// Window gained focus
    Focused,
    /// Window lost focus
    Unfocused,
    /// Keyboard input
    KeyboardInput(KeyCode, KeyState),
    /// Cursor movement
    CursorMoved(Vector2),
    /// Cursor entered the window
    CursorEntered,
    /// Cursor exited the window
    CursorExited,
    /// Cursor mouse wheel movement
    MouseWheel(Vector2, TouchPhase),
    /// Cursor input
    MouseInput(MouseInput, KeyState, Vector2),
    /// Two-finger pinch gesture
    /// MacOS and iOS only.
    PinchGesture(f64),
    /// Pan gesture
    /// MacOS and iOS only.
    PanGesture(Vector2),
    /// Double tap gesture
    /// MacOS and iOS only.
    DoubleTapGesture,
    /// Two-finger rotation gesture
    /// MacOS and iOS only.
    RotationGesture(f32),
    /// Touchpad pressure, including stage.
    /// MacOS only.
    TouchpadPressure(f32, i64),
    /// Touch input
    Touch(Vector2, TouchPhase),
    /// Window moved across screens with different DPIs.
    ScaleChanged(f64),
    /// Window hidden behind another.
    Occluded,
    /// Window became visible
    Visible,
    /// Window should be redrawn
    RedrawRequested,
    /// Window should be redrawn
    Unknown,
}

impl From<WindowEvent> for Event {
    fn from(value: WindowEvent) -> Self {
        match value {
            WindowEvent::Resized(physical) => Self::Resized(Vector2 {
                x: physical.width as f32,
                y: physical.height as f32,
            }),
            WindowEvent::Moved(physical) => Self::Moved(Vector2 {
                x: physical.x as f32,
                y: physical.y as f32,
            }),
            WindowEvent::CloseRequested => Self::CloseRequested,
            WindowEvent::Destroyed => Self::Destroyed,
            WindowEvent::DroppedFile(path) => Self::DroppedFile(path),
            WindowEvent::HoveredFile(path) => Self::HoveredFile(path),
            WindowEvent::HoveredFileCancelled => Self::HoveredFileCancelled,
            WindowEvent::Focused(true) => Self::Focused,
            WindowEvent::Focused(false) => Self::Unfocused,
            WindowEvent::CursorEntered { .. } => Self::CursorEntered,
            WindowEvent::CursorLeft { .. } => Self::CursorExited,
            WindowEvent::PinchGesture { delta, .. } => Self::PinchGesture(delta),
            WindowEvent::PanGesture { delta, .. } => Self::PanGesture(Vector2 {
                x: delta.x,
                y: delta.y,
            }),
            WindowEvent::DoubleTapGesture { .. } => Self::DoubleTapGesture,
            WindowEvent::RotationGesture { delta, .. } => Self::RotationGesture(delta),
            WindowEvent::TouchpadPressure {
                pressure, stage, ..
            } => Self::TouchpadPressure(pressure, stage),
            WindowEvent::Touch(touch) => Self::Touch(
                Vector2 {
                    x: touch.location.x as f32,
                    y: touch.location.y as f32,
                },
                touch.phase.into(),
            ),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                Self::ScaleChanged(scale_factor)
            }
            WindowEvent::Occluded(true) => Self::Occluded,
            WindowEvent::Occluded(false) => Self::Visible,
            WindowEvent::RedrawRequested => Self::RedrawRequested,
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => Self::KeyboardInput(code.into(), event.state.into()),
                PhysicalKey::Unidentified(_) => Self::Unknown,
            },
            WindowEvent::CursorMoved { position, .. } => Self::CursorMoved(Vector2 {
                x: position.x as f32,
                y: position.y as f32,
            }),
            WindowEvent::MouseWheel { delta, phase, .. } => Self::MouseWheel(
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vector2 { x, y },
                    MouseScrollDelta::PixelDelta(pos) => Vector2 {
                        x: pos.x as f32,
                        y: pos.y as f32,
                    },
                },
                phase.into(),
            ),
            _ => Self::Unknown,
        }
    }
}

/// The context of the game.
#[derive(Clone)]
pub struct Context {
    pub inputs: Inputs,
    pub cursor_pos: Vector2,
    pub config: WindowConfig,
}

impl Context {
    /// If a key is currently held.
    pub fn is_held(&self, keycode: KeyCode) -> bool {
        self.inputs.held(keycode)
    }

    /// If a key has been pressed between the current frame and the last.
    pub fn is_just_pressed(&self, keycode: KeyCode) -> bool {
        self.inputs.just_pressed(keycode)
    }

    /// If a key has been released between the current frame and the last.
    pub fn is_just_released(&self, keycode: KeyCode) -> bool {
        self.inputs.just_released(keycode)
    }
}
