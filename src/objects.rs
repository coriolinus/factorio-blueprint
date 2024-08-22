use crate::Container;
use noisy_float::types::R64;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

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
    pub snap_to_grid: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_snapping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_relative_to_grid: Option<Position>,
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
            snap_to_grid: Default::default(),
            absolute_snapping: Default::default(),
            position_relative_to_grid: Default::default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<u8>,
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

impl Entity {
    pub const fn new(entity_number: EntityNumber, name: Prototype, position: Position) -> Self {
        Self {
            entity_number,
            name,
            position,
            direction: None,
            orientation: None,
            connections: None,
            control_behavior: None,
            items: None,
            recipe: None,
            bar: None,
            inventory: None,
            infinity_settings: None,
            type_: None,
            input_priority: None,
            output_priority: None,
            filter: None,
            filters: None,
            filter_mode: None,
            override_stack_size: None,
            drop_position: None,
            pickup_position: None,
            request_filters: None,
            request_from_buffers: None,
            parameters: None,
            alert_parameters: None,
            auto_launch: None,
            variation: None,
            color: None,
            station: None,
            switch_state: None,
            manual_trains_limit: None,
            neighbours: None,
        }
    }

    pub const fn with_direction(mut self, direction: u8) -> Self {
        self.direction = Some(direction);
        self
    }
    pub fn with_orientation(mut self, orientation: f64) -> Self {
        self.orientation = Some(R64::new(orientation));
        self
    }
    pub fn with_connections(mut self, connections: EntityConnections) -> Self {
        self.connections = Some(connections);
        self
    }
    pub fn with_control_behavior(mut self, control_behavior: ControlBehavior) -> Self {
        self.control_behavior = Some(control_behavior);
        self
    }
    pub fn with_items(mut self, items: ItemRequest) -> Self {
        self.items = Some(items);
        self
    }
    pub fn with_recipe(mut self, recipe: Prototype) -> Self {
        self.recipe = Some(recipe);
        self
    }
    pub const fn with_bar(mut self, bar: ItemStackIndex) -> Self {
        self.bar = Some(bar);
        self
    }
    pub fn with_inventory(mut self, inventory: Inventory) -> Self {
        self.inventory = Some(inventory);
        self
    }
    pub fn with_infinity_settings(mut self, infinity_settings: InfinitySettings) -> Self {
        self.infinity_settings = Some(infinity_settings);
        self
    }
    pub const fn with_type(mut self, type_: EntityType) -> Self {
        self.type_ = Some(type_);
        self
    }
    pub const fn with_input_priority(mut self, input_priority: EntityPriority) -> Self {
        self.input_priority = Some(input_priority);
        self
    }
    pub const fn with_output_priority(mut self, output_priority: EntityPriority) -> Self {
        self.output_priority = Some(output_priority);
        self
    }
    pub fn with_filter(mut self, filter: Prototype) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn with_filters(mut self, filters: Vec<ItemFilter>) -> Self {
        self.filters = Some(filters);
        self
    }
    pub const fn with_filter_mode(mut self, filter_mode: EntityFilterMode) -> Self {
        self.filter_mode = Some(filter_mode);
        self
    }
    pub const fn with_override_stack_size(mut self, override_stack_size: u8) -> Self {
        self.override_stack_size = Some(override_stack_size);
        self
    }
    pub const fn with_drop_position(mut self, drop_position: Position) -> Self {
        self.drop_position = Some(drop_position);
        self
    }
    pub const fn with_pickup_position(mut self, pickup_position: Position) -> Self {
        self.pickup_position = Some(pickup_position);
        self
    }
    pub fn with_request_filters(mut self, request_filters: Vec<LogisticFilter>) -> Self {
        self.request_filters = Some(request_filters);
        self
    }
    pub const fn with_request_from_buffers(mut self, request_from_buffers: bool) -> Self {
        self.request_from_buffers = Some(request_from_buffers);
        self
    }
    pub const fn with_parameters(mut self, parameters: SpeakerParameter) -> Self {
        self.parameters = Some(parameters);
        self
    }
    pub fn with_alert_parameters(mut self, alert_parameters: SpeakerAlertParameter) -> Self {
        self.alert_parameters = Some(alert_parameters);
        self
    }
    pub const fn with_auto_launch(mut self, auto_launch: bool) -> Self {
        self.auto_launch = Some(auto_launch);
        self
    }
    pub const fn with_variation(mut self, variation: GraphicsVariation) -> Self {
        self.variation = Some(variation);
        self
    }
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn with_station(mut self, station: String) -> Self {
        self.station = Some(station);
        self
    }
    pub const fn with_switch_state(mut self, switch_state: bool) -> Self {
        self.switch_state = Some(switch_state);
        self
    }
    pub const fn with_manual_trains_limit(mut self, manual_trains_limit: u32) -> Self {
        self.manual_trains_limit = Some(manual_trains_limit);
        self
    }
    pub fn with_neighbours(mut self, neighbours: Vec<EntityNumber>) -> Self {
        self.neighbours = Some(neighbours);
        self
    }
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

impl ControlBehavior {
    pub const fn new() -> Self {
        Self {
            connect_to_logistic_network: None,
            arithmetic_conditions: None,
            decider_conditions: None,
            logistic_condition: None,
            filters: None,
            is_on: None,
            use_colors: None,
            circuit_condition: None,
            circuit_mode_of_operation: None,
            circuit_enable_disable: None,
            circuit_contents_read_mode: None,
            circuit_hand_read_mode: None,
            circuit_read_hand_contents: None,
            circuit_set_stack_size: None,
            stack_control_input_signal: None,
            circuit_parameters: None,
            output_signal: None,
            read_from_train: None,
            read_stopped_train: None,
            read_trains_count: None,
            set_trains_limit: None,
            send_to_train: None,
            train_stopped_signal: None,
            trains_count_signal: None,
            trains_limit_signal: None,
            read_logistics: None,
            read_robot_stats: None,
            available_construction_output_signal: None,
            available_logistic_output_signal: None,
            total_construction_output_signal: None,
            total_logistic_output_signal: None,
            circuit_open_gate: None,
            circuit_read_sensor: None,
            circuit_close_signal: None,
            circuit_read_signal: None,
        }
    }

