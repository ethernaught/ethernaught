use std::cell::RefCell;
use std::rc::Rc;
use gtk::{gdk, Builder, Button, CellRendererText, Container, CssProvider, Entry, Image, Label, ListBox, ListStore, ScrolledWindow, StyleContext, TreeView, TreeViewColumn};
use gtk::glib::{ObjectExt, Type};
use gtk::prelude::{AdjustmentExt, BuilderExtManual, CellLayoutExt, ContainerExt, CssProviderExt, LabelExt, ListBoxExt, ScrolledWindowExt, TreeModelExt, TreeViewColumnExt, TreeViewExt, WidgetExt};

#[derive(Clone)]
pub struct PacketsView {
    pub root: gtk::Box,
    pub search: Entry,
    pub scroll_layout: ScrolledWindow,
    pub tree_view: TreeView
}

impl PacketsView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packets_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in packet_view.ui");

        let search: Entry = builder
            .object("search")
            .expect("Couldn't find 'search' in packet_view.ui");

        let scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in packet_view.ui");

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in packet_view.ui");
        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);

        tree_view.set_model(Some(&model));

        init_column(&tree_view, "No.", 0, 100);
        init_column(&tree_view, "Time", 1, 150);
        init_column(&tree_view, "Source", 2, 180);
        init_column(&tree_view, "Destination", 3, 180);
        init_column(&tree_view, "Protocol", 4, 80);
        init_column(&tree_view, "Length", 5, 80);
        init_column(&tree_view, "Info", 6, 80);



        let vadj = Rc::new(scroll_layout.vadjustment());
        let needs_scroll = Rc::new(RefCell::new(false));
        let user_scrolled_up = Rc::new(RefCell::new(false));

        {
            let vadj = vadj.clone();
            let user_scrolled_up = user_scrolled_up.clone();
            vadj.connect_value_changed(move |adj| {
                let is_at_bottom = (adj.upper() - adj.value() - adj.page_size()).abs() < 100.0;
                *user_scrolled_up.borrow_mut() = !is_at_bottom;
            });
        }

        model.connect_row_inserted({
            let vadj = vadj.clone();
            let needs_scroll = needs_scroll.clone();
            let user_scrolled_up = user_scrolled_up.clone();

            move |_, _, _| {
                if !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = true;
                }

                let vadj = vadj.clone();
                let needs_scroll = needs_scroll.clone();
                let user_scrolled_up = user_scrolled_up.clone();

                if *needs_scroll.borrow() && !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = false;
                    vadj.set_value(vadj.upper() - vadj.page_size());
                }
            }
        });

        Self {
            root,
            search,
            scroll_layout,
            tree_view,
        }
    }
}

fn init_column(tree: &TreeView, title: &str, col_id: i32, min_width: i32) {
    let renderer = CellRendererText::new();
    let column = TreeViewColumn::new();
    column.set_min_width(min_width);
    column.set_title(title);
    CellLayoutExt::pack_start(&column, &renderer, true);
    CellLayoutExt::add_attribute(&column, &renderer, "text", col_id);

    CellLayoutExt::set_cell_data_func(&column, &renderer, Some(Box::new(move |_, cell, model, iter| {
        let protocol: String = model.value(iter, 4).get().unwrap_or_default();

        let color = match protocol.as_str() {
            "ARP" => {
                "#05211b"
            }
            "Broadcast" => {
                "#000000"
            }
            "TCP" => {
                "#1e0926"
            }
            "UDP" => {
                "#070c1f"
            }
            "ICMP" => {
                "#260d07"
            }
            "GRE" => {
                "#122407"
            }
            _ => {
                "#1e1f22"
            }
        };

        cell.set_property("cell-background", &color);
    })));

    tree.append_column(&column);
}
