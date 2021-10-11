pub use kurbo::{Affine};
pub use piet::{Color, StrokeStyle, Error};

mod engine;
mod rendering;
mod expressions;
mod components;
mod primitives;
mod runtime;

pub use crate::engine::*;
pub use crate::primitives::*;
pub use crate::rendering::*;
pub use crate::expressions::*;
pub use crate::components::*;
pub use crate::runtime::*;

/*
Creative development environment
for makers of
graphical user interfaces

Creative dev env
for makers of GUIs
[ . . . . . ]
(design, build, and ship.)?

TODO:
=== HIGH
    [x] Refactor PoC code into multi-file, better structure
    [x] Refactor web chassis
    [x] Logging
    [x] Stroke, color, fill
    [x] Sizing
        [x] Browser resize support
        [x] None-sizing
        [x] Transform.align
        [x] Transform.origin
    [x] Expression engine
        [x] variables, declaration & storage
        [x] node IDs
        [x] summonables
            [x] built-in vars like frame count
        [x] MVP rust closures + manifest of deps
    [x] Spreads (née Stacks)
        [x] Decide `primitive` vs. userland `components`
            `components`
        [x] Internal template mechanism for components
            [x] Make `root` into a component definition
        [x] Control-flow `placeholder` (`placeholder`) for inputs/children
            [x] Ensure path forward to userland `placeholders`
        [x] Clipping & Frames
        [x] Control-flow `repeat` for cells & dividers inside template
        [x] Gutter
    [ ] Split out userland code
            [ ] Add a third project to workspace, the sample project
    [ ] Timelines, transitions, t9ables
    [ ] Documentation & usage
    [ ] Mixed mode
        [ ] Native layout
        [ ] Text primitives
        [ ] Native-layer clipping (accumulate clipping path for elements above DOM elements, communicate as Path to web layer for foreignObject + SVG clipping)
        [ ] Form controls
            [ ] ButtonNative (vs. ButtonGroup/ButtonContainer/ButtonFrame?) (or vs. a click event on any ol element)
            [ ] Text input
            [ ] Dropdown
    [ ] Hook up all relevant properties to Property
    [ ] Template compilation
        [ ] Syntax & file design
        [ ] File extension, FS structure decisions
            - Non-HTML extension (so it doesn't get confusedly treated as HTML)
              but "every Dash file is a valid HTML file"
            - Check out Unity FS structure: components, assets, code-behind
        [ ] Code-behind & default implementations
        [ ] Helpful compiler errors, line numbers
    [ ] Refactors
        [x] Bundle Transform into "sugary transform," incl. origin & align; consider a separate transform_matrix property
        [x] Is there a way to better-DRY the shared logic across render-nodes?
            e.g. check out the `get_size` methods for Frame and Spread
        [x] Maybe related to above:  can we DRY the default properties for a render node?
            Perhaps a macro is the answer?
        [ ] Revisit ..Default::default() constructor pattern, field privacy, ergonomics
            - Maybe break out all design-time properties into Properties objects,
              where we do ..Default::default(), then do plain ol' constructors for the rest
        [ ] Update expression/injector ergonomics; perhaps take a pass at macro-ifying injection (and/or removing variadics)
        [ ] Should (can?) `align` be something like (Size::Percent, Size::Percent) instead of a less explicit (f64, f64)?
            Same with `scale`
        [x] Can we do something better than `(Box<......>, Box<.......>)` for `Size`?
        [x] Rename various properties, e.g. bounding_dimens => bounds
        [x] Take a pass on references/ownership in render_render_tree — perhaps &Affine should transfer ownership instead, for example
        [ ] introduce a way to #derive `compute_in_place`
        [x] Better ergonomics for `wrap_render_node_ptr_into_list`
        [x] Evaluate whether to refactor the `unsafe` + PolymorphicType/PolymorphicData approach in expressions + scope data storage
        [ ] literal!() macro for literal property values
            (wrap in `Box::new(PropertyLiteral{value: `)
        [ ] expression!(|engine: &CarbonEngine| -> Color {}) macro

=== MED
    [ ] Dependency management for expressions
        [ ] Declare runtime-accessible metadata around Properties
            objects (probably a macro)
        [ ] Automate registration of Properties objects into PropertiesCoproduct
            (probably the same macro as above)
        [ ] Support `descendent-properties-as-dependancies`, including
            [ ] Piece together properties/deps as eval'd from children
            during render tree traversal
        [ ] Disallow circular dependencies a la Excel

    [ ] Ellipse
    [ ] Path
    [ ] Frames: overflow scrolling
    [ ] Macros for magical expression declaration
    [ ] PoC on macOS, iOS, Android
        [ ] Extricate Engine's dependency on WebRenderContext
    [ ] Image primitive
        [ ] Hook into `piet`s image rendering
        [ ] Asset management
    [ ] Gradients
        [ ] Multiple (stacked, polymorphic) fills
    [ ] Expressions
        [ ] dependency graph, smart traversal, circ. ref detection
        [ ] nested property access & figure out access control (descendent vs ancestor vs global+acyclic+(+private?))
        [ ] parser & syntax
        [ ] control flow ($repeat, $if)
        [ ] dependency graph + caching
    [ ] Tests
    [ ] State + Actions
        [ ] track and update custom states/variables
        [ ] expose API for manipulating state via Actions
    [ ] Authoring tool
        [ ] De/serialization to BESTful (HTML-ish, template) format
        [ ] Drawing tools
        [ ] Layout-building tools
=== LOW
    [ ] Transform.shear
    [ ] Audio/video components
        [ ] "headless" components
    [ ] Expression pre-compiler
        [ ] Enforce uniqueness and valid node/var naming, e.g. for `my_node.var.name`
        [ ] Parser for custom expression lang
    [ ] Debugging chassis
    [ ] Perf-optimize Rectangle (assuming BezPath is inefficient)
 */




/*

Scribble: can (should?) we achieve something like the following?

Sparse constructor pattern (ft. macro)

#derive[(Default)]
struct MyStruct {
    pub a: &str,
    pub b: &str,
    pub c: &str,
}

impl MyStruct {

    fn new() -> Self {

    }

    MyStruct { sparse!(MyStruct { a: "abc", c: "cde"}) }) //note the lack of `b`
}

 */



