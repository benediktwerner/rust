//! Static files bundled with documentation output.
//!
//! All the static files are included here for centralized access in case anything other than the
//! HTML rendering code (say, the theme checker) needs to access one of these files.
//!
//! Note about types: CSS and JavaScript files are included as `&'static str` to allow for the
//! minifier to run on them. All other files are included as `&'static [u8]` so they can be
//! directly written to a `Write` handle.

/// The file contents of the main `rustdoc.css` file, responsible for the core layout of the page.
crate static RUSTDOC_CSS: &str = include_str!("static/css/rustdoc.css");

/// The file contents of `settings.css`, responsible for the items on the settings page.
crate static SETTINGS_CSS: &str = include_str!("static/css/settings.css");

/// The file contents of the `noscript.css` file, used in case JS isn't supported or is disabled.
crate static NOSCRIPT_CSS: &str = include_str!("static/css/noscript.css");

/// The file contents of `normalize.css`, included to even out standard elements between browser
/// implementations.
crate static NORMALIZE_CSS: &str = include_str!("static/css/normalize.css");

/// The file contents of `main.js`, which contains the core JavaScript used on documentation pages,
/// including search behavior and docblock folding, among others.
crate static MAIN_JS: &str = include_str!("static/js/main.js");

/// The file contents of `search.js`, which contains the search behavior.
crate static SEARCH_JS: &str = include_str!("static/js/search.js");

/// The file contents of `settings.js`, which contains the JavaScript used to handle the settings
/// page.
crate static SETTINGS_JS: &str = include_str!("static/js/settings.js");

/// The file contents of `storage.js`, which contains functionality related to browser Local
/// Storage, used to store documentation settings.
crate static STORAGE_JS: &str = include_str!("static/js/storage.js");

/// The file contents of `scraped-examples.js`, which contains functionality related to the
/// --scrape-examples flag that inserts automatically-found examples of usages of items.
crate static SCRAPE_EXAMPLES_JS: &str = include_str!("static/js/scrape-examples.js");

crate static SCRAPE_EXAMPLES_HELP_MD: &str = include_str!("static/scrape-examples-help.md");

/// The file contents of `wheel.svg`, the icon used for the settings button.
crate static WHEEL_SVG: &[u8] = include_bytes!("static/images/wheel.svg");

/// The file contents of `clipboard.svg`, the icon used for the "copy path" button.
crate static CLIPBOARD_SVG: &[u8] = include_bytes!("static/images/clipboard.svg");

/// The file contents of `down-arrow.svg`, the icon used for the crate choice combobox.
crate static DOWN_ARROW_SVG: &[u8] = include_bytes!("static/images/down-arrow.svg");

/// The file contents of `toggle-minus.svg`, the icon used for opened toggles.
crate static TOGGLE_MINUS_PNG: &[u8] = include_bytes!("static/images/toggle-minus.svg");

/// The file contents of `toggle-plus.svg`, the icon used for closed toggles.
crate static TOGGLE_PLUS_PNG: &[u8] = include_bytes!("static/images/toggle-plus.svg");

/// The contents of `COPYRIGHT.txt`, the license listing for files distributed with documentation
/// output.
crate static COPYRIGHT: &[u8] = include_bytes!("static/COPYRIGHT.txt");

/// The contents of `LICENSE-APACHE.txt`, the text of the Apache License, version 2.0.
crate static LICENSE_APACHE: &[u8] = include_bytes!("static/LICENSE-APACHE.txt");

/// The contents of `LICENSE-MIT.txt`, the text of the MIT License.
crate static LICENSE_MIT: &[u8] = include_bytes!("static/LICENSE-MIT.txt");

/// The contents of `rust-logo.svg`, the default icon of the documentation.
crate static RUST_LOGO_SVG: &[u8] = include_bytes!("static/images/rust-logo.svg");

/// The default documentation favicons (SVG and PNG fallbacks)
crate static RUST_FAVICON_SVG: &[u8] = include_bytes!("static/images/favicon.svg");
crate static RUST_FAVICON_PNG_16: &[u8] = include_bytes!("static/images/favicon-16x16.png");
crate static RUST_FAVICON_PNG_32: &[u8] = include_bytes!("static/images/favicon-32x32.png");

/// The built-in themes given to every documentation site.
crate mod themes {
    /// The "light" theme, selected by default when no setting is available. Used as the basis for
    /// the `--check-theme` functionality.
    crate static LIGHT: &str = include_str!("static/css/themes/light.css");

    /// The "dark" theme.
    crate static DARK: &str = include_str!("static/css/themes/dark.css");

    /// The "ayu" theme.
    crate static AYU: &str = include_str!("static/css/themes/ayu.css");
}

