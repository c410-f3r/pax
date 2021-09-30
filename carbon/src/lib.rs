use kurbo::{Affine, BezPath, Point, Vec2};
pub use piet::{Color, Error, StrokeStyle};
use piet::RenderContext;
pub use piet_web::WebRenderContext;

/*
TODO:
=== HIGH
    [ ] Refactor PoC code into multi-file, better structure
    [x] Refactor web chassis
    [x] Logging
    [ ] Stroke, color, fill
    [ ] Sizing + resizing support
        [ ] Transform.align
    [ ] Transform.origin
    [ ] Ellipse
    [ ] Clean up warnings
    [ ] Expression engine, state
    [ ] Text prefabs + support layer for Web adapter
        [ ] Also clean up the drawing_loop logic for Web
        [ ] Clipping
    [ ] Layouts (stacks)
        [ ] Clipping
    [ ] Timelines, transitions, t9ables
=== MED
    [ ] PoC on iOS, Android
    [ ] Gradients
    [ ] De/serializing for BESTful format
    [ ] Authoring tool
        [ ] Drawing tools
        [ ] Layout-building tools
=== LOW
    [ ] Transform.shear
    [ ] Debugging chassis
    [ ] Perf-optimize Rectangle (assuming BezPath is inefficient)
 */

trait RenderNode
{
    fn get_children(&self) -> Option<&Vec<Box<dyn RenderNode>>>;
    fn get_transform(&self) -> &Affine;
    fn render(&self, rc: &mut WebRenderContext, transform: &Affine);
}

struct Group {
    children: Vec<Box<dyn RenderNode>>,
    transform: Affine,
}

impl RenderNode for Group {
    fn get_children(&self) -> Option<&Vec<Box<dyn RenderNode>>> {
        Some(&self.children)
    }
    fn get_transform(&self) -> &Affine {
        &self.transform
    }
    fn render(&self, rc: &mut WebRenderContext, transform: &Affine) {}
}

struct Stroke {
    color: Color,
    style: StrokeStyle,
}

struct Rectangle {
    width: f64,
    height: f64,
    transform: Affine,
    stroke: Stroke,
}

impl RenderNode for Rectangle {
    fn get_children(&self) -> Option<&Vec<Box<dyn RenderNode>>> {
        None
    }
    fn get_transform(&self) -> &Affine {
        &self.transform
    }
    fn render(&self, rc: &mut WebRenderContext, transform: &Affine) {

        let bp_width: f64 = self.width;
        let bp_height: f64 = self.height;
        let mut bez_path = BezPath::new();
        bez_path.move_to(Point::new(-bp_width / 2., -bp_height / 2.));
        bez_path.line_to(Point::new(bp_width / 2., -bp_height / 2.));
        bez_path.line_to(Point::new(bp_width / 2., bp_height / 2.));
        bez_path.line_to(Point::new(-bp_width / 2., bp_height / 2.));
        bez_path.line_to(Point::new(-bp_width / 2., -bp_height / 2.));
        bez_path.close_path();

        let transformed_bez_path = *transform * bez_path;
        let duplicate_transformed_bez_path = transformed_bez_path.clone();

        let phased_color = Color::rgba(227., 225., 27., 0.25);
        rc.fill(transformed_bez_path, &phased_color);

        rc.stroke(duplicate_transformed_bez_path, &self.stroke.color, 3.0);
    }
}

// Public method for consumption by engine chassis, e.g. WebChassis
pub fn get_engine(logger: fn(&str)) -> CarbonEngine {
    return CarbonEngine::new(logger);
}

pub struct SceneGraph {
    root: Box<dyn RenderNode>
}

pub struct CarbonEngine {
    logger: fn(&str),
    frames_elapsed: u32,
    scene_graph: SceneGraph,
}

impl CarbonEngine {

