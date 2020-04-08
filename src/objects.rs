use noisy_float::types::{R32, R64};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DEFAULT_VERSION: u64 = 77310525440;

pub type Prototype = String;
pub type EntityNumber = OneBasedIndex;
pub type ItemStackIndex = u16;
pub type ItemCountType = u32;
pub type GraphicsVariation = u8;
pub type OneBasedIndex = std::num::NonZeroUsize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default)]
/// https://wiki.factorio.com/Blueprint_string_format#Blueprint_book_object
pub struct BlueprintBook {
    pub item: String,
    pub label: String,
    pub label_color: Option<Color>,
    pub blueprints: Vec<BlueprintBookBlueprintValue>,
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
            blueprints: Default::default(),
            active_index: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct BlueprintBookBlueprintValue {
    pub index: usize,
    pub blueprint: Blueprint,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(default)]
/// https://wiki.factorio.com/Blueprint_string_format#Blueprint_object
pub struct Blueprint {
    pub item: String,
    pub label: String,
    pub label_color: Option<Color>,
    pub entities: Vec<Entity>,
    pub tiles: Vec<Tile>,
    pub icons: Vec<Icon>,
    pub schedules: Vec<Schedule>,
    pub version: u64,
}

impl Default for Blueprint {
    fn default() -> Blueprint {
        Blueprint {
            item: "blueprint-book".into(),
            version: DEFAULT_VERSION,
            label: Default::default(),
            label_color: Default::default(),
            entities: Default::default(),
            tiles: Default::default(),
            icons: Default::default(),
            schedules: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Icon_object
pub struct Icon {
    pub index: OneBasedIndex,
    pub signal: SignalID,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#SignalID_object
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Entity_object
pub struct Entity {
    pub entity_number: EntityNumber,
    pub name: Prototype,
    pub position: Position,
    pub direction: Option<u8>,
    pub orientation: Option<R64>,
    pub connections: Option<HashMap<OneBasedIndex, Connection>>,
    pub control_behaviour: Option<ControlBehaviour>,
    pub items: Option<ItemRequest>,
    pub recipe: Option<Prototype>,
    pub bar: Option<ItemStackIndex>,
    pub inventory: Option<Inventory>,
    pub infinity_settings: Option<InfinitySettings>,
    #[serde(rename = "type")]
    pub type_: Option<EntityType>,
    pub input_priority: Option<EntityPriority>,
    pub output_priority: Option<EntityPriority>,
    pub filter: Option<Prototype>,
    pub filters: Option<Vec<ItemFilter>>,
    pub filter_mode: Option<EntityFilterMode>,
    pub override_stack_size: Option<u8>,
    pub drop_position: Option<Position>,
    pub pickup_position: Option<Position>,
    pub request_filters: Option<Vec<LogisticFilter>>,
    pub request_from_buffers: Option<bool>,
    pub parameters: Option<SpeakerParameter>,
    pub alert_parameters: Option<SpeakerAlertParameter>,
    pub auto_launch: Option<bool>,
    pub variation: Option<GraphicsVariation>,
    pub color: Option<Color>,
    pub station: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct ControlBehaviour;

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Inventory_object
pub struct Inventory {
    pub filters: Vec<ItemFilter>,
    pub bar: Option<ItemStackIndex>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Schedule_object
pub struct Schedule {
    pub schedule: Vec<ScheduleRecord>,
    pub locomotives: Vec<EntityNumber>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Schedule_Record_object
pub struct ScheduleRecord {
    pub station: String,
    pub wait_conditions: Vec<WaitCondition>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Wait_Condition_object
pub struct WaitCondition {
    #[serde(rename = "type")]
    pub type_: WaitConditionType,
    pub compare_type: CompareType,
    pub ticks: Option<u64>,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct CircuitCondition;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Tile_object
pub struct Tile {
    pub name: Prototype,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Position_object
pub struct Position {
    pub x: R64,
    pub y: R64,
}

/// https://wiki.factorio.com/Blueprint_string_format#Connection_object
pub type Connection = ConnectionPoint;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Connection_point_object
pub struct ConnectionPoint {
    pub red: Option<Vec<ConnectionData>>,
    pub green: Option<Vec<ConnectionData>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Connection_data_object
pub struct ConnectionData {
    pub entity_id: EntityNumber,
    // FIXME: this should be an enum which maps to the defined ints, but
    // I don't have the definitions handy right now.
    pub circuit_id: Option<i32>,
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Item_filter_object
pub struct ItemFilter {
    pub name: Prototype,
    pub index: OneBasedIndex,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Infinity_settings_object
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool,
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Infinity_filter_object
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Logistic_filter_object
pub struct LogisticFilter {
    pub name: Prototype,
    pub index: OneBasedIndex,
    pub count: ItemCountType,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Speaker_parameter_object
pub struct SpeakerParameter {
    pub playback_volume: R64,
    pub playback_globally: bool,
    pub allow_polyphony: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Speaker_alert_parameter_object
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,
    pub icon_signal_id: Option<SignalID>,
    pub alert_message: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// https://wiki.factorio.com/Blueprint_string_format#Color_object
pub struct Color {
    pub r: R32,
    pub g: R32,
    pub b: R32,
    pub a: R32,
}