    pub const fn with_connect_to_logistic_network(
        mut self,
        connect_to_logistic_network: bool,
    ) -> Self {
        self.connect_to_logistic_network = Some(connect_to_logistic_network);
        self
    }
    pub fn with_arithmetic_conditions(
        mut self,
        arithmetic_conditions: ArithmeticConditions,
    ) -> Self {
        self.arithmetic_conditions = Some(arithmetic_conditions);
        self
    }
    pub fn with_decider_conditions(mut self, decider_conditions: DeciderConditions) -> Self {
        self.decider_conditions = Some(decider_conditions);
        self
    }
    pub fn with_logistic_condition(mut self, logistic_condition: LogisticCondition) -> Self {
        self.logistic_condition = Some(logistic_condition);
        self
    }
    pub fn with_filters(mut self, filters: Vec<ControlFilter>) -> Self {
        self.filters = Some(filters);
        self
    }
    pub const fn with_is_on(mut self, is_on: bool) -> Self {
        self.is_on = Some(is_on);
        self
    }
    pub const fn with_use_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = Some(use_colors);
        self
    }
    pub fn with_circuit_condition(mut self, circuit_condition: CircuitCondition) -> Self {
        self.circuit_condition = Some(circuit_condition);
        self
    }
    pub const fn with_circuit_mode_of_operation(
        mut self,
        circuit_mode_of_operation: CircuitModeOfOperation,
    ) -> Self {
        self.circuit_mode_of_operation = Some(circuit_mode_of_operation);
        self
    }
    pub const fn with_circuit_enable_disable(mut self, circuit_enable_disable: bool) -> Self {
        self.circuit_enable_disable = Some(circuit_enable_disable);
        self
    }
    pub const fn with_circuit_contents_read_mode(
        mut self,
        circuit_contents_read_mode: ContentReadMode,
    ) -> Self {
        self.circuit_contents_read_mode = Some(circuit_contents_read_mode);
        self
    }
    pub const fn with_circuit_hand_read_mode(
        mut self,
        circuit_hand_read_mode: ContentReadMode,
    ) -> Self {
        self.circuit_hand_read_mode = Some(circuit_hand_read_mode);
        self
    }
    pub const fn with_circuit_read_hand_contents(
        mut self,
        circuit_read_hand_contents: bool,
    ) -> Self {
        self.circuit_read_hand_contents = Some(circuit_read_hand_contents);
        self
    }
    pub const fn with_circuit_set_stack_size(mut self, circuit_set_stack_size: bool) -> Self {
        self.circuit_set_stack_size = Some(circuit_set_stack_size);
        self
    }
    pub fn with_stack_control_input_signal(
        mut self,
        stack_control_input_signal: SimpleEntity,
    ) -> Self {
        self.stack_control_input_signal = Some(stack_control_input_signal);
        self
    }
    pub const fn with_circuit_parameters(
        mut self,
        circuit_parameters: SpeakerCircuitParameters,
    ) -> Self {
        self.circuit_parameters = Some(circuit_parameters);
        self
    }
    pub fn with_output_signal(mut self, output_signal: SimpleEntity) -> Self {
        self.output_signal = Some(output_signal);
        self
    }
    pub const fn with_read_from_train(mut self, read_from_train: bool) -> Self {
        self.read_from_train = Some(read_from_train);
        self
    }
    pub const fn with_read_stopped_train(mut self, read_stopped_train: bool) -> Self {
        self.read_stopped_train = Some(read_stopped_train);
        self
    }
    pub const fn with_read_trains_count(mut self, read_trains_count: bool) -> Self {
        self.read_trains_count = Some(read_trains_count);
        self
    }
    pub const fn with_set_trains_limit(mut self, set_trains_limit: bool) -> Self {
        self.set_trains_limit = Some(set_trains_limit);
        self
    }
    pub const fn with_send_to_train(mut self, send_to_train: bool) -> Self {
        self.send_to_train = Some(send_to_train);
        self
    }
    pub fn with_train_stopped_signal(mut self, train_stopped_signal: SimpleEntity) -> Self {
        self.train_stopped_signal = Some(train_stopped_signal);
        self
    }
    pub fn with_trains_count_signal(mut self, trains_count_signal: SimpleEntity) -> Self {
        self.trains_count_signal = Some(trains_count_signal);
        self
    }
    pub fn with_trains_limit_signal(mut self, trains_limit_signal: SimpleEntity) -> Self {
        self.trains_limit_signal = Some(trains_limit_signal);
        self
    }
    pub const fn with_read_logistics(mut self, read_logistics: bool) -> Self {
        self.read_logistics = Some(read_logistics);
        self
    }
    pub const fn with_read_robot_stats(mut self, read_robot_stats: bool) -> Self {
        self.read_robot_stats = Some(read_robot_stats);
        self
    }
    pub fn with_available_construction_output_signal(
        mut self,
        available_construction_output_signal: SimpleEntity,
    ) -> Self {
        self.available_construction_output_signal = Some(available_construction_output_signal);
        self
    }
    pub fn with_available_logistic_output_signal(
        mut self,
        available_logistic_output_signal: SimpleEntity,
    ) -> Self {
        self.available_logistic_output_signal = Some(available_logistic_output_signal);
        self
    }
    pub fn with_total_construction_output_signal(
        mut self,
        total_construction_output_signal: SimpleEntity,
    ) -> Self {
        self.total_construction_output_signal = Some(total_construction_output_signal);
        self
    }
    pub fn with_total_logistic_output_signal(
        mut self,
        total_logistic_output_signal: SimpleEntity,
    ) -> Self {
        self.total_logistic_output_signal = Some(total_logistic_output_signal);
        self
    }
    pub const fn with_circuit_open_gate(mut self, circuit_open_gate: bool) -> Self {
        self.circuit_open_gate = Some(circuit_open_gate);
        self
    }
    pub const fn with_circuit_read_sensor(mut self, circuit_read_sensor: bool) -> Self {
        self.circuit_read_sensor = Some(circuit_read_sensor);
        self
    }
    pub const fn with_circuit_close_signal(mut self, circuit_close_signal: bool) -> Self {
        self.circuit_close_signal = Some(circuit_close_signal);
        self
    }
    pub const fn with_circuit_read_signal(mut self, circuit_read_signal: bool) -> Self {
        self.circuit_read_signal = Some(circuit_read_signal);
        self
    }
}

impl Default for ControlBehavior {
    fn default() -> Self {
        Self::new()
    }
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
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_signal: Option<SignalID>,
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
    pub comparator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_signal: Option<SignalID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_count_from_input: Option<bool>,
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct Position {
    #[serde(serialize_with = "serialize_r64")]
    pub x: R64,
    #[serde(serialize_with = "serialize_r64")]
    pub y: R64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: R64::new(x),
            y: R64::new(y),
        }
    }

    pub const fn x(self) -> f64 {
        self.x.const_raw()
    }

    pub const fn y(self) -> f64 {
        self.y.const_raw()
    }
}

impl core::ops::Add for Position {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl core::ops::Sub for Position {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
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
