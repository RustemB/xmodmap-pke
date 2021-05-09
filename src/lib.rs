use std::collections::HashMap;
use std::ffi::CStr;
use x11::xlib::{
    Display, KeyCode, KeySym, XCloseDisplay, XDisplayKeycodes, XFree, XGetKeyboardMapping,
    XKeysymToString, XOpenDisplay,
};

pub type XmodmapPke = HashMap<u8, Vec<String>>;

/// Return `HashMap` with `KeyCode` as key and vector of `KeySyms` as value
/// (This block of code was taken from original `xmodmap` source code)
pub fn xmodmap_pke() -> Result<XmodmapPke, String> {
    let dpy: *mut Display = unsafe { XOpenDisplay(std::ptr::null::<libc::c_char>()) };
    let mut xmodmap_pke_table: XmodmapPke = HashMap::new();
    let mut min_keycode = 0;
    let mut max_keycode = 0;
    let mut keysyms_per_keycode = 0;
    let mut keymap: *mut KeySym;
    let origkeymap: *mut KeySym;
    unsafe {
        XDisplayKeycodes(dpy, &mut min_keycode, &mut max_keycode);
    }
    origkeymap = unsafe {
        XGetKeyboardMapping(
            dpy,
            min_keycode as KeyCode,
            max_keycode - min_keycode + 1_i32,
            &mut keysyms_per_keycode,
        )
    };
    if origkeymap.is_null() {
        return Err("unable to get keyboard mapping table.".to_string());
    }
    keymap = origkeymap;
    for i in min_keycode..=max_keycode {
        let mut max = keysyms_per_keycode - 1;
        while max >= 0 && unsafe { *keymap.offset(max as isize) } == 0_u64 {
            max -= 1
        }
        let mut ksyms: Vec<String> = Vec::new();
        for j in 0..=max {
            let ks: KeySym = unsafe { *keymap.offset(j as isize) };
            let s = if ks == 0_u64 {
                "NoSymbol"
            } else {
                unsafe { CStr::from_ptr(XKeysymToString(ks)) }
                .to_str()
                    .unwrap()
            };
            if s.is_empty() {
                ksyms.push(format!("{:0>4x}", ks));
            } else {
                ksyms.push(s.to_string());
            }
        }
        keymap = unsafe { keymap.offset(keysyms_per_keycode as isize) };
        xmodmap_pke_table.insert(i as u8, ksyms);
    }
    unsafe {
        XFree(origkeymap.cast::<i8>().cast::<libc::c_void>());
        XCloseDisplay(dpy);
    }
    Ok(xmodmap_pke_table)
}
