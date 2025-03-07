use std::cmp::Ordering;
use std::collections::HashMap;

use crate::parsing::escape_identifier;
use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json;

/// Definition container for an entire Pax cartridge
#[derive(Serialize, Deserialize)]
pub struct PaxManifest {
    pub components: HashMap<String, ComponentDefinition>,
    pub main_component_type_id: String,
    pub expression_specs: Option<HashMap<usize, ExpressionSpec>>,
    pub type_table: TypeTable,
    pub import_paths: std::collections::HashSet<String>,
}

impl Eq for ExpressionSpec {}

impl PartialEq<Self> for ExpressionSpec {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for ExpressionSpec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for ExpressionSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.partial_cmp(&other.id).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExpressionSpec {
    /// Unique id for vtable entry — used for binding a node definition property to vtable
    pub id: usize,

    /// Used to wrap the return type in TypesCoproduct
    pub pascalized_return_type: String,

    /// Representations of symbols used in an expression, and the necessary
    /// metadata to "invoke" those symbols from the runtime
    pub invocations: Vec<ExpressionSpecInvocation>,

    /// String (RIL) representation of the compiled expression
    pub output_statement: String,

    /// String representation of the original input statement
    pub input_statement: String,

    /// Special-handling for Repeat codegen
    pub is_repeat_source_iterable_expression: bool,

    /// The PropertiesCoproduct variant (type_id_escaped) of the inner
    /// type `T` for some iterable repeat source type, e.g. `Vec<T>`
    pub repeat_source_iterable_type_id_escaped: String,
}

/// The spec of an expression `invocation`, the necessary configuration
/// for initializing a pointer to (or copy of, in some cases) the data behind a symbol.
/// For example, if an expression uses `i`, that `i` needs to be "invoked," bound dynamically
/// to some data on the other side of `i` for the context of a particular expression.  `ExpressionSpecInvocation`
/// holds the recipe for such an `invocation`, populated as a part of expression compilation.
#[derive(Serialize, Deserialize, Clone)]
pub struct ExpressionSpecInvocation {
    /// Identifier of the top-level symbol (stripped of `this` or `self`) for nested symbols (`foo` for `foo.bar`) or the
    /// identifier itself for non-nested symbols (`foo` for `foo`)
    pub root_identifier: String,

    /// Identifier escaped so that all operations (like `.` or `[...]`) are
    /// encoded as a valid single identifier
    pub escaped_identifier: String,

    /// Statically known stack offset for traversing Repeat-based scopes at runtime
    pub stack_offset: usize,

    /// Type of the containing Properties struct, for unwrapping from PropertiesCoproduct.  For example, `Foo` for `PropertiesCoproduct::Foo` or `RepeatItem` for PropertiesCoproduct::RepeatItem
    pub properties_coproduct_type: String,

    /// For symbolic invocations that refer to repeat elements, this is the enum identifier within
    /// the TypesCoproduct that represents the appropriate `datum_cast` type
    pub iterable_type_id_escaped: String,

    /// Flags used for particular corner cases of `Repeat` codegen
    pub is_numeric: bool,
    pub is_primitive_nonnumeric: bool,

    /// Flags describing attributes of properties
    pub property_flags: PropertyDefinitionFlags,

    /// Metadata used for nested symbol invocation, like `foo.bar.baz`
    /// Holds an RIL "tail" string for appending to invocation literal bodies,
    /// like `.bar.get().baz.get()` for the nested symbol invocation `foo.bar.baz`.
    pub nested_symbol_tail_literal: String,
    /// Flag describing whether the nested symbolic invocation, e.g. `foo.bar`, ultimately
    /// resolves to a numeric type (as opposed to `is_numeric`, which represents the root of a nested type)
    pub is_nested_numeric: bool,
}

pub const SUPPORTED_NUMERIC_PRIMITIVES: [&str; 13] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f64",
];

pub const SUPPORTED_NONNUMERIC_PRIMITIVES: [&str; 2] = ["String", "bool"];

impl ExpressionSpecInvocation {
    pub fn is_primitive_nonnumeric(property_properties_coproduct_type: &str) -> bool {
        SUPPORTED_NONNUMERIC_PRIMITIVES.contains(&property_properties_coproduct_type)
    }

    pub fn is_numeric(property_properties_coproduct_type: &str) -> bool {
        SUPPORTED_NUMERIC_PRIMITIVES.contains(&property_properties_coproduct_type)
    }
}

/// Container for an entire component definition — includes template, settings,
/// event bindings, property definitions, and compiler + reflection metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentDefinition {
    pub type_id: String,
    pub type_id_escaped: String,
    pub is_main_component: bool,
    pub is_primitive: bool,

    /// Flag describing whether this component definition is a "struct-only component", a
    /// struct decorated with `#[derive(Pax)]` for use as the `T` in `Property<T>`.
    pub is_struct_only_component: bool,

    pub pascal_identifier: String,
    pub module_path: String,

    /// For primitives like Rectangle or Group, a separate import
    /// path is required for the Instance (render context) struct
    /// and the Definition struct.  For primitives, then, we need
    /// to store an additional import path to use when instantiating.
    pub primitive_instance_import_path: Option<String>,
    pub template: Option<Vec<TemplateNodeDefinition>>,
    pub settings: Option<Vec<SettingsSelectorBlockDefinition>>,
    pub events: Option<Vec<EventDefinition>>,
}

