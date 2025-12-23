use wxdragon::prelude::*;

#[cfg(not(target_os = "linux"))]
const WELCOME_TEXT: &str = "Drag and drop your .ipa here";
#[cfg(target_os = "linux")]
const WELCOME_TEXT: &str = "Press 'import' and select an .ipa to get started";

#[derive(Clone)]
pub struct DefaultPage {
    pub panel: Panel,
}

impl DefaultPage {
    #[cfg(not(target_os = "linux"))]
    fn is_allowed_file(path: &str) -> bool {
        path.ends_with(".ipa") || path.ends_with(".tipa")
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_file_handlers(&self, on_drop: impl Fn(String) + 'static) {
        _ = FileDropTarget::builder(&self.panel)
            .with_on_drop_files(move |files, _, _| {
                if files.len() != 1 || !DefaultPage::is_allowed_file(&files[0]) {
                    return false;
                }
                on_drop(files[0].clone());
                true
            })
            .with_on_drag_over(move |_, _, _| DragResult::Move)
            .with_on_enter(move |_, _, _| DragResult::Move)
            .build();
    }
}

pub fn create_default_page(frame: &Frame) -> DefaultPage {
    let panel = Panel::builder(frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    sizer.add_stretch_spacer(1);

    let welcome_text = StaticText::builder(&panel)
        .with_label(WELCOME_TEXT)
        .with_style(StaticTextStyle::AlignCenterHorizontal)
        .build();

    sizer.add(
        &welcome_text,
        0,
        SizerFlag::AlignCenterHorizontal | SizerFlag::All,
        0,
    );

    sizer.add_stretch_spacer(1);

    let love_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let made_with_text = StaticText::builder(&panel)
        .with_label("Made with 💖 from ")
        .build();
    let khcrysalis_link = HyperlinkCtrl::builder(&panel)
        .with_label("SAMSAM")
        .with_url("https://github.com/khcrysalis")
        .with_style(HyperlinkCtrlStyle::AlignLeft | HyperlinkCtrlStyle::NoUnderline)
        .build();
    let separator1 = StaticText::builder(&panel).with_label(" • ").build();
    let github_link = HyperlinkCtrl::builder(&panel)
        .with_label("GitHub")
        .with_url("https://github.com/khcrysalis/plumeimpactor")
        .with_style(HyperlinkCtrlStyle::AlignLeft | HyperlinkCtrlStyle::NoUnderline)
        .build();
    let separator2 = StaticText::builder(&panel).with_label(" • ").build();
    let donate_link = HyperlinkCtrl::builder(&panel)
        .with_label("Donate")
        .with_url("https://github.com/sponsors/khcrysalis")
        .with_style(HyperlinkCtrlStyle::AlignLeft | HyperlinkCtrlStyle::NoUnderline)
        .build();
    love_sizer.add(&made_with_text, 0, SizerFlag::AlignCenterVertical, 0);
    love_sizer.add(&khcrysalis_link, 0, SizerFlag::AlignCenterVertical, 0);
    love_sizer.add(&separator1, 0, SizerFlag::AlignCenterVertical, 0);
    love_sizer.add(&github_link, 0, SizerFlag::AlignCenterVertical, 0);
    love_sizer.add(&separator2, 0, SizerFlag::AlignCenterVertical, 0);
    love_sizer.add(&donate_link, 0, SizerFlag::AlignCenterVertical, 0);

    sizer.add_sizer(&love_sizer, 0, SizerFlag::Left | SizerFlag::All, 15);

    panel.set_sizer(sizer, true);

    DefaultPage { panel }
}
