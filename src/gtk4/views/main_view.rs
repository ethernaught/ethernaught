use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use gtk4::{gdk, style_context_add_provider_for_display, Builder, Button, CssProvider, Paned, StyleContext, Widget};
use gtk4::gio::SimpleAction;
use gtk4::glib::property::PropertyGet;
use gtk4::prelude::{ActionMapExt, Cast, StyleContextExt, WidgetExt};
use rlibpcap::devices::Device;
use rlibpcap::pcap::pcap::Pcap;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use crate::bus::event_bus::{pause_event, register_event, resume_event, unregister_event};
use crate::bus::event_bus::EventPropagation::Continue;
use crate::bus::events::capture_event::CaptureEvent;
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::views::packets_view::PacketsView;
use crate::gtk4::windows::main_window::MainWindow;

pub struct MainView {
    pub show_capture_bar: Option<Rc<RefCell<dyn Fn(bool)>>>,
    pub show_title_bar: Box<dyn Fn(bool)>,
    pub root: gtk4::Box,
    pub activity_pane: Paned,
    pub content_pane: Paned,
    pub packets: PacketsView,
    //pub sidebar: Rc<RefCell<Option<SidebarView>>>,
    //pub terminal: Rc<RefCell<Option<TerminalView>>>,
    pub event_listener: Option<RefCell<u32>>
}

impl MainView {

    pub fn new(window: &MainWindow) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/main_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        //activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        //activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);


        let terminal_button: Button = builder
            .object("terminal")
            .expect("Couldn't find 'terminal' in main_view.ui");
        /*let terminal = Rc::new(RefCell::new(None::<TerminalView>));

        terminal_button.connect_button_press_event({
            let activity_pane = activity_pane.clone();
            let terminal = terminal.clone();
            move |button, _| {
                let mut term_ref = terminal.borrow_mut();

                if let Some(terminal) = term_ref.as_ref() {
                    button.style_context().remove_class("selected");
                    activity_pane.remove(&terminal.root);
                    *term_ref = None::<TerminalView>;
                    return Proceed;
                }

                button.style_context().add_class("selected");
                let view = TerminalView::new();
                activity_pane.add(&view.root);
                *term_ref = Some(view);

                Proceed
            }
        });*/


        let show_title_bar = Box::new(show_title_bar(window, "Any", DataLinkTypes::Null));

        /*let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let actions = SimpleActionGroup::new();

        root.insert_action_group("dialog", Some(&actions));

        let action = SimpleAction::new("dismiss", None);
        action.connect_activate({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |_, _| {
                let view = sidebar.borrow().as_ref().map(|view| view.root.clone());

                if let Some(view) = view {
                    content_pane.remove(&view);
                    *sidebar.borrow_mut() = None;
                }
            }
        });
        actions.add_action(&action);*/

        let packets = PacketsView::new();
        /*packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });*/
        content_pane.set_start_child(Some(&packets.root));
        //content_pane.append(&packets.root);

        let event_listener = Some(RefCell::new(register_event("capture_event", {
            let packets = packets.clone();
            move |event| {
                let event = event.as_any().downcast_ref::<CaptureEvent>().unwrap();
                packets.add(event.get_packet().clone());
                Continue
            }
        }, true)));

        let show_capture_bar = Rc::new(RefCell::new(show_capture_bar(&window, &packets)));

        let action = SimpleAction::new("start", None);
        action.connect_activate({
            let show_capture_bar = show_capture_bar.clone();
            let event_listener = event_listener.as_ref().unwrap().clone();
            move |_, _| {
                show_capture_bar.borrow()(true);
                resume_event("capture_event", event_listener.borrow().clone());
            }
        });
        window.window.add_action(&action);
        window.title_bar.start.show();

        let action = SimpleAction::new("stop", None);
        action.connect_activate({
            let show_capture_bar = show_capture_bar.clone();
            let event_listener = event_listener.as_ref().unwrap().clone();
            move |_, _| {
                show_capture_bar.borrow()(false);
                pause_event("capture_event", event_listener.borrow().clone());
            }
        });
        window.window.add_action(&action);

