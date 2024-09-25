use crate::Container;
use noisy_float::types::R64;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, fmt::Display};

const DEFAULT_VERSION: u64 = 77310525440;

pub type Prototype = String;
pub type EntityNumber = OneBasedIndex;
pub type ItemStackIndex = u16;
pub type ItemCountType = u32;
pub type GraphicsVariation = u8;
pub type OneBasedIndex = std::num::NonZeroUsize;

/// https://wiki.factorio.com/Blueprint_string_format#Blueprint_book_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BlueprintBook {
    pub item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blueprints: Vec<BlueprintBookBlueprintValue>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<Icon>,
    pub active_index: usize,
    pub version: u64,
}

impl Default for BlueprintBook {
    fn default() -> BlueprintBook {
        BlueprintBook {
            item: "blueprint-book".into(),
            version: DEFAULT_VERSION,
            label: Default::default(),
            label_color: Default::default(),
            description: Default::default(),
            blueprints: Default::default(),
            icons: Default::default(),
            active_index: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BlueprintBookBlueprintValue {
    pub index: usize,
    #[serde(flatten)]
    pub item: Container,
}

/// https://wiki.factorio.com/Blueprint_string_format#Blueprint_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Blueprint {
    pub item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "label_color")]
    pub label_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<Tile>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<Icon>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_relative_to_grid: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_grid: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_snapping: Option<bool>,
    pub version: u64,
}

