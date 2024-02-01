fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/d20.ico");
        res.compile().expect("Could not attach exe icon");
    }
}