        Self {
            show_title_bar,
            show_capture_bar: Some(show_capture_bar),
            root,
            activity_pane,
            content_pane,
            packets,
            //sidebar,
            //terminal,
            event_listener
        }
    }

    /*
    pub fn from_device(window: &MainWindow, device: &Device) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/main_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);


        let terminal_button: Button = builder
            .object("terminal")
            .expect("Couldn't find 'terminal' in main_view.ui");
        let terminal = Rc::new(RefCell::new(None::<TerminalView>));

        terminal_button.connect_button_press_event({
            let activity_pane = activity_pane.clone();
            let terminal = terminal.clone();
            move |button, _| {
                let mut term_ref = terminal.borrow_mut();

                if let Some(terminal) = term_ref.as_ref() {
                    button.style_context().remove_class("selected");
                    activity_pane.remove(&terminal.root);
                    *term_ref = None::<TerminalView>;
                    return Proceed;
                }

                button.style_context().add_class("selected");
                let view = TerminalView::new();
                activity_pane.add(&view.root);
                *term_ref = Some(view);

                Proceed
            }
        });


        let show_title_bar = Box::new(show_title_bar(window, &device.get_name(), device.get_data_link_type()));

        let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let actions = SimpleActionGroup::new();

        root.insert_action_group("dialog", Some(&actions));

        let action = SimpleAction::new("dismiss", None);
        action.connect_activate({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |_, _| {
                let view = sidebar.borrow().as_ref().map(|view| view.root.clone());

                if let Some(view) = view {
                    content_pane.remove(&view);
                    *sidebar.borrow_mut() = None;
                }
            }
        });
        actions.add_action(&action);

        let packets = PacketsView::new();
        packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });
        content_pane.add(&packets.root);

        let event_listener = Some(RefCell::new(register_event("capture_event", {
            let if_index = device.get_index();
            let packets = packets.clone();
            move |event| {
                let event = event.as_any().downcast_ref::<CaptureEvent>().unwrap();

                if event.get_if_index() == if_index {
                    packets.add(event.get_packet().clone());
                }
                Continue
            }
        }, true)));

        let show_capture_bar = Rc::new(RefCell::new(show_capture_bar(&window, &packets)));

        let action = SimpleAction::new("start", None);
        action.connect_activate({
            let show_capture_bar = show_capture_bar.clone();
            let event_listener = event_listener.as_ref().unwrap().clone();
            move |_, _| {
                show_capture_bar.borrow()(true);
                resume_event("capture_event", event_listener.borrow().clone());
            }
        });
        window.window.add_action(&action);
        window.title_bar.start.show();


        let action = SimpleAction::new("stop", None);
        action.connect_activate({
            let show_capture_bar = show_capture_bar.clone();
            let event_listener = event_listener.as_ref().unwrap().clone();
            move |_, _| {
                show_capture_bar.borrow()(false);
                pause_event("capture_event", event_listener.borrow().clone());
            }
        });
        window.window.add_action(&action);

        Self {
            show_title_bar,
            show_capture_bar: Some(show_capture_bar),
            root,
            activity_pane,
            content_pane,
            packets,
            sidebar,
            terminal,
            event_listener
        }
    }

    pub fn from_pcap(window: &MainWindow, path: &PathBuf) -> Self {
        let pcap = Pcap::from_file(path.to_str().unwrap()).expect("Couldn't parse pcap");

        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/main_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);


        let terminal_button: Button = builder
            .object("terminal")
            .expect("Couldn't find 'terminal' in main_view.ui");
        let terminal = Rc::new(RefCell::new(None::<TerminalView>));

        terminal_button.connect_button_press_event({
            let activity_pane = activity_pane.clone();
            let terminal = terminal.clone();
            move |button, _| {
                let mut term_ref = terminal.borrow_mut();

                if let Some(terminal) = term_ref.as_ref() {
                    button.style_context().remove_class("selected");
                    activity_pane.remove(&terminal.root);
                    *term_ref = None::<TerminalView>;
                    return Proceed;
                }

                button.style_context().add_class("selected");
                let view = TerminalView::new();
                activity_pane.add(&view.root);
                *term_ref = Some(view);

                Proceed
            }
        });


        let show_title_bar = Box::new(show_title_bar(window, path.file_name().unwrap().to_str().unwrap(), pcap.get_data_link_type()));

        let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let actions = SimpleActionGroup::new();

        root.insert_action_group("dialog", Some(&actions));

        let action = SimpleAction::new("dismiss", None);
        action.connect_activate({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |_, _| {
                let view = sidebar.borrow().as_ref().map(|view| view.root.clone());

                if let Some(view) = view {
                    content_pane.remove(&view);
                    *sidebar.borrow_mut() = None;
                }
            }
        });
        actions.add_action(&action);

        let mut packets = PacketsView::from_pcap(pcap);
        packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });
        content_pane.add(&packets.root);

        Self {
            show_title_bar,
            show_capture_bar: None,
            root,
            activity_pane,
            content_pane,
            packets,
            sidebar,
            terminal,
            event_listener: None
        }
    }*/
}

