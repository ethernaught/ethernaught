mod pcap_ext;
mod database;
mod utils;
mod bus;
mod sniffer;

use std::path::PathBuf;

#[cfg(feature = "gtk3")]
mod gtk3;
#[cfg(feature = "gtk3")]
use crate::gtk3::app::App;

#[cfg(feature = "gtk4")]
mod gtk4;
#[cfg(feature = "gtk4")]
use crate::gtk4::app::App;

//export GTK_DEBUG=interactive

//glib-compile-resources res/gtk4/linux.gresources.xml --target=res/resources.gresources

/*
rustup install nightly
rustup override set nightly
*/

//re-implement dropdown replay...
//save fragments so we can clear adapter
//redo dropdown - doesnt look quite right on drop downs...

//FIND A BETTER METHOD WITHIN PCAP LIB TO SEND ADDRESS DETAILS AS IT WOND BE THE SAME STRUCT PER OS
// - not to mention we will need the data when saving, so might want to add to the packet - MAINLY FOR PROMISCUOUS MODE...

//MacOS Font goes to /Library/fonts

//Setting to change programming language of choice...

// - fix time in list

//look into switching from CSS for handling all icons...

//MAKE GTK3 and GTK4 not use global stylesheets but rather per view add styles...

//do we need refcell to store children for GTK3 overlay or can we take from widget...

//re-add the min-height for the damn menubar on GTK3 and add it for GTk4

// NO TRANSIENT PARENT...

fn main() {
    //unsafe { env::set_var("GTK_THEME", "Adwaita:dark") };

    let app = App::new();
    app.run();
}

//CAN WE CHANGE THIS TO A VARIABLE SET ON BUILD...?
pub fn get_lib_path(file_name: &str) -> PathBuf {
    if cfg!(debug_assertions) {
        return PathBuf::from(file_name);
    }

    PathBuf::from(format!("/usr/var/lib/ethernaught/{}", file_name))
}
/*
fn is_root() -> bool {
    match env::var("USER") {
        Ok(user) => user == "root",
        Err(_) => false,
    }
}
*/