impl Default for Blueprint {
    fn default() -> Blueprint {
        Blueprint {
            item: "blueprint".into(),
            version: DEFAULT_VERSION,
            label: Default::default(),
            label_color: Default::default(),
            description: Default::default(),
            entities: Default::default(),
            tiles: Default::default(),
            icons: Default::default(),
            schedules: Default::default(),
            position_relative_to_grid: Default::default(),
            snap_to_grid: Default::default(),
            absolute_snapping: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct DeconstructionPlanner {
    pub item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub settings: Option<DeconstructionSettings>,
    pub version: u64,
}

impl Default for DeconstructionPlanner {
    fn default() -> DeconstructionPlanner {
        DeconstructionPlanner {
            item: "deconstruction_planner".into(),
            label: None,
            settings: None,
            version: DEFAULT_VERSION,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct DeconstructionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<Icon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filters: Option<Vec<DeconstructionFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filter_mode: Option<DeconstructionEntityFilterMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trees_and_rocks_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_filters: Option<Vec<DeconstructionFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_selection_mode: Option<TileSelectionMode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct DeconstructionFilter {
    index: u32,
    name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct UpgradePlanner {
    pub item: String,
    pub settings: Option<UpgradePlannerSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub version: u64,
}

impl Default for UpgradePlanner {
    fn default() -> UpgradePlanner {
        UpgradePlanner {
            item: "upgrade_planner".into(),
            settings: None,
            label: Default::default(),
            version: DEFAULT_VERSION,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct UpgradePlannerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappers: Option<Vec<Mapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<Icon>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Mapper {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<SimpleEntity>,
    pub index: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SimpleEntity {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum DeconstructionEntityFilterMode {
    // Note: Factorio produces and requires ints
    Whitelist = 0,
    Blacklist = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum TileSelectionMode {
    // Note: Factorio produces and requires ints
    Normal = 0,
    Always = 1,
    Never = 2,
    Only = 3,
}

/// https://wiki.factorio.com/Blueprint_string_format#Icon_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Icon {
    pub index: OneBasedIndex,
    pub signal: SignalID,
}

/// https://wiki.factorio.com/Blueprint_string_format#SignalID_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SignalID {
    pub name: Prototype,
    #[serde(rename = "type")]
    pub type_: SignalIDType,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalIDType {
    Item,
    Fluid,
    Virtual,
}

/// https://wiki.factorio.com/Blueprint_string_format#Entity_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Entity {
    pub entity_number: EntityNumber,
    pub name: Prototype,
    pub position: Position,
    #[serde(default = "Direction::default")]
    pub direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<R64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<EntityConnections>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_behavior: Option<ControlBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<Prototype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<ItemStackIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Inventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinity_settings: Option<InfinitySettings>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<EntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_priority: Option<EntityPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_priority: Option<EntityPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Prototype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ItemFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<EntityFilterMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_stack_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_filters: Option<Vec<LogisticFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_from_buffers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<SpeakerParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_parameters: Option<SpeakerAlertParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_launch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation: Option<GraphicsVariation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_trains_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbours: Option<Vec<EntityNumber>>,
}

/// Direction of an entity
#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr, Default)]
#[repr(u8)]
pub enum Direction {
    #[default]
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

/// Reverse-engineered by hand, contains circuit network metadata
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ControlBehavior {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_to_logistic_network: Option<bool>,
    /// Used in arithmetic combinators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arithmetic_conditions: Option<ArithmeticConditions>,
    /// Used in decider combinators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decider_conditions: Option<DeciderConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logistic_condition: Option<LogisticCondition>,
    /// Used in constant combinators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ControlFilter>>,
    /// Used in constant combinators, optional. Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_condition: Option<CircuitCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_mode_of_operation: Option<CircuitModeOfOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_enable_disable: Option<bool>,
    /// Read mode for belts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_contents_read_mode: Option<ContentReadMode>,
    /// Read mode for inserters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_hand_read_mode: Option<ContentReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_read_hand_contents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_set_stack_size: Option<bool>,
    /// Used for inserters with the set stack size option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_control_input_signal: Option<SimpleEntity>,
    /// Used for Speakers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_parameters: Option<SpeakerCircuitParameters>,

    /// Used for accumulators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_signal: Option<SimpleEntity>,

    // Train stops
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_from_train: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_stopped_train: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_trains_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_trains_limit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to_train: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_stopped_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trains_count_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trains_limit_signal: Option<SimpleEntity>,

    // Roboports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_logistics: Option<bool>,
    /// If this roboport is set to read robot statistics
    /// Note that if the output signals are None while this is set to Some(true)
    /// the game will use the default signals of X, Y, Z, T
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_robot_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_construction_output_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_logistic_output_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_construction_output_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_logistic_output_signal: Option<SimpleEntity>,

    // Gates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_open_gate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_read_sensor: Option<bool>,

    // Rail signals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_close_signal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_read_signal: Option<bool>,
}

/// Reverse-engineered by hand, contains arithmetic combinator metadata
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ArithmeticConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_constant: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_constant: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signal: Option<SignalID>,
    pub operation: ArithmeticOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_signal: Option<SignalID>,
}

/// Possible operation performed by an arithmetic combinator
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum ArithmeticOperation {
    /// Addition (+)
    #[serde(rename = "+")]
    Add,
    /// Subtraction (−)
    #[serde(rename = "-")]
    Subtract,
    /// Multiplication (*)
    #[serde(rename = "*")]
    Multiply,
    /// Division (/)
    #[serde(rename = "/")]
    Divide,
    /// Modulo (%)
    #[serde(rename = "%")]
    Modulo,
    /// Exponentiation (^)
    #[serde(rename = "^")]
    Exponentiate,
    /// Left bit shift (<<)
    #[serde(rename = "<<")]
    LeftShift,
    /// Right bit shift (>>)
    #[serde(rename = ">>")]
    RightShift,
    /// Bitwise AND (&)
    #[serde(rename = "AND")]
    And,
    /// Bitwise OR (|)
    #[serde(rename = "OR")]
    Or,
    /// Bitwise XOR (^)
    #[serde(rename = "XOR")]
    Xor,
}

impl Display for ArithmeticOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOperation::Add => write!(f, "+"),
            ArithmeticOperation::Subtract => write!(f, "-"),
            ArithmeticOperation::Multiply => write!(f, "*"),
            ArithmeticOperation::Divide => write!(f, "/"),
            ArithmeticOperation::Modulo => write!(f, "%"),
            ArithmeticOperation::Exponentiate => write!(f, "^"),
            ArithmeticOperation::LeftShift => write!(f, "<<"),
            ArithmeticOperation::RightShift => write!(f, ">>"),
            ArithmeticOperation::And => write!(f, "&"),
            ArithmeticOperation::Or => write!(f, "|"),
            ArithmeticOperation::Xor => write!(f, "^"),
        }
    }
}

/// Reverse-engineered by hand, contains constant combinator metadata
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct DeciderConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<i32>,
    pub comparator: DeciderComparator,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_count_from_input: Option<bool>,
}

/// Possible comparisons performed by decider combinator
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum DeciderComparator {
    /// "is greater than" (>)
    #[serde(rename = ">")]
    GreaterThan,
    /// "is less than" (<)
    #[serde(rename = "<")]
    LessThan,
    /// "greater than or equal to" (>=)
    #[serde(rename = "≥")]
    GreaterThanOrEqual,
    /// "less than or equal to" (<=)
    #[serde(rename = "≤")]
    LessThanOrEqual,
    /// "is equal to" (=)
    #[serde(rename = "=")]
    Equal,
    /// "is not equal to" (!=)
    #[serde(rename = "≠")]
    NotEqual,
}

impl Display for DeciderComparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeciderComparator::GreaterThan => write!(f, ">"),
            DeciderComparator::LessThan => write!(f, "<"),
            DeciderComparator::GreaterThanOrEqual => write!(f, ">="),
            DeciderComparator::LessThanOrEqual => write!(f, "<="),
            DeciderComparator::Equal => write!(f, "="),
            DeciderComparator::NotEqual => write!(f, "!="),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct LogisticCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<i32>,
    pub comparator: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum CircuitModeOfOperation {
    /// TODO what does this correspond to? power_book.txt has this set to 0
    /// https://github.com/coriolinus/factorio-blueprint/issues/7
    RoboportThing = 0,
    SetRequests = 1,
    // TODO what does this correspond to?
    // https://github.com/coriolinus/factorio-blueprint/issues/7
    // TWO = 2,
    None = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum ContentReadMode {
    Pulse = 0,
    Hold = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SpeakerCircuitParameters {
    pub instrument_id: i32,
    pub note_id: i32,
    pub signal_value_is_pitch: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntityConnections {
    StringIdx(HashMap<String, Connection>),
    NumberIdx(HashMap<OneBasedIndex, Connection>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Input,
    Output,
    Item,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityPriority {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityFilterMode {
    Whitelist,
    Blacklist,
}

/// https://wiki.factorio.com/Blueprint_string_format#Inventory_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Inventory {
    pub filters: Vec<ItemFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<ItemStackIndex>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Schedule_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Schedule {
    pub schedule: Vec<ScheduleRecord>,
    pub locomotives: Vec<EntityNumber>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Schedule_Record_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ScheduleRecord {
    pub station: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_conditions: Option<Vec<WaitCondition>>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Wait_Condition_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct WaitCondition {
    #[serde(rename = "type")]
    pub type_: WaitConditionType,
    pub compare_type: CompareType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<CircuitCondition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitConditionType {
    Time,
    Inactivity,
    Full,
    Empty,
    ItemCount,
    Circuit,
    RobotsInactive,
    FluidCount,
    PassengerPresent,
    PassengerNotPresent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareType {
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct CircuitCondition {
    pub comparator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_signal: Option<SimpleEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signal: Option<SimpleEntity>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Tile_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Tile {
    pub name: Prototype,
    pub position: Position,
}

/// https://wiki.factorio.com/Blueprint_string_format#Position_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Position {
    #[serde(serialize_with = "serialize_r64")]
    pub x: R64,
    #[serde(serialize_with = "serialize_r64")]
    pub y: R64,
}

/// https://wiki.factorio.com/Blueprint_string_format#Connection_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Connection {
    Single(ConnectionPoint),
    Multiple(Vec<ConnectionData>),
}

/// https://wiki.factorio.com/Blueprint_string_format#Connection_point_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ConnectionPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<Vec<ConnectionData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<Vec<ConnectionData>>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Connection_data_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ConnectionData {
    pub entity_id: EntityNumber,
    // FIXME: this should be an enum which maps to the defined ints, but
    // I don't have the definitions handy right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_id: Option<i32>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Item_request_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemRequest {
    Compact(HashMap<Prototype, ItemCountType>),
    Verbose(Vec<ItemRequestVerbose>),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ItemRequestVerbose {
    pub item: Prototype,
    pub count: ItemCountType,
}

/// https://wiki.factorio.com/Blueprint_string_format#Item_filter_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ItemFilter {
    pub name: Prototype,
    pub index: OneBasedIndex,
}

/// https://wiki.factorio.com/Blueprint_string_format#Infinity_settings_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InfinityFilter>>,
}

/// https://wiki.factorio.com/Blueprint_string_format#Infinity_filter_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct InfinityFilter {
    pub name: Prototype,
    pub count: ItemCountType,
    pub mode: InfinityFilterMode,
    pub index: OneBasedIndex,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfinityFilterMode {
    AtLeast,
    AtMost,
    Exactly,
}

/// https://wiki.factorio.com/Blueprint_string_format#Logistic_filter_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct LogisticFilter {
    pub name: Prototype,
    pub index: OneBasedIndex,
    pub count: ItemCountType,
}

/// Reverse-engineered by hand, contains constant combinator metadata
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ControlFilter {
    pub signal: SignalID,
    pub index: OneBasedIndex,
    pub count: i32,
}

/// https://wiki.factorio.com/Blueprint_string_format#Speaker_parameter_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SpeakerParameter {
    #[serde(serialize_with = "serialize_r64")]
    pub playback_volume: R64,
    pub playback_globally: bool,
    pub allow_polyphony: bool,
}

/// https://wiki.factorio.com/Blueprint_string_format#Speaker_alert_parameter_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_signal_id: Option<SignalID>,
    pub alert_message: String,
}

/// https://wiki.factorio.com/Blueprint_string_format#Color_object
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Color {
    pub r: R64,
    pub g: R64,
    pub b: R64,
    pub a: R64,
}

/// Serialize this R64 value in the same way that Factorio does
/// If the number fractional component is 0, omit the decimal places
fn serialize_r64<S: Serializer>(v: &R64, s: S) -> Result<S::Ok, S::Error> {
    if v.raw().fract() == 0.0 {
        s.serialize_i64(v.raw() as i64)
    } else {
        v.serialize(s)
    }
}