    fn new(logger: fn(&str)) -> Self {
        CarbonEngine {
            logger,
            frames_elapsed: 0,
            scene_graph: SceneGraph {
                root: Box::new(Group {
                    transform: Affine::default(),
                    children: vec![
                        Box::new(Rectangle {
                            width: 50.0,
                            height: 50.0,
                            transform: Affine::translate(Vec2 { x: 550.0, y: 550.0 }),
                            stroke: Stroke {
                                color: Color::rgb8(150, 20, 200),
                                style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                            },
                        }),
                        Box::new(Rectangle {
                            width: 100.0,
                            height: 100.0,
                            transform: Affine::translate(Vec2 { x: 350.0, y: 350.0 }),
                            stroke: Stroke {
                                color: Color::rgb8(150, 20, 200),
                                style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                            },
                        }),
                        Box::new(Rectangle {
                            width: 250.0,
                            height: 250.0,
                            transform: Affine::translate(Vec2 { x: 750.0, y: 750.0 }),
                            stroke: Stroke {
                                color: Color::rgb8(150, 20, 200),
                                style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                            },
                        }),
                        Box::new(Group {
                            transform: Affine::translate(Vec2{x: 800.0, y:-200.0}),
                            children: vec![
                                Box::new(Rectangle {
                                    width: 50.0,
                                    height: 50.0,
                                    transform: Affine::translate(Vec2 { x: 550.0, y: 550.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                                Box::new(Rectangle {
                                    width: 100.0,
                                    height: 100.0,
                                    transform: Affine::translate(Vec2 { x: 350.0, y: 350.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                                Box::new(Rectangle {
                                    width: 250.0,
                                    height: 250.0,
                                    transform: Affine::translate(Vec2 { x: 750.0, y: 750.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                            ],
                        }),
                        Box::new(Group {
                            transform: Affine::translate(Vec2{x: 400.0, y:-100.0}),
                            children: vec![
                                Box::new(Rectangle {
                                    width: 50.0,
                                    height: 50.0,
                                    transform: Affine::translate(Vec2 { x: 550.0, y: 550.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                                Box::new(Rectangle {
                                    width: 100.0,
                                    height: 100.0,
                                    transform: Affine::translate(Vec2 { x: 350.0, y: 350.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                                Box::new(Rectangle {
                                    width: 250.0,
                                    height: 250.0,
                                    transform: Affine::translate(Vec2 { x: 750.0, y: 750.0 }),
                                    stroke: Stroke {
                                        color: Color::rgb8(150, 20, 200),
                                        style: StrokeStyle {line_cap: None, dash: None, line_join: None, miter_limit: None,},
                                    },
                                }),
                            ],
                        })
                    ],
                }),
            },
        }
    }

    fn log(&self, msg: &str) {
        (self.logger)(msg);
    }

    fn render_scene_graph(&self, rc: &mut WebRenderContext) -> Result<(), Error> {
        // hello world scene graph
        //           (root)
        //           /    \
        //       (rect)  (rect)

        // 1. find lowest node (last child of last node), accumulating transform along the way
        // 2. start rendering, from lowest node on-up
        self.recurse_render_scene_graph(rc, &self.scene_graph.root, &Affine::default());
        Ok(())
    }

    fn recurse_render_scene_graph(&self, rc: &mut WebRenderContext, node: &Box<dyn RenderNode>, accumulated_transform: &Affine) -> Result<(), Error> {
        // Recurse:
        //  - iterate backwards over children (lowest first); recurse until there are no more descendants.  track transform matrix along the way.
        //  - we now have the back-most leaf node.  Render it.  Return.
        //  - we're now at the second back-most leaf node.  Render it.  Return ...
        //  - done

        let new_accumulated_transform = *accumulated_transform * *node.get_transform();

        match node.get_children() {
            Some(children) => {
                //keep recursing
                for i in (0..children.len()).rev() {
                    //note that we're iterating starting from the last child
                    let child = children.get(i); //TODO: ?-syntax
                    match child {
                        None => { return Ok(()) },
                        Some(child) => {
                            &self.recurse_render_scene_graph(rc, child, &new_accumulated_transform);
                        }
                    }
                }
            },
            None => {
                //this is a leaf node.  render it & return.
                let theta = (self.frames_elapsed as f64 / 50.0).sin() * 5.0 + (self.frames_elapsed as f64 / 50.0);
                let frame_rotated_transform = new_accumulated_transform * Affine::rotate(theta);
                node.render(rc, &frame_rotated_transform);
            }
        }

        //TODO: Now that children have been rendered, if there's rendering to be done at this node,
        //      (e.g. for layouts, perhaps virtual nodes like $repeat), do that rendering here

        Ok(())
    }


    pub fn tick_and_render(&mut self, rc: &mut WebRenderContext) -> Result<(), Error> {
        rc.clear(Color::rgb8(0, 0, 0));

        self.render_scene_graph(rc);
        self.frames_elapsed = self.frames_elapsed + 1;

        // Logging example:
        // self.log(format!("Frame: {}", self.frames_elapsed).as_str());

        Ok(())
    }


    pub fn tick_and_render_disco_taps(&mut self, rc: &mut WebRenderContext) -> Result<(), Error> {
        let hue = (((self.frames_elapsed + 1) as f64 * 2.0) as i64 % 360) as f64;
        let current_color = Color::hlc(hue, 75.0, 127.0);
        rc.clear(current_color);

        for x in 0..20 {
            for y in 0..12 {
                let bp_width: f64 = 100.;
                let bp_height: f64 = 100.;
                let mut bez_path = BezPath::new();
                bez_path.move_to(Point::new(-bp_width / 2., -bp_height / 2.));
                bez_path.line_to(Point::new(bp_width / 2., -bp_height / 2.));
                bez_path.line_to(Point::new(bp_width / 2., bp_height / 2.));
                bez_path.line_to(Point::new(-bp_width / 2., bp_height / 2.));
                bez_path.line_to(Point::new(-bp_width / 2., -bp_height / 2.));
                bez_path.close_path();

                let theta = self.frames_elapsed as f64 * (0.04 + (x as f64 + y as f64 + 10.) / 64.) / 10.;
                let transform =
                    Affine::translate(Vec2::new(x as f64 * bp_width, y as f64 * bp_height)) *
                    Affine::rotate(theta) *
                    Affine::scale(theta.sin() * 1.2)
                ;

                let transformed_bez_path = transform * bez_path;

                let phased_hue = ((hue + 180.) as i64 % 360) as f64;
                let phased_color = Color::hlc(phased_hue, 75., 127.);
                rc.fill(transformed_bez_path, &phased_color);
            }
        }

        self.frames_elapsed = self.frames_elapsed + 1;
        Ok(())
    }
}

