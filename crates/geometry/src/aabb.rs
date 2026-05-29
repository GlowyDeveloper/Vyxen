use vyxen_math::Vector2;

/// Axis-Aligned Bounding Box (AABB)
///
/// # Examples
///
/// ```rust
/// use vyxen_math::Vector2;
/// use vyxen_geometry::aabb::AABB;
///
/// let a = AABB::new(
///     Vector2 { x: 0.0, y: 0.0 },
///     Vector2 { x: 1.0, y: 1.0 },
/// );
/// assert_eq!(a.get_min(), Vector2 { x: 0.0, y: 0.0 });
/// assert_eq!(a.get_max(), Vector2 { x: 1.0, y: 1.0 });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    min: Vector2,
    max: Vector2,
}

impl AABB {
    /// Create a new `AABB` from the given `min` and `max` corners.
    pub fn new(min: Vector2, max: Vector2) -> Self {
        AABB { min, max }
    }

    /// Create a new `AABB` from raw coordinate components.
    pub fn new_from_uncalculated(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        AABB {
            min: Vector2 { x: min_x, y: min_y },
            max: Vector2 { x: max_x, y: max_y },
        }
    }

    /// Return the minimum corner of the bounding box.
    pub fn get_min(&self) -> Vector2 {
        self.min
    }

    /// Return the maximum corner of the bounding box.
    pub fn get_max(&self) -> Vector2 {
        self.max
    }

    /// Checks **if** two `AABB`s intersect eachother
    /// 
    /// # Examples
    /// ```rust
    /// use vyxen_math::Vector2;
    /// use vyxen_geometry::aabb::AABB;
    ///
    /// let a = AABB::new(
    ///     Vector2 { x: 0.0, y: 0.0 },
    ///     Vector2 { x: 1.0, y: 1.0 },
    /// );
    /// 
    /// let b = AABB::new(
    ///     Vector2 { x: 0.5, y: 0.5 },
    ///     Vector2 { x: 1.5, y: 1.5 },
    /// );
    /// 
    /// assert!(AABB::intersect_aabb(a, b));
    /// ```
    pub fn intersect_aabb(aabb_a: AABB, aabb_b: AABB) -> bool {
        if aabb_a.get_max().x <= aabb_b.get_min().x ||
            aabb_b.get_max().x <= aabb_a.get_min().x ||
            aabb_a.get_max().y <= aabb_b.get_min().y ||
            aabb_b.get_max().y <= aabb_a.get_min().y {
            return false;
        }
        return true;
    }
}