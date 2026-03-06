use std::cell::RefCell;

use gtk4::{glib, Buildable, Orientation, Snapshot, Widget};
use gtk4::graphene::Rect;
use gtk4::prelude::{ObjectExt, SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

mod imp {
    use super::*;
    use gtk4::prelude::StyleContextExt;
    use gtk4::StateFlags;
    use gtk4::subclass::prelude::ObjectSubclassExt;

    #[derive(Default)]
    pub struct GraphImpl {
        pub points: RefCell<Vec<u32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GraphImpl {
        
        const NAME: &'static str = "Graph";
        type Type = super::Graph;
        type ParentType = Widget;

        fn class_init(class: &mut Self::Class) {
            class.set_css_name("graph");
        }
    }

    impl ObjectImpl for GraphImpl {}

    impl WidgetImpl for GraphImpl {

        fn snapshot(&self, snapshot: &Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));

            let style_context = widget.style_context();
            style_context.set_state(StateFlags::NORMAL);

            let color = style_context.color();

            if let Some(background) = style_context.lookup_color("background-color") {
                cr.set_source_rgba(
                    background.red() as f64,
                    background.green() as f64,
                    background.blue() as f64,
                    background.alpha() as f64,
                );
                cr.paint().unwrap();
            }

            let width = widget.width() as f64;
            let height = widget.height() as f64;

            cr.set_line_width(1.0);
            cr.set_source_rgba(
                color.red() as f64,
                color.green() as f64,
                color.blue() as f64,
                color.alpha() as f64,
            );

            let points = self.points.borrow();

            if points.len() < 2 {
                cr.move_to(0.0, height);
                cr.line_to(width, height);
                cr.stroke().unwrap();
                return;
            }

            let min = *points.iter().min().unwrap() as f64;
            let max = *points.iter().max().unwrap() as f64;
            let range = if max > min { max - min } else { 1.0 };

            let distance = 4.0;

            let first_y = height - ((points[0] as f64 - min) / range) * height;
            cr.move_to(0.0, first_y);

            for (i, &point) in points.iter().enumerate() {
                let x = i as f64 * distance;
                let y = height - ((point as f64 - min) / range) * height;
                cr.line_to(x, y);
            }

            cr.stroke().unwrap();
        }

        fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                Orientation::Horizontal => (MIN_WIDTH, MIN_WIDTH, -1, -1),
                Orientation::Vertical => (MIN_HEIGHT, MIN_HEIGHT, -1, -1),
                _ => unimplemented!()
            }
        }
    }
}

glib::wrapper! {
    pub struct Graph(ObjectSubclass<imp::GraphImpl>)
        @extends Widget,
        @implements Buildable;
}

impl Graph {

    pub fn new() -> Self {
        glib::Object::builder::<Graph>().build()
    }

    pub fn set_points(&self, points: Vec<u32>) {
        self.imp().points.replace(points);
        self.queue_draw();
    }

    pub fn add_point(&self, point: u32) {
        let width = self.width();
        let distance = 4usize;

        let max_points = if width > 0 {
            (width as usize / distance).max(1)
        } else {
            1
        };

        let mut points = self.imp().points.borrow_mut();

        if points.len() >= max_points {
            let remove_count = points.len() + 1 - max_points;
            points.drain(0..remove_count);
        }

        points.push(point);
        drop(points);

        self.queue_draw();
    }

    pub fn points(&self) -> Vec<u32> {
        self.imp().points.borrow().clone()
    }

    pub fn clear_points(&self) {
        self.imp().points.borrow_mut().clear();
        self.queue_draw();
    }
}

impl Default for Graph {

    fn default() -> Self {
        Self::new()
    }
}
