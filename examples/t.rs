use xmodmap_pke::xmodmap_pke;

pub fn main() {
    unsafe {
        println!("{:#?}", xmodmap_pke());
    }
}