impl Stackable for MainView {

    fn get_name(&self) -> String {
        "main_view".to_string()
    }

    fn get_root(&self) -> &Widget {
        self.root.upcast_ref()
    }

    fn on_create(&self) {
        (self.show_title_bar)(true);
    }

    fn on_resume(&self) {
        (self.show_title_bar)(true);
    }

    fn on_pause(&self) {
        (self.show_title_bar)(false);

        /*if let Some(event_listener) = &self.event_listener {
            pause_event("capture_event", *event_listener.borrow());
        }

        if let Some(show_capture_bar) = &self.show_capture_bar {
            show_capture_bar.borrow()(false);
        }*/
    }

    fn on_destroy(&self) {
        /*if let Some(event_listener) = &self.event_listener {
            unregister_event("capture_event", *event_listener.borrow());
        }*/
    }
}

fn show_title_bar(window: &MainWindow, name: &str, data_link_type: DataLinkTypes) -> impl Fn(bool) {
    let title_bar = window.title_bar.clone();
    let name = name.to_string();
    move |shown| {
        if shown {
            match data_link_type {
                DataLinkTypes::Null => {
                    title_bar.root.style_context().add_class("any");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_any.svg"));
                }
                DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                    title_bar.root.style_context().add_class("ethernet");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
                }
                DataLinkTypes::Loop => {
                    title_bar.root.style_context().add_class("lan");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
                }
                DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                    title_bar.root.style_context().add_class("vpn");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
                }
                DataLinkTypes::Ieee802_11 => {
                    title_bar.root.style_context().add_class("wifi");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_wifi.svg"));
                }
                /*
                DataLinkTypes::BluetoothHciH4 => {
                    titlebar.style_context().add_class("bluetooth");
                    icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
                }
                */
                _ => {}
            }

            title_bar.network_type_label.set_label(&name);

            title_bar.network_type_icon.show();
            title_bar.network_type_label.show();

            title_bar.app_options.show();
            return;
        }

        match data_link_type {
            DataLinkTypes::Null => {
                title_bar.root.style_context().remove_class("any");
            }
            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                title_bar.root.style_context().remove_class("ethernet");
            }
            DataLinkTypes::Loop => {
                title_bar.root.style_context().remove_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                title_bar.root.style_context().remove_class("vpn");
            }
            DataLinkTypes::Ieee802_11 => {
                title_bar.root.style_context().remove_class("wifi");
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().remove_class("bluetooth");
            }
            */
            _ => {}
        }

        title_bar.network_type_icon.hide();
        title_bar.network_type_label.hide();

        title_bar.app_options.hide();
    }
}

fn show_capture_bar(window: &MainWindow, packets: &PacketsView) -> impl Fn(bool) {
    let title_bar = window.title_bar.clone();
    let packets = packets.clone();
    move |shown| {
        if shown {
            title_bar.app_options.style_context().add_class("running");
            title_bar.stop.show();
            packets.clear();
            return;
        }

        title_bar.app_options.style_context().remove_class("running");
        title_bar.stop.hide();
    }
}
