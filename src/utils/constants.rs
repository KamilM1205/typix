pub const GOLOS_BOLD: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/Golos-Text_Bold.ttf"
));
pub const GOLOS_REGULAR: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/Golos-Text_Regular.ttf"
));

pub const ICON_SMALL: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/icons/icon_small.png"
));

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CONFIGS: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets/configs");

pub const SAMPLES: &[i32] = &[0, 2, 4, 8];
