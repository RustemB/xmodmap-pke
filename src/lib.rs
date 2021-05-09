use std::collections::HashMap;
use std::ffi::CStr;
use x11::xlib::{
    Display, KeyCode, KeySym, XDisplayKeycodes, XFree, XGetKeyboardMapping, XKeysymToString,
};

pub type XmodmapPke = HashMap<u8, Vec<String>>;

/// Return HashMap with KeyCode as key and vector of KeySyms as value
/// (This block of code was taken from original `xmodmap` source code)
pub unsafe fn xmodmap_pke(dpy: *mut Display) -> Result<XmodmapPke, String> {
    let mut xmodmap_pke_table: XmodmapPke = HashMap::new();
    let mut min_keycode = 0;
    let mut max_keycode = 0;
    let mut keysyms_per_keycode = 0;
    let mut keymap: *mut KeySym;
    let origkeymap: *mut KeySym;
    XDisplayKeycodes(dpy, &mut min_keycode, &mut max_keycode);
    origkeymap = XGetKeyboardMapping(
        dpy,
        min_keycode as KeyCode,
        max_keycode - min_keycode + 1_i32,
        &mut keysyms_per_keycode,
    );
    if origkeymap.is_null() {
        return Err("unable to get keyboard mapping table.".to_string());
    }
    keymap = origkeymap;
    for i in min_keycode..=max_keycode {
        let mut max = keysyms_per_keycode - 1;
        while max >= 0 && *keymap.offset(max as isize) == 0_u64 {
            max -= 1
        }
        let mut ksyms: Vec<String> = Vec::new();
        for j in 0..=max {
            let ks: KeySym = *keymap.offset(j as isize);
            let s = if ks != 0_u64 {
                CStr::from_ptr(XKeysymToString(ks)).to_str().unwrap()
            } else {
                "NoSymbol"
            };
            if !s.is_empty() {
                ksyms.push(s.to_string());
            } else {
                ksyms.push(format!("{:0>4x}", ks));
            }
        }
        keymap = keymap.offset(keysyms_per_keycode as isize);
        xmodmap_pke_table.insert(i as u8, ksyms);
    }
    XFree(origkeymap as *mut libc::c_char as *mut libc::c_void);
    Ok(xmodmap_pke_table)
}
