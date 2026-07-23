#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::HashSet;

pub use keycode::KeyCode;
use winit::event::{ElementState, MouseButton, TouchPhase as WinitTouchPhase};

mod keycode;

/// Inputs handler for `game`.
///
/// # Examples
///
/// ```rust
/// use vyxen_input::{Inputs, KeyCode};
///
/// let mut inputs = Inputs::new();
///
/// inputs.key_pressed(KeyCode::KeyH);
///
/// assert!(inputs.just_pressed(KeyCode::KeyH));
/// assert!(inputs.held(KeyCode::KeyH));
///
/// inputs.key_released(KeyCode::KeyH);
///
/// assert!(inputs.just_released(KeyCode::KeyH));
/// assert!(!inputs.held(KeyCode::KeyH));
/// ```
#[derive(Clone)]
pub struct Inputs {
    held: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs::new()
    }
}

impl Inputs {
    /// Creates a new input handler.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_pressed(KeyCode::KeyH));
    /// assert!(inputs.held(KeyCode::KeyH));
    ///
    /// inputs.key_released(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_released(KeyCode::KeyH));
    /// assert!(!inputs.held(KeyCode::KeyH));
    /// ```
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    /// Clears the keys that were just pressed/released.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_pressed(KeyCode::KeyH));
    ///
    /// inputs.begin_frame();
    ///
    /// assert!(!inputs.just_pressed(KeyCode::KeyH));
    /// ```
    pub fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Denotes a key as pressed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_pressed(KeyCode::KeyH));
    /// assert!(inputs.held(KeyCode::KeyH));
    /// ```
    pub fn key_pressed(&mut self, key: KeyCode) {
        if self.held.insert(key) {
            self.just_pressed.insert(key);
        }
    }

    /// Denotes a key as released.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_pressed(KeyCode::KeyH));
    /// assert!(inputs.held(KeyCode::KeyH));
    ///
    /// inputs.key_released(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_released(KeyCode::KeyH));
    /// assert!(!inputs.held(KeyCode::KeyH));
    /// ```
    pub fn key_released(&mut self, key: KeyCode) {
        self.held.remove(&key);
        self.just_released.insert(key);
    }

    /// If a key is held
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.held(KeyCode::KeyH));
    /// ```
    pub fn held(&self, key: KeyCode) -> bool {
        self.held.contains(&key)
    }

    /// If a key is just pressed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_pressed(KeyCode::KeyH));
    /// ```
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    /// If a key is just released
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vyxen_input::{Inputs, KeyCode};
    ///
    /// let mut inputs = Inputs::new();
    ///
    /// inputs.key_pressed(KeyCode::KeyH);
    ///
    /// assert!(!inputs.just_released(KeyCode::KeyH));
    ///
    /// inputs.key_released(KeyCode::KeyH);
    ///
    /// assert!(inputs.just_released(KeyCode::KeyH));
    /// ```
    pub fn just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}

/// If a key is pressed or released.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyState {
    Pressed,
    Released,
}

impl From<ElementState> for KeyState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => KeyState::Pressed,
            ElementState::Released => KeyState::Released,
        }
    }
}

/// The current state of a touch input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl From<WinitTouchPhase> for TouchPhase {
    fn from(value: WinitTouchPhase) -> Self {
        match value {
            WinitTouchPhase::Started => Self::Started,
            WinitTouchPhase::Moved => Self::Moved,
            WinitTouchPhase::Ended => Self::Ended,
            WinitTouchPhase::Cancelled => Self::Cancelled,
        }
    }
}

/// All mouse inputs that can be captured
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseInput {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Unknown(u16),
}

impl From<MouseButton> for MouseInput {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
            MouseButton::Back => Self::Back,
            MouseButton::Forward => Self::Forward,
            MouseButton::Other(id) => Self::Unknown(id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs() {
        let mut inputs = Inputs::new();
        assert!(inputs.just_pressed.is_empty());
        assert!(inputs.just_released.is_empty());
        assert!(inputs.held.is_empty());

        inputs.key_pressed(KeyCode::KeyA);
        inputs.key_pressed(KeyCode::KeyB);

        assert!(inputs.just_pressed.contains(&KeyCode::KeyA));
        assert!(inputs.just_pressed.contains(&KeyCode::KeyB));
        assert!(inputs.held.contains(&KeyCode::KeyA));
        assert!(inputs.held.contains(&KeyCode::KeyB));

        inputs.key_released(KeyCode::KeyB);

        assert!(inputs.just_released.contains(&KeyCode::KeyB));
        assert!(inputs.held.contains(&KeyCode::KeyA));
        assert!(!inputs.held.contains(&KeyCode::KeyB));

        inputs.begin_frame();

        assert!(inputs.just_pressed.is_empty());
        assert!(inputs.just_released.is_empty());
        assert!(inputs.held.contains(&KeyCode::KeyA));
        assert!(!inputs.held.contains(&KeyCode::KeyB));
    }

    #[test]
    fn test_key_state_convertion() {
        let element_state_1 = ElementState::Pressed;
        let element_state_2 = ElementState::Released;

        assert_eq!(KeyState::Pressed, element_state_1.into());
        assert_eq!(KeyState::Released, element_state_2.into());
    }

    #[test]
    fn test_touch_phase_convertion() {
        let touch_phase_1 = WinitTouchPhase::Started;
        let touch_phase_2 = WinitTouchPhase::Ended;
        let touch_phase_3 = WinitTouchPhase::Cancelled;
        let touch_phase_4 = WinitTouchPhase::Moved;

        assert_eq!(TouchPhase::Started, touch_phase_1.into());
        assert_eq!(TouchPhase::Ended, touch_phase_2.into());
        assert_eq!(TouchPhase::Cancelled, touch_phase_3.into());
        assert_eq!(TouchPhase::Moved, touch_phase_4.into());
    }

    #[test]
    fn test_mouse_input_convertion() {
        let mouse_input_1 = MouseButton::Left;
        let mouse_input_2 = MouseButton::Right;
        let mouse_input_3 = MouseButton::Middle;
        let mouse_input_4 = MouseButton::Back;
        let mouse_input_5 = MouseButton::Forward;
        let mouse_input_6 = MouseButton::Other(5);

        assert_eq!(MouseInput::Left, mouse_input_1.into());
        assert_eq!(MouseInput::Right, mouse_input_2.into());
        assert_eq!(MouseInput::Middle, mouse_input_3.into());
        assert_eq!(MouseInput::Back, mouse_input_4.into());
        assert_eq!(MouseInput::Forward, mouse_input_5.into());
        assert_eq!(MouseInput::Unknown(5), mouse_input_6.into());
    }
}
