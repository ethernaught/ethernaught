use std::any::Any;
use gtk::{gdk, glib, Builder, ComboBoxText, Container, CssProvider, ListBox, ListBoxRow, Paned, Stack, StyleContext};
use gtk::gdk::RGBA;
use gtk::glib::{Cast, PropertyGet, Receiver, Sender};
use gtk::prelude::{BuilderExtManual, ComboBoxExt, ComboBoxExtManual, ComboBoxTextExt, ContainerExt, CssProviderExt, PanedExt, WidgetExt};
use crate::ui::activity::inter::activity::Activity;
use crate::ui::context::Context;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::widgets::hex_editor::HexEditor;

#[derive(Clone)]
pub struct PacketPlaygroundActivity {
    context: Context,
    root: Option<Container>
}

impl PacketPlaygroundActivity {

    pub fn new(context: Context) -> Self {
        Self {
            context,
            root: None
        }
    }
}

impl Activity for PacketPlaygroundActivity {

    fn get_name(&self) -> String {
        "packet_playground_activity".to_string()
    }

    fn get_title(&self) -> String {
        "PacketPlaygroundActivity".to_string()
    }

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_activity.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        self.root = Some(builder
            .object("packet_playground_activity_layout")
            .expect("Couldn't find 'packet_playground_activity_layout' in packet_playground_window.ui"));



        let window_content: Paned = builder
            .object("window_content")
            .expect("Couldn't find 'window_content' in packet_playground_window.ui");

        let hex_scroll_layout: Container = builder
            .object("hex_scroll_layout")
            .expect("Couldn't find 'hex_scroll_layout' in packet_playground_window.ui");

        window_content.set_child_shrink(&hex_scroll_layout, false);
        window_content.set_child_resize(&hex_scroll_layout, true);

        let selection_scroll_layout: Container = builder
            .object("selection_scroll_layout")
            .expect("Couldn't find 'selection_scroll_layout' in packet_playground_window.ui");

        window_content.set_child_shrink(&selection_scroll_layout, false);







        let hex_data: Vec<u8> = vec![0xe6, 0x38, 0x83, 0x2e, 0xf3, 0x2, 0xf0, 0x77, 0xc3, 0xbe, 0xd0, 0x70, 0x8, 0x0, 0x45, 0x0, 0x0, 0x48, 0x10, 0x1c, 0x0, 0x0, 0x40, 0x11, 0x3d, 0xf8, 0xa, 0x1, 0xc, 0x8f, 0xa, 0x1, 0xc, 0x1, 0x81, 0xf9, 0x0, 0x35, 0x0, 0x34, 0x2c, 0xd7, 0x39, 0xe9, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x73, 0x73, 0x6c, 0x7, 0x67, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x41, 0x0, 0x1, 0x0, 0x0, 0x29, 0x5, 0xc0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

        let hex_editor: HexEditor = builder
            .object("hex_editor")
            .expect("Couldn't find 'hex_editor' in packet_playground_window.ui");

        hex_editor.set_data(hex_data);
        hex_editor.set_hexpand(true);
        hex_editor.set_vexpand(true);
        hex_editor.set_line_number_color(RGBA::new(0.286, 0.306, 0.341, 1.0));
        hex_editor.set_cursor_color(RGBA::new(0.608, 0.616, 0.624, 1.0));
        hex_editor.set_selection_color(RGBA::new(0.349, 0.263, 0.431, 1.0));





        let selection_layout: Container = builder
            .object("selection_layout")
            .expect("Couldn't find 'selection_layout' in packet_playground_window.ui");

        let combo_box = ComboBoxText::new();
        combo_box.append_text("Data Link Type");
        combo_box.append_text("Ethernet");
        combo_box.set_active(Some(0));

        combo_box.connect_changed(move |combobox| {
            println!("SELECTION RECEIVED   {}", combobox.active_text().unwrap());
        });



        selection_layout.add(&combo_box);

        combo_box.show_all();



        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
    }

    fn on_pause(&self) {
    }

    fn on_destroy(&self) {
    }

    fn get_context(&self) -> &Context {
        &self.context
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Activity> {
        Box::new(self.clone())
    }
}
