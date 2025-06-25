use iced::{font, Font};

pub fn load_fonts() -> Vec<Font> {
    vec![
        Font::with_name("Noto Sans JP"),
        // フォントファイルから直接読み込み（将来の実装用）
        // Font::External {
        //     name: "Noto Sans JP".into(),
        //     bytes: include_bytes!("../fonts/NotoSansJP-Regular.ttf").as_slice().into(),
        // },
    ]
}

pub fn default_font() -> Font {
    Font::with_name("Noto Sans JP")
}

// フォント設定のヘルパー関数
pub fn setup_fonts() -> Vec<u8> {
    // フォントファイルを埋め込み
    include_bytes!("../fonts/NotoSansJP-Regular.ttf").to_vec()
}