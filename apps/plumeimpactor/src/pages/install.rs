use wxdragon::prelude::*;

#[derive(Clone)]
pub struct InstallPage {
    pub panel: Panel,
    pub cancel_button: Button,
    pub install_button: Button,
    pub top_text: StaticText,
}

pub fn create_install_page(frame: &Frame) -> InstallPage {
    let panel = Panel::builder(frame).build();

    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

    let top_text = StaticText::builder(&panel)
        .with_label("Unknown")
        .build();
    main_sizer.add(&top_text, 0, SizerFlag::Left | SizerFlag::Top, 14);

    main_sizer.add_stretch_spacer(1);

    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let cancel_button = Button::builder(&panel)
        .with_label("Cancel")
        .build();

    let install_button = Button::builder(&panel)
        .with_label("Install")
        .build();

    button_sizer.add_stretch_spacer(1);
    button_sizer.add(&cancel_button, 0, SizerFlag::Right, 12);
    button_sizer.add(&install_button, 0, SizerFlag::All, 0);

    main_sizer.add_sizer(&button_sizer, 0, SizerFlag::Right | SizerFlag::Bottom | SizerFlag::Expand, 14);

    panel.set_sizer(main_sizer, true);

    InstallPage {
        panel,
        cancel_button,
        install_button,
        top_text,
    }
}

impl InstallPage {
    pub fn set_cancel_handler(&self, on_cancel: impl Fn() + 'static) {
        self.cancel_button.on_click(move |_evt| {
            on_cancel();
        });
    }
	
	pub fn set_install_handler(&self, on_install: impl Fn() + 'static) {
		self.install_button.on_click(move |_evt| {
			on_install();
		});
	}

    pub fn set_top_text(&self, text: &str) {
        self.top_text.set_label(text);
    }
}
