use std::cell::RefCell;
use std::cmp::max;
use gtk::gdk::{EventMask, WindowAttr, WindowType, WindowWindowClass, RGBA};
use gtk::{gdk, glib, pango, Allocation, Buildable, Misc, StateFlags, Widget};
use gtk::cairo::{Context, FontSlant, FontWeight};
use gtk::glib::Propagation;
use gtk::glib::Propagation::Proceed;
use gtk::pango::Weight;
use gtk::prelude::{StyleContextExt, StyleContextExtManual, WidgetExt};
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl};

pub struct TerminalImpl {
    cursor: RefCell<Option<usize>>,
    selection: RefCell<Option<(usize, usize)>>,
    cursor_color: RefCell<RGBA>,
    selection_color: RefCell<RGBA>
}

impl Default for TerminalImpl {

    fn default() -> Self {
        Self {
            cursor: RefCell::new(None),
            selection: RefCell::new(None),
            cursor_color: RefCell::new(RGBA::new(0.8, 0.8, 0.8, 1.0)),
            selection_color: RefCell::new(RGBA::new(0.4, 0.0, 0.4, 1.0))
        }
    }
}

impl TerminalImpl {

    fn calculate_size(&self) -> (i32, i32) {

        (100 as i32, 100 as i32)
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalImpl {

    const NAME: &'static str = "Terminal";
    type ParentType = Widget;
    type Type = Terminal;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("terminal");
    }
}

impl ObjectImpl for TerminalImpl {}

impl WidgetImpl for TerminalImpl {

    fn draw(&self, cr: &Context) -> Propagation {
        let widget = self.obj();
        let style_context = widget.style_context();
        let pango_context = widget.create_pango_context();

        style_context.set_state(StateFlags::NORMAL);
        let text_color = style_context.color(StateFlags::NORMAL);

        let font_desc = style_context.font(StateFlags::NORMAL);
        let metrics = pango_context.metrics(Some(&font_desc), None);

        let ascent = metrics.ascent() as f64 / pango::SCALE as f64;
        let decent = metrics.descent() as f64 / pango::SCALE as f64;

        let font_size = if font_desc.is_size_absolute() {
            font_desc.size() as f64
        } else {
            font_desc.size() as f64 / pango::SCALE as f64
        };

        let font_weight = match font_desc.weight() {
            Weight::Bold => FontWeight::Bold,
            _ => {
                FontWeight::Normal
            }
        };

        let padding = style_context.padding(StateFlags::NORMAL);

        if let Ok(background) = style_context.style_property_for_state("background-color", StateFlags::NORMAL).get::<RGBA>() {
            cr.set_source_rgba(background.red(), background.green(), background.blue(), background.alpha());
            cr.paint();
        }

        let char_width = metrics.approximate_char_width() as f64 / pango::SCALE as f64;
        let row_height = ascent + decent;


        cr.select_font_face(font_desc.family().unwrap().as_str(), FontSlant::Normal, font_weight);
        cr.set_font_size(font_size);



        cr.set_source_rgba(text_color.red(), text_color.green(), text_color.blue(), text_color.alpha());
        let data = "user@linux:~$ echo 'Hello, World!'";
        cr.move_to(padding.left as f64, row_height);
        cr.show_text(&data);




        Proceed
    }

    fn realize(&self) {
        let widget = self.obj();
        let allocation = widget.allocation();

        let attr = WindowAttr {
            title: None,
            event_mask: EventMask::KEY_PRESS_MASK,
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




        widget.connect_key_press_event(move |_, event| {
            println!("{:?}", event);

            Proceed
        });

    }
}

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<TerminalImpl>)
        @extends Misc, Widget, @implements Buildable;
}

impl Terminal {

    pub fn new() -> Self {
        let _self = glib::Object::builder::<Terminal>().build();
        _self
    }

    pub fn set_cursor_color(&self, color: RGBA) {
        *self.imp().cursor_color.borrow_mut() = color;
    }

    pub fn get_cursor_color(&self) -> RGBA {
        self.imp().cursor_color.borrow().clone()
    }

    pub fn set_selection_color(&self, color: RGBA) {
        *self.imp().selection_color.borrow_mut() = color;
    }

    pub fn get_selection_color(&self) -> RGBA {
        self.imp().selection_color.borrow().clone()
    }

    pub fn set_selection(&self, a: usize, b: usize) {
        *self.imp().selection.borrow_mut() = Some((a, b));
        self.queue_draw();
    }

    pub fn get_selection(&self) -> Option<(usize, usize)> {
        self.imp().selection.borrow().clone()
    }
}
