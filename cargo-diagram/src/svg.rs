use std::path::Path;

fn with_svg_options<T>(callback: impl FnOnce(usvg::OptionsRef<'_>) -> T) -> T {
    let mut options = usvg::Options::default();
    options.fontdb.load_system_fonts();
    let options_ref = options.to_ref();

    callback(options_ref)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_path(path: &std::path::Path) -> Result<usvg::Tree, std::io::Error> {
    let svg_data = std::fs::read(path)?;

    with_svg_options(|options| {
        usvg::Tree::from_data(&svg_data, &options)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    })
}

pub fn load_from_data(slice: &[u8]) -> Result<usvg::Tree, usvg::Error> {
    with_svg_options(|options| usvg::Tree::from_data(slice, &options))
}

pub fn svg_to_png(input: &str, output: &str) {
    let tree = load_from_path(Path::new(input)).unwrap();
    let pixmap_size = tree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.save_png(output).unwrap();
}


