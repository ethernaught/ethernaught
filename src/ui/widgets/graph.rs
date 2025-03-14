use std::cell::RefCell;
use std::cmp::max;
use gtk::gdk::{EventMask, WindowAttr, WindowType, WindowWindowClass, RGBA};
use gtk::{gdk, glib, Allocation, Buildable, Misc, StateFlags, Widget};
use gtk::cairo::Context;
use gtk::glib::Propagation;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{StyleContextExt, WidgetExt};
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl};

#[derive(Default)]
pub struct GraphImpl {
    points: RefCell<Vec<u32>>
}

impl GraphImpl {

    fn calculate_size(&self) -> (i32, i32) {

        (100 as i32, 100 as i32)
    }
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

        cr.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());

        let min = *self.points.borrow().iter().min().unwrap();
        let max = *self.points.borrow().iter().max().unwrap();

        println!("A {:?}  {}  {}", allocation, min, max);



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

        let (calculated_width, calculated_height) = self.calculate_size();

        let width = max(calculated_width, allocation.width());
        let height = max(calculated_height, allocation.height());

        widget.set_size_request(width, height);
        self.size_allocate(&Allocation::new(allocation.x(), allocation.y(), width, height));
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
    }

    pub fn get_points(&self) -> Vec<u32> {
        self.imp().points.borrow().clone()
    }
}