impl ComponentDefinition {
    pub fn get_snake_case_id(&self) -> String {
        self.type_id
            .replace("::", "_")
            .replace("/", "_")
            .replace("\\", "_")
            .replace(">", "_")
            .replace("<", "_")
            .replace(".", "_")
    }

    pub fn get_property_definitions<'a>(&self, tt: &'a TypeTable) -> &'a Vec<PropertyDefinition> {
        &tt.get(&self.type_id).unwrap().property_definitions
    }
}

/// Represents an entry within a component template, e.g. a <Rectangle> declaration inside a template
/// Each node in a template is represented by exactly one `TemplateNodeDefinition`, and this is a compile-time
/// concern.  Note the difference between compile-time `definitions` and runtime `instances`.
/// A compile-time `TemplateNodeDefinition` corresponds to a single runtime `RenderNode` instance.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TemplateNodeDefinition {
    /// Component-unique int ID.  Conventionally, id 0 will be the root node for a component's template
    pub id: usize,
    /// Vec of int IDs representing the child TemplateNodeDefinitions of this TemplateNodeDefinition
    pub child_ids: Vec<usize>,
    /// Reference to the unique string ID for a component, e.g. `primitive::Frame` or `component::Stacker`
    pub type_id: String,
    /// Iff this TND is a control-flow node: parsed control flow attributes (slot/if/for)
    pub control_flow_settings: Option<ControlFlowSettingsDefinition>,
    /// IFF this TND is NOT a control-flow node: parsed key-value store of attribute definitions (like `some_key="some_value"`)
    pub settings: Option<Vec<(String, ValueDefinition)>>,
    /// e.g. the `SomeName` in `<SomeName some_key="some_value" />`
    pub pascal_identifier: String,
}

pub type TypeTable = HashMap<String, TypeDefinition>;
pub fn get_primitive_type_table() -> TypeTable {
    let mut ret: TypeTable = Default::default();

    SUPPORTED_NUMERIC_PRIMITIVES.into_iter().for_each(|snp| {
        ret.insert(snp.to_string(), TypeDefinition::primitive(snp));
    });
    SUPPORTED_NONNUMERIC_PRIMITIVES
        .into_iter()
        .for_each(|snnp| {
            ret.insert(snnp.to_string(), TypeDefinition::primitive(snnp));
        });

    ret
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PropertyDefinition {
    /// String representation of the symbolic identifier of a declared Property
    pub name: String,

    /// Flags, used ultimately by ExpressionSpecInvocations, to denote
    /// e.g. whether a property is the `i` or `elem` of a `Repeat`, which allows
    /// for special-handling the RIL that invokes these values
    pub flags: PropertyDefinitionFlags,

    /// Statically known type_id for this Property's associated TypeDefinition
    pub type_id: String,
}

impl PropertyDefinition {
    pub fn get_type_definition<'a>(&'a self, tt: &'a TypeTable) -> &TypeDefinition {
        tt.get(&self.type_id).unwrap()
    }

    pub fn get_inner_iterable_type_definition<'a>(
        &'a self,
        tt: &'a TypeTable,
    ) -> Option<&TypeDefinition> {
        if let Some(ref iiti) = tt.get(&self.type_id).unwrap().inner_iterable_type_id {
            Some(tt.get(iiti).unwrap())
        } else {
            None
        }
    }
}

/// These flags describe the aspects of properties that affect RIL codegen.
/// Properties are divided into modal axes (exactly one value should be true per axis per struct instance)
/// Codegen considers each element of the cartesian product of these axes
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PropertyDefinitionFlags {
    // // //
    // Binding axis
    //
    /// Does this property represent the index `i` in `for (elem, i)` ?
    pub is_binding_repeat_i: bool,
    /// Does this property represent `elem` in `for (elem, i)` OR `for elem in 0..5` ?
    pub is_binding_repeat_elem: bool,

    // // //
    // Source axis
    //
    /// Is the source being iterated over a Range?
    pub is_repeat_source_range: bool,
    /// Is the source being iterated over an iterable, like Vec<T>?
    pub is_repeat_source_iterable: bool,

    /// Describes whether this property is a `Property`-wrapped `T` in `Property<T>`
    /// This distinction affects our ability to dirty-watch a particular property, and
    /// has implications on codegen
    pub is_property_wrapped: bool,
}

