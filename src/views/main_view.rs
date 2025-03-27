use gtk::Builder;

pub struct MainView {

}

impl MainView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/main_activity.ui");

        Self {

        }
    }
}
