fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon(".\\img.ico")
           .set("FileVersion", "1.0.0.0")
           .set("ProductName", "Binary Info")
           .set("CompanyName", "Victor :/")
           .set("FileDescription", ":/");
            // Add more metadata as needed

        if let Err(e) = res.compile() {
            panic!("Failed to compile resources: {}", e);
        }
    }
}
