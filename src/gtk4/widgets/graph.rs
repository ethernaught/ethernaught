use std::cell::RefCell;
use gtk4::{glib, Buildable, Orientation, Snapshot, StateFlags, Widget};
use gtk4::glib::property::PropertyGet;
use gtk4::graphene::Rect;
use gtk4::prelude::{DisplayExt, ObjectExt, SnapshotExt, StyleContextExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

#[derive(Default)]
pub struct GraphImpl {
    points: RefCell<Vec<u32>>
}

#[glib::object_subclass]
impl ObjectSubclass for GraphImpl {

    const NAME: &'static str = "Graph";
    type ParentType = Widget;
    type Type = Graph;

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

        // Draw a solid background with Cairo
        let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));
        //cr.set_source_rgb(0.2, 0.4, 0.6);
        //cr.rectangle(0.0, 0.0, width as f64, height as f64);
        //cr.fill().unwrap();




        let widget = self.obj();
        let style_context = widget.style_context();

        style_context.set_state(StateFlags::NORMAL);
        let color = style_context.color();

        let padding = style_context.padding();

        if let Some(background) = style_context.lookup_color("background-color") {
            cr.set_source_rgba(background.red() as f64, background.green() as f64, background.blue() as f64, background.alpha() as f64);
            cr.paint();
        }

        let allocation = self.obj().allocation();
        let width = allocation.width() as f64;
        let height = allocation.height() as f64;

        cr.set_line_width(1.0);

        cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);

        let points = self.points.borrow();
        if points.len() < 2 {
            cr.move_to(0.0, height);
            cr.line_to(width, height);
            cr.stroke().unwrap();

            return;
        }

        let min = *points.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as f64;
        let max = *points.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as f64;

        let range = if max > min { max - min } else { 1.0 };
        let distance = 4.0;// * widget.screen().unwrap().resolution() / 96.0;

        cr.move_to(0.0, height - ((points[0] as f64 - min) / range) * height);

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

    /*
    fn draw(&self, cr: &Context) -> Propagation {
        let widget = self.obj();
        let style_context = widget.style_context();

        style_context.set_state(StateFlags::NORMAL);
        let color = style_context.color(StateFlags::NORMAL);

        let padding = style_context.padding(StateFlags::NORMAL);

        if let Ok(background) = style_context.style_property_for_state("background-color", StateFlags::NORMAL).get::<RGBA>() {
            cr.set_source_rgba(background.red(), background.green(), background.blue(), background.alpha());
            cr.paint();
        }


        let allocation = self.obj().allocation();
        let width = allocation.width() as f64;
        let height = allocation.height() as f64;

        cr.set_line_width(1.0);

        cr.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());

        let points = self.points.borrow();
        if points.len() < 2 {
            cr.move_to(0.0, height);
            cr.line_to(width, height);
            cr.stroke().unwrap();

            return Proceed;
        }

        let min = *points.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as f64;
        let max = *points.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as f64;

        let range = if max > min { max - min } else { 1.0 };
        let distance = 4.0 * widget.screen().unwrap().resolution() / 96.0;

        cr.move_to(0.0, height - ((points[0] as f64 - min) / range) * height);

        for (i, &point) in points.iter().enumerate() {
            let x = i as f64 * distance;
            let y = height - ((point as f64 - min) / range) * height;
            cr.line_to(x, y);
        }

        cr.stroke().unwrap();

        Proceed
    }
    */
}

glib::wrapper! {
    pub struct Graph(ObjectSubclass<GraphImpl>)
        @extends Widget, @implements Buildable;
}

impl Graph {

    pub fn new() -> Self {
        let _self = glib::Object::builder::<Graph>().build();
        _self
    }

    pub fn set_points(&self, points: Vec<u32>) {
        *self.imp().points.borrow_mut() = points;
        self.queue_draw();
    }

    pub fn add_point(&self, point: u32) {
        let allocation = self.allocation();
        let width = allocation.width();
        let distance = 4.0;// * self.screen().unwrap().resolution() / 96.0;

        if self.imp().points.borrow().len() >= width as usize / distance as usize {
            let size_to_remove = self.imp().points.borrow_mut().len() - width as usize / distance as usize;
            self.imp().points.borrow_mut().drain(0..size_to_remove);
        }

        self.imp().points.borrow_mut().push(point);
        self.queue_draw();
    }

    pub fn get_points(&self) -> Vec<u32> {
        self.imp().points.borrow().clone()
    }

    pub fn clear_points(&self) {
        self.imp().points.borrow_mut().clear();
    }
}
