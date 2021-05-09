use x11::xlib::{XCloseDisplay, XOpenDisplay};
use xmodmap_pke::xmodmap_pke;

pub fn main() {
    unsafe {
        let dpy = XOpenDisplay(std::ptr::null::<libc::c_char>());
        println!("{:#?}", xmodmap_pke(dpy));
        XCloseDisplay(dpy);
    }
}