/// Describes static metadata surrounding a property, for example
/// the string representation of the property's name and a `TypeInfo`
/// entry for the property's statically discovered type
impl PropertyDefinition {
    /// Shorthand factory / constructor
    pub fn primitive_with_name(type_name: &str, symbol_name: &str) -> Self {
        PropertyDefinition {
            name: symbol_name.to_string(),
            flags: PropertyDefinitionFlags::default(),
            type_id: type_name.to_string(),
        }
    }
}

/// Describes metadata surrounding a property's type, gathered from a combination of static & dynamic analysis
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TypeDefinition {
    /// Program-unique ID for this type
    pub type_id: String,

    /// Same as fully qualified type, but Pascalized to make a suitable enum identifier
    pub type_id_escaped: String,

    /// Unlike type_id, contains no generics data.  Simply used for qualifying / importing a type, like `std::vec::Vec`
    pub import_path: String,

    /// Statically known type_id for this Property's iterable TypeDefinition, that is,
    /// T for some Property<Vec<T>>
    pub inner_iterable_type_id: Option<String>,

    /// A vec of PropertyType, describing known addressable (sub-)properties of this PropertyType
    pub property_definitions: Vec<PropertyDefinition>,
}

impl TypeDefinition {
    pub fn primitive(type_name: &str) -> Self {
        Self {
            type_id_escaped: escape_identifier(type_name.to_string()),
            type_id: type_name.to_string(),
            property_definitions: vec![],
            inner_iterable_type_id: None,
            import_path: type_name.to_string(),
        }
    }

    ///Used by Repeat for source expressions, e.g. the `self.some_vec` in `for elem in self.some_vec`
    pub fn builtin_vec_rc_properties_coproduct(inner_iterable_type_id: String) -> Self {
        let type_id = "std::vec::Vec<std::rc::Rc<PropertiesCoproduct>>";
        Self {
            type_id: type_id.to_string(),
            type_id_escaped: escape_identifier(type_id.to_string()),
            property_definitions: vec![],
            inner_iterable_type_id: Some(inner_iterable_type_id),
            import_path: "std::vec::Vec".to_string(),
        }
    }

    pub fn builtin_range_isize() -> Self {
        let type_id = "std::ops::Range<isize>";
        Self {
            type_id: type_id.to_string(),
            type_id_escaped: escape_identifier(type_id.to_string()),
            property_definitions: vec![],
            inner_iterable_type_id: Some("isize".to_string()),
            import_path: "std::ops::Range".to_string(),
        }
    }

    pub fn builtin_rc_properties_coproduct() -> Self {
        let type_id = "std::rc::Rc<PropertiesCoproduct>";
        Self {
            type_id: type_id.to_string(),
            type_id_escaped: escape_identifier(type_id.to_string()),
            property_definitions: vec![],
            inner_iterable_type_id: None,
            import_path: "std::rc::Rc".to_string(),
        }
    }
}
/// Container for settings values, storing all possible
/// variants, populated at parse-time and used at compile-time
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum ValueDefinition {
    #[default]
    Undefined, //Used for `Default`
    LiteralValue(String),
    Block(LiteralBlockDefinition),
    /// (Expression contents, vtable id binding)
    Expression(String, Option<usize>),
    /// (Expression contents, vtable id binding)
    Identifier(String, Option<usize>),
    EventBindingTarget(String),
}

/// Container for holding parsed data describing a Repeat (`for`)
/// predicate, for example the `(elem, i)` in `for (elem, i) in foo` or
/// the `elem` in `for elem in foo`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ControlFlowRepeatPredicateDefinition {
    ElemId(String),
    ElemIdIndexId(String, String),
}

/// Container for storing parsed control flow information, for
/// example the string (PAXEL) representations of condition / slot / repeat
/// expressions and the related vtable ids (for "punching" during expression compilation)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ControlFlowSettingsDefinition {
    pub condition_expression_paxel: Option<String>,
    pub condition_expression_vtable_id: Option<usize>,
    pub slot_index_expression_paxel: Option<String>,
    pub slot_index_expression_vtable_id: Option<usize>,
    pub repeat_predicate_definition: Option<ControlFlowRepeatPredicateDefinition>,
    pub repeat_source_definition: Option<ControlFlowRepeatSourceDefinition>,
}

/// Container describing the possible variants of a Repeat source
/// — namely a range expression in PAXEL or a symbolic binding
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ControlFlowRepeatSourceDefinition {
    pub range_expression_paxel: Option<String>,
    pub vtable_id: Option<usize>,
    pub symbolic_binding: Option<String>,
}

/// Container for parsed Settings blocks (inside `@settings`)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsSelectorBlockDefinition {
    pub selector: String,
    pub value_block: LiteralBlockDefinition,
}

/// Container for a parsed
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LiteralBlockDefinition {
    pub explicit_type_pascal_identifier: Option<String>,
    pub settings_key_value_pairs: Vec<(String, ValueDefinition)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Number {
    Float(f64),
    Int(isize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Unit {
    Pixels,
    Percent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventDefinition {
    pub key: String,
    pub value: Vec<String>,
}
