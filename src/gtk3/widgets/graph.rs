use std::cell::RefCell;
use std::cmp::max;
use gtk::gdk::{EventMask, WindowAttr, WindowType, WindowWindowClass, RGBA};
use gtk::{gdk, glib, Allocation, Buildable, Misc, Orientation, StateFlags, Widget};
use gtk::cairo::Context;
use gtk::glib::Propagation;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{StyleContextExt, WidgetExt};
use gtk::subclass::prelude::{ClassStruct, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl, WidgetImplExt};

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

    fn realize(&self) {
        let widget = self.obj();
        let allocation = widget.allocation();

        let attr = WindowAttr {
            title: None,
            event_mask: EventMask::empty(),
            x: Some(allocation.x()),
            y: Some(allocation.y()),
            width: allocation.width(),
            height: allocation.height(),
            wclass: WindowWindowClass::InputOutput,
            visual: None,
            window_type: WindowType::Child,
            cursor: None,
            override_redirect: false,
            type_hint: None
        };

        let parent_window = widget.parent_window().unwrap();
        let window = gdk::Window::new(Some(&parent_window), &attr);

        widget.register_window(&window);
        widget.set_window(window);
        widget.set_realized(true);

        widget.set_can_focus(true);
    }

    fn adjust_size_request(&self, orientation: Orientation, minimum_size: &mut i32, natural_size: &mut i32) {
        match orientation {
            Orientation::Horizontal => {
                *minimum_size = MIN_WIDTH;
                if *natural_size < MIN_WIDTH {
                    *natural_size = MIN_WIDTH;
                }
            }
            Orientation::Vertical => {
                *minimum_size = MIN_HEIGHT;
                if *natural_size < MIN_HEIGHT {
                    *natural_size = MIN_HEIGHT;
                }
            }
            _ => unimplemented!()
        }

        self.parent_adjust_size_request(orientation, minimum_size, natural_size);
    }

    fn size_allocate(&self, allocation: &Allocation) {
        let widget = self.obj();

        widget.set_allocation(allocation);
        if widget.is_realized() {
            if let Some(window) = widget.window() {
                window.move_resize(allocation.x(), allocation.y(), allocation.width(), allocation.height());
            }
        }
    }
}

glib::wrapper! {
    pub struct Graph(ObjectSubclass<GraphImpl>)
         @extends Misc, Widget, @implements Buildable;
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
        let distance = 4.0 * self.screen().unwrap().resolution() / 96.0;

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
