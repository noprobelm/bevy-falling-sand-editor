use std::fmt;

use avian2d::{debug_render::ContactGizmoScale, prelude::PhysicsGizmos};
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use serde::{Deserialize, Serialize};

use crate::{
    brush::{BrushKeyBindings, BrushModeState, BrushSize, BrushTypeState},
    camera::CameraKeyBindings,
    ui::UiKeyBindings,
};

/// A unified input type that can represent either a keyboard key or a mouse button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputButton {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl InputButton {
    /// Insert this input button into an `InputMap` for the given action.
    pub fn insert_into_input_map<A: leafwing_input_manager::Actionlike>(
        self,
        input_map: &mut InputMap<A>,
        action: A,
    ) {
        match self {
            InputButton::Key(key) => {
                input_map.insert(action, key);
            }
            InputButton::Mouse(button) => {
                input_map.insert(action, button);
            }
        }
    }
}

impl fmt::Display for InputButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputButton::Key(key) => write!(f, "{key:?}"),
            InputButton::Mouse(button) => write!(f, "{button:?}"),
        }
    }
}

impl From<KeyCode> for InputButton {
    fn from(key: KeyCode) -> Self {
        InputButton::Key(key)
    }
}

impl From<MouseButton> for InputButton {
    fn from(button: MouseButton) -> Self {
        InputButton::Mouse(button)
    }
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub brush: BrushConfig,
    pub bfs_debug: BevyFallingSandDebugConfig,
    pub avian_debug: AvianDebugConfig,
    pub keys: Keybindings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrushConfig {
    pub btype: BrushTypeState,
    pub mode: BrushModeState,
    pub size: BrushSize,
}

impl Default for BrushConfig {
    fn default() -> Self {
        Self {
            btype: BrushTypeState::default(),
            mode: BrushModeState::default(),
            size: BrushSize(2),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BevyFallingSandDebugConfig {
    pub map: OptionalColor,
    pub dirty_rects: OptionalColor,
}

impl Default for BevyFallingSandDebugConfig {
    fn default() -> Self {
        Self {
            map: OptionalColor {
                enabled: false,
                color: [0.67, 0.21, 0.24, 1.0],
            },
            dirty_rects: OptionalColor {
                enabled: false,
                color: [1., 1., 1., 1.],
            },
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Keybindings {
    pub camera: CameraKeyBindings,
    pub ui: UiKeyBindings,
    pub brush: BrushKeyBindings,
}

/// TOML-friendly mirror of [`PhysicsGizmos`].
///
/// Each `Option<T>` field from `PhysicsGizmos` is split into an `enabled` bool
/// and a value, so that `None` can be persisted as `enabled = false` in TOML.
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct AvianDebugConfig {
    pub axis_lengths: OptionalVec2,
    pub aabb_color: OptionalColor,
    pub collider_color: OptionalColor,
    pub sleeping_color_multiplier: OptionalColorMultiplier,
    pub contact_point_color: OptionalColor,
    pub contact_normal_color: OptionalColor,
    pub contact_normal_scale: ContactNormalScale,
    pub joint_anchor_color: OptionalColor,
    pub joint_separation_color: OptionalColor,
    pub raycast_color: OptionalColor,
    pub raycast_point_color: OptionalColor,
    pub raycast_normal_color: OptionalColor,
    pub shapecast_color: OptionalColor,
    pub shapecast_shape_color: OptionalColor,
    pub shapecast_point_color: OptionalColor,
    pub shapecast_normal_color: OptionalColor,
    pub island_color: OptionalColor,
    pub hide_meshes: bool,
}

impl AvianDebugConfig {
    pub const fn with_axis_lengths(self, axis_lengths: OptionalVec2) -> Self {
        Self {
            axis_lengths,
            ..self
        }
    }

    pub const fn with_aabb_color(self, aabb_color: OptionalColor) -> Self {
        Self { aabb_color, ..self }
    }

    pub const fn with_collider_color(self, collider_color: OptionalColor) -> Self {
        Self {
            collider_color,
            ..self
        }
    }

    pub const fn with_sleeping_color_multiplier(
        self,
        sleeping_color_multiplier: OptionalColorMultiplier,
    ) -> Self {
        Self {
            sleeping_color_multiplier,
            ..self
        }
    }

    pub const fn with_contact_point_color(self, contact_point_color: OptionalColor) -> Self {
        Self {
            contact_point_color,
            ..self
        }
    }

    pub const fn with_contact_normal_color(self, contact_normal_color: OptionalColor) -> Self {
        Self {
            contact_normal_color,
            ..self
        }
    }

    pub const fn with_contact_normal_scale(self, contact_normal_scale: ContactNormalScale) -> Self {
        Self {
            contact_normal_scale,
            ..self
        }
    }

    pub const fn with_joint_anchor_color(self, joint_anchor_color: OptionalColor) -> Self {
        Self {
            joint_anchor_color,
            ..self
        }
    }

    pub const fn with_joint_separation_color(self, joint_separation_color: OptionalColor) -> Self {
        Self {
            joint_separation_color,
            ..self
        }
    }

    pub const fn with_raycast_color(self, raycast_color: OptionalColor) -> Self {
        Self {
            raycast_color,
            ..self
        }
    }

    pub const fn with_raycast_point_color(self, raycast_point_color: OptionalColor) -> Self {
        Self {
            raycast_point_color,
            ..self
        }
    }

    pub const fn with_raycast_normal_color(self, raycast_normal_color: OptionalColor) -> Self {
        Self {
            raycast_normal_color,
            ..self
        }
    }

    pub const fn with_shapecast_color(self, shapecast_color: OptionalColor) -> Self {
        Self {
            shapecast_color,
            ..self
        }
    }

    pub const fn with_shapecast_shape_color(self, shapecast_shape_color: OptionalColor) -> Self {
        Self {
            shapecast_shape_color,
            ..self
        }
    }

    pub const fn with_shapecast_point_color(self, shapecast_point_color: OptionalColor) -> Self {
        Self {
            shapecast_point_color,
            ..self
        }
    }

    pub const fn with_shapecast_normal_color(self, shapecast_normal_color: OptionalColor) -> Self {
        Self {
            shapecast_normal_color,
            ..self
        }
    }

    pub const fn with_island_color(self, island_color: OptionalColor) -> Self {
        Self {
            island_color,
            ..self
        }
    }

    pub const fn with_hide_meshes(self, hide_meshes: bool) -> Self {
        Self {
            hide_meshes,
            ..self
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalColor {
    pub enabled: bool,
    #[serde(default = "default_color")]
    pub color: [f32; 4],
}

fn default_color() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

impl OptionalColor {
    fn some(color: Color) -> Self {
        let c = color.to_srgba();
        Self {
            enabled: true,
            color: [c.red, c.green, c.blue, c.alpha],
        }
    }

    fn none() -> Self {
        Self {
            enabled: false,
            color: default_color(),
        }
    }

    fn from_option(opt: Option<Color>) -> Self {
        match opt {
            Some(c) => Self::some(c),
            None => Self::none(),
        }
    }

    fn to_option(&self) -> Option<Color> {
        if self.enabled {
            Some(Color::srgba(
                self.color[0],
                self.color[1],
                self.color[2],
                self.color[3],
            ))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalVec2 {
    pub enabled: bool,
    #[serde(default = "default_vec2")]
    pub value: [f32; 2],
}

fn default_vec2() -> [f32; 2] {
    [0.5, 0.5]
}

impl OptionalVec2 {
    fn from_option(opt: Option<Vec2>) -> Self {
        match opt {
            Some(v) => Self {
                enabled: true,
                value: [v.x, v.y],
            },
            None => Self {
                enabled: false,
                value: default_vec2(),
            },
        }
    }

    fn to_option(&self) -> Option<Vec2> {
        if self.enabled {
            Some(Vec2::new(self.value[0], self.value[1]))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionalColorMultiplier {
    pub enabled: bool,
    #[serde(default = "default_color_multiplier")]
    pub value: [f32; 4],
}

fn default_color_multiplier() -> [f32; 4] {
    [1.0, 1.0, 0.4, 1.0]
}

impl OptionalColorMultiplier {
    fn from_option(opt: Option<[f32; 4]>) -> Self {
        match opt {
            Some(v) => Self {
                enabled: true,
                value: v,
            },
            None => Self {
                enabled: false,
                value: default_color_multiplier(),
            },
        }
    }

    fn to_option(&self) -> Option<[f32; 4]> {
        if self.enabled { Some(self.value) } else { None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContactNormalScale {
    Constant(f32),
    Scaled(f32),
}

impl Default for ContactNormalScale {
    fn default() -> Self {
        Self::Scaled(0.025)
    }
}

impl From<ContactGizmoScale> for ContactNormalScale {
    fn from(scale: ContactGizmoScale) -> Self {
        match scale {
            ContactGizmoScale::Constant(v) => Self::Constant(v),
            ContactGizmoScale::Scaled(v) => Self::Scaled(v),
        }
    }
}

impl From<ContactNormalScale> for ContactGizmoScale {
    fn from(scale: ContactNormalScale) -> Self {
        match scale {
            ContactNormalScale::Constant(v) => Self::Constant(v),
            ContactNormalScale::Scaled(v) => Self::Scaled(v),
        }
    }
}

impl Default for AvianDebugConfig {
    fn default() -> Self {
        let mut config = Self::from(PhysicsGizmos::default());
        config.axis_lengths.enabled = false;
        config.aabb_color.enabled = false;
        config.collider_color.enabled = false;
        config.sleeping_color_multiplier.enabled = false;
        config.contact_point_color.enabled = false;
        config.contact_normal_color.enabled = false;
        config.joint_anchor_color.enabled = false;
        config.joint_separation_color.enabled = false;
        config.raycast_color.enabled = false;
        config.raycast_point_color.enabled = false;
        config.raycast_normal_color.enabled = false;
        config.shapecast_color.enabled = false;
        config.shapecast_shape_color.enabled = false;
        config.shapecast_point_color.enabled = false;
        config.shapecast_normal_color.enabled = false;
        config.island_color.enabled = false;
        config.hide_meshes = false;
        config
    }
}

impl From<PhysicsGizmos> for AvianDebugConfig {
    fn from(g: PhysicsGizmos) -> Self {
        Self {
            axis_lengths: OptionalVec2::from_option(g.axis_lengths),
            aabb_color: OptionalColor::from_option(g.aabb_color),
            collider_color: OptionalColor::from_option(g.collider_color),
            sleeping_color_multiplier: OptionalColorMultiplier::from_option(
                g.sleeping_color_multiplier,
            ),
            contact_point_color: OptionalColor::from_option(g.contact_point_color),
            contact_normal_color: OptionalColor::from_option(g.contact_normal_color),
            contact_normal_scale: g.contact_normal_scale.into(),
            joint_anchor_color: OptionalColor::from_option(g.joint_anchor_color),
            joint_separation_color: OptionalColor::from_option(g.joint_separation_color),
            raycast_color: OptionalColor::from_option(g.raycast_color),
            raycast_point_color: OptionalColor::from_option(g.raycast_point_color),
            raycast_normal_color: OptionalColor::from_option(g.raycast_normal_color),
            shapecast_color: OptionalColor::from_option(g.shapecast_color),
            shapecast_shape_color: OptionalColor::from_option(g.shapecast_shape_color),
            shapecast_point_color: OptionalColor::from_option(g.shapecast_point_color),
            shapecast_normal_color: OptionalColor::from_option(g.shapecast_normal_color),
            island_color: OptionalColor::from_option(g.island_color),
            hide_meshes: g.hide_meshes,
        }
    }
}

impl From<AvianDebugConfig> for PhysicsGizmos {
    fn from(c: AvianDebugConfig) -> Self {
        Self {
            axis_lengths: c.axis_lengths.to_option(),
            aabb_color: c.aabb_color.to_option(),
            collider_color: c.collider_color.to_option(),
            sleeping_color_multiplier: c.sleeping_color_multiplier.to_option(),
            contact_point_color: c.contact_point_color.to_option(),
            contact_normal_color: c.contact_normal_color.to_option(),
            contact_normal_scale: c.contact_normal_scale.into(),
            joint_anchor_color: c.joint_anchor_color.to_option(),
            joint_separation_color: c.joint_separation_color.to_option(),
            raycast_color: c.raycast_color.to_option(),
            raycast_point_color: c.raycast_point_color.to_option(),
            raycast_normal_color: c.raycast_normal_color.to_option(),
            shapecast_color: c.shapecast_color.to_option(),
            shapecast_shape_color: c.shapecast_shape_color.to_option(),
            shapecast_point_color: c.shapecast_point_color.to_option(),
            shapecast_normal_color: c.shapecast_normal_color.to_option(),
            island_color: c.island_color.to_option(),
            hide_meshes: c.hide_meshes,
        }
    }
}