/// Files related to the Fira Sans font.
crate mod fira_sans {
    /// The file `FiraSans-Regular.woff2`, the Regular variant of the Fira Sans font in woff2.
    crate static REGULAR: &[u8] = include_bytes!("static/fonts/FiraSans-Regular.woff2");

    /// The file `FiraSans-Medium.woff2`, the Medium variant of the Fira Sans font in woff2.
    crate static MEDIUM: &[u8] = include_bytes!("static/fonts/FiraSans-Medium.woff2");

    /// The file `FiraSans-LICENSE.txt`, the license text for the Fira Sans font.
    crate static LICENSE: &[u8] = include_bytes!("static/fonts/FiraSans-LICENSE.txt");
}

/// Files related to the Source Serif 4 font.
crate mod source_serif_4 {
    /// The file `SourceSerif4-Regular.ttf.woff2`, the Regular variant of the Source Serif 4 font in
    /// woff2.
    crate static REGULAR: &[u8] = include_bytes!("static/fonts/SourceSerif4-Regular.ttf.woff2");

    /// The file `SourceSerif4-Bold.ttf.woff2`, the Bold variant of the Source Serif 4 font in
    /// woff2.
    crate static BOLD: &[u8] = include_bytes!("static/fonts/SourceSerif4-Bold.ttf.woff2");

    /// The file `SourceSerif4-It.ttf.woff2`, the Italic variant of the Source Serif 4 font in
    /// woff2.
    crate static ITALIC: &[u8] = include_bytes!("static/fonts/SourceSerif4-It.ttf.woff2");

    /// The file `SourceSerif4-LICENSE.txt`, the license text for the Source Serif 4 font.
    crate static LICENSE: &[u8] = include_bytes!("static/fonts/SourceSerif4-LICENSE.md");
}

/// Files related to the Source Code Pro font.
crate mod source_code_pro {
    /// The file `SourceCodePro-Regular.ttf.woff2`, the Regular variant of the Source Code Pro font
    /// in woff2.
    crate static REGULAR: &[u8] = include_bytes!("static/fonts/SourceCodePro-Regular.ttf.woff2");

    /// The file `SourceCodePro-Semibold.ttf.woff2`, the Semibold variant of the Source Code Pro
    /// font in woff2.
    crate static SEMIBOLD: &[u8] = include_bytes!("static/fonts/SourceCodePro-Semibold.ttf.woff2");

    /// The file `SourceCodePro-It.ttf.woff2`, the Italic variant of the Source Code Pro font in
    /// woff2.
    crate static ITALIC: &[u8] = include_bytes!("static/fonts/SourceCodePro-It.ttf.woff2");

    /// The file `SourceCodePro-LICENSE.txt`, the license text of the Source Code Pro font.
    crate static LICENSE: &[u8] = include_bytes!("static/fonts/SourceCodePro-LICENSE.txt");
}

/// Files related to the Nanum Barun Gothic font.
///
/// These files are used to avoid some legacy CJK serif fonts in Windows.
///
/// Note that the Noto Sans KR font, which was used previously but was not very readable on Windows,
/// has been replaced by the Nanum Barun Gothic font. This is due to Windows' implementation of font
/// rendering that distorts OpenType fonts too much.
///
/// The font files were generated with these commands:
///
/// ```sh
/// pyftsubset NanumBarunGothic.ttf \
/// --unicodes=U+AC00-D7AF,U+1100-11FF,U+3130-318F,U+A960-A97F,U+D7B0-D7FF \
/// --output-file=NanumBarunGothic.ttf.woff2 --flavor=woff2
/// ```
crate mod nanum_barun_gothic {
    /// The file `NanumBarunGothic.ttf.woff2`, the Regular variant of the Nanum Barun Gothic font.
    crate static REGULAR: &[u8] = include_bytes!("static/fonts/NanumBarunGothic.ttf.woff2");

    /// The file `NanumBarunGothic-LICENSE.txt`, the license text of the Nanum Barun Gothic font.
    crate static LICENSE: &[u8] = include_bytes!("static/fonts/NanumBarunGothic-LICENSE.txt");
}

/// Files related to the sidebar in rustdoc sources.
crate mod sidebar {
    /// File script to handle sidebar.
    crate static SOURCE_SCRIPT: &str = include_str!("static/js/source-script.js");
}
