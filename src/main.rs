#![debugger_visualizer(natvis_file = "Spike.natvis")]

#[cfg_attr(not(doc), repr(transparent))]
#[derive(Debug)]
pub struct Spike {
    inner: str,
}
impl Spike {
    fn new(s: &str) -> &Self {
        unsafe { &*(core::ptr::from_ref::<str>(s) as *const Self) }
    }
}

fn main() {
    let spike = Spike::new("example");
    let path = std::path::Path::new("example");
    println!("{path:?}\n{spike:?}");
}
