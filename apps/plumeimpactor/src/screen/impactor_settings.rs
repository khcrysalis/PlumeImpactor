use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Center, Element, Fill};
use iced_aw::SelectionList;
use plume_store::AccountStore;

use crate::{Message, appearance};

pub fn view(account_store: Option<&AccountStore>) -> Element<'static, Message> {
    let mut content = column![].spacing(10).padding(10);

    if let Some(store) = account_store {
        let mut accounts: Vec<_> = store.accounts().iter().collect();
        accounts.sort_by_key(|(email, _)| *email);

        let selected_email = store.selected_account().map(|a| a.email().to_string());

        let selected_index = if let Some(ref email) = selected_email {
            accounts.iter().position(|(e, _)| *e == email)
        } else {
            None
        };

        if accounts.is_empty() {
            content = content.push(text("No accounts added yet"));
        } else {
            let account_labels: &'static [String] = Box::leak(
                accounts
                    .iter()
                    .enumerate()
                    .map(|(index, (_, account))| {
                        let first_name = account.first_name();
                        let name = if !first_name.is_empty() {
                            format!("{} ({})", first_name, account.email())
                        } else {
                            account.email().to_string()
                        };

                        if Some(index) == selected_index {
                            format!(" [✓] {}", name)
                        } else {
                            format!(" [ ] {}", name)
                        }
                    })
                    .collect::<Vec<String>>()
                    .into_boxed_slice(),
            );

            let selection_list = SelectionList::new_with(
                account_labels,
                |index, _label| Message::SelectAccount(index),
                appearance::THEME_FONT_SIZE.into(),
                5.0,
                iced_aw::style::selection_list::primary,
                selected_index,
                appearance::p_font(),
            );

            content = content.push(container(selection_list).height(Fill).style(
                |theme: &iced::Theme| container::Style {
                    border: iced::Border {
                        width: 1.0,
                        color: theme.palette().background.scale_alpha(0.5),
                        radius: appearance::THEME_CORNER_RADIUS.into(),
                    },
                    ..Default::default()
                },
            ));
        }

        let mut buttons = row![].spacing(10);

        buttons = buttons.push(
            button(text("Add Account").align_x(Center))
                .on_press(Message::ShowLogin)
                .style(appearance::p_button),
        );

        if selected_index.is_some() {
            buttons = buttons.push(
                button(text("Remove Selected").align_x(Center))
                    .on_press_maybe(selected_index.map(Message::RemoveAccount))
                    .style(appearance::p_button),
            );
            buttons = buttons.push(
                button(text("Export P12").align_x(Center))
                    .on_press(Message::ExportP12)
                    .style(appearance::p_button),
            );
        }

        content = content.push(buttons.align_y(Alignment::Center));
    } else {
        content = content.push(text("Loading accounts..."));
    }

    content.into()
}
