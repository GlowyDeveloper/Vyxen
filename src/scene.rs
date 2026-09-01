use crate::{
    AABB, Collider, Context, Node, Vector2,
    error::Error,
    physics2d::{Collision, ContactPoints, Manifold},
};
use std::collections::HashMap;

/// Scene to hold nodes in the game
///
/// # Examples
/// ```rust
/// use vyxen::{Scene, Node, Game, Circle, Vector2, physics2d::RigidBody};
///
/// let mut scene = Scene::new();
///
/// let mut node = Node::new("Foo".to_string());
/// let id = node.get_id();
/// node.add_component(RigidBody::new(1.0, 0.5, Circle::new(1.0), 0.6, 0.4));
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
    /// use vyxen::{Scene, Node, Circle, Vector2, physics2d::RigidBody};
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// let id = node.get_id();
    /// node.add_component(RigidBody::new(1.0, 0.5, Circle::new(1.0), 0.6, 0.4));
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
    /// use vyxen::Scene;
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
    /// use vyxen::Scene;
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
    /// use vyxen::Scene;
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
    /// use vyxen::Scene;
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
    /// use vyxen::{Scene, Node};
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
    /// use vyxen::{Scene, Node};
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
    /// use vyxen::{Scene, Node, Vector2, Circle};
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
    pub fn remove_node(&mut self, node: &Node) -> Result<(), Error> {
        self.remove_node_by_id(node.get_id())
    }

    /// Removes the node from the scene by id with all of its children
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen::{Scene, Node, Vector2, Circle};
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
    ///
    /// # Errors
    ///
    /// Errors if you attempt to remove the root node (0).
    pub fn remove_node_by_id(&mut self, id: u64) -> Result<(), Error> {
        if id == 0 {
            return Err(Error::RootNodeRemoval);
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
    /// use vyxen::{Scene, Node, Vector2, Circle};
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
    /// use vyxen::{Scene, Vector2};
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
    /// use vyxen::{Scene, Vector2};
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
    /// use vyxen::Scene;
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
    /// use vyxen::Scene;
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
    /// use vyxen::{
    ///     Scene, Node, Context, Vector2, Circle,
    ///     WindowConfig, physics2d::RigidBody, inputs::Inputs
    /// };
    ///
    /// let mut scene = Scene::new();
    ///
    /// let mut node = Node::new("Foo".to_string());
    /// node.add_component(RigidBody::new(1.0, 0.5, Circle::new(1.0), 0.6, 0.4));
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

                self.nodes.insert(id_a, node_a);
                self.nodes.insert(id_b, node_b);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Game;

    #[test]
    fn test_scene_initialization() {
        let scene = Scene::new();
        assert_eq!(scene.get_nodes_len(), 1);
        assert_eq!(scene.get_root().get_name(), "Root");
        assert_eq!(scene.get_gravity(), Vector2 { x: 0.0, y: -9.81 });
    }

    #[test]
    fn test_scene_node_management() {
        let mut scene = Scene::new();
        let node = Node::new("Player".to_string());
        let id = node.get_id();

        scene.add_node(node);
        assert_eq!(scene.get_nodes_len(), 2);
        assert!(scene.get_node(id).is_some());

        scene.remove_node_by_id(id).unwrap();
        assert_eq!(scene.get_nodes_len(), 1);
        assert!(scene.get_node(id).is_none());
    }

    #[test]
    fn test_game_scene_management() {
        let mut game = Game::new();
        assert!(game.get_scene().is_none());

        let scene = Scene::new();
        game.load_scene(scene);

        assert!(game.get_scene().is_some());
    }
}
