// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    //TODO: export in cosmic::widget
    iced::{
        widget::{column, horizontal_rule},
        Alignment, Background, Length,
    },
    theme,
    widget::{
        self, horizontal_space,
        menu::{ItemHeight, ItemWidth, MenuBar, MenuTree},
        segmented_button,
    },
    Element,
};
use std::collections::HashMap;

use crate::{fl, icon_cache_get, Action, Config, KeyBind, Message};

macro_rules! menu_button {
    ($($x:expr),+ $(,)?) => (
        widget::button(
            widget::Row::with_children(
                vec![$(Element::from($x)),+]
            )
            .align_items(Alignment::Center)
        )
        .height(Length::Fixed(32.0))
        .padding([4, 16])
        .width(Length::Fill)
        .style(theme::Button::MenuItem)
    );
}

pub fn context_menu<'a>(
    key_binds: &HashMap<KeyBind, Action>,
    entity: segmented_button::Entity,
) -> Element<'a, Message> {
    let menu_item = |menu_label, menu_action| {
        let mut key = String::new();
        for (key_bind, key_action) in key_binds.iter() {
            if key_action == &menu_action {
                key = key_bind.to_string();
                break;
            }
        }
        menu_button!(
            widget::text(menu_label),
            horizontal_space(Length::Fill),
            widget::text(key)
        )
        .on_press(Message::TabContextAction(entity, menu_action))
    };

    widget::container(column!(
        menu_item(fl!("undo"), Action::Undo),
        menu_item(fl!("redo"), Action::Redo),
        horizontal_rule(1),
        menu_item(fl!("cut"), Action::Cut),
        menu_item(fl!("copy"), Action::Copy),
        menu_item(fl!("paste"), Action::Paste),
        menu_item(fl!("select-all"), Action::SelectAll),
    ))
    .padding(1)
    //TODO: move style to libcosmic
    .style(theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        let component = &cosmic.background.component;
        widget::container::Appearance {
            icon_color: Some(component.on.into()),
            text_color: Some(component.on.into()),
            background: Some(Background::Color(component.base.into())),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: component.divider.into(),
        }
    }))
    .width(Length::Fixed(240.0))
    .into()
}

pub fn menu_bar<'a>(config: &Config, key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    //TODO: port to libcosmic
    let menu_root = |label| {
        widget::button(widget::text(label))
            .padding([4, 12])
            .style(theme::Button::MenuRoot)
    };

    let menu_folder =
        |label| menu_button!(widget::text(label), horizontal_space(Length::Fill), ">");

    let find_key = |action: &Action| -> String {
        let mut key = String::new();
        for (key_bind, key_action) in key_binds.iter() {
            if action == key_action {
                key = key_bind.to_string();
                break;
            }
        }
        key
    };

    let menu_item = |label, action| {
        let key = find_key(&action);
        MenuTree::new(
            menu_button!(
                widget::text(label),
                horizontal_space(Length::Fill),
                widget::text(key)
            )
            .on_press(action.message()),
        )
    };

    //TODO: support key lookup?
    let menu_checkbox = |label, value, action| {
        let check: Element<_> = if value {
            icon_cache_get("object-select-symbolic", 16).into()
        } else {
            widget::Space::with_width(Length::Fixed(16.0)).into()
        };
        let key = find_key(&action);
        MenuTree::new(
            menu_button!(
                check,
                widget::Space::with_width(Length::Fixed(8.0)),
                widget::text(label),
                horizontal_space(Length::Fill),
                widget::text(key)
            )
            .on_press(action.message()),
        )
    };

    let menu_key = |label, key, action: Action| {
        MenuTree::new(
            menu_button!(widget::text(label), horizontal_space(Length::Fill), key)
                .on_press(action.message()),
        )
    };

    let menu_tab_width = |tab_width: u16| {
        menu_checkbox(
            fl!("tab-width", tab_width = tab_width),
            config.tab_width == tab_width,
            Action::TabWidth(tab_width),
        )
    };

    MenuBar::new(vec![
        MenuTree::with_children(
            menu_root(fl!("file")),
            vec![
                menu_item(fl!("new-file"), Action::NewFile),
                menu_item(fl!("new-window"), Action::NewWindow),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("open-file"), Action::OpenFileDialog),
                MenuTree::with_children(
                    menu_folder(fl!("open-recent-file")),
                    vec![menu_item(fl!("todo"), Action::Todo)],
                ),
                menu_item(fl!("close-file"), Action::CloseFile),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("menu-open-project"), Action::OpenProjectDialog),
                MenuTree::with_children(
                    menu_folder(fl!("open-recent-project")),
                    vec![menu_item(fl!("todo"), Action::Todo)],
                ),
                menu_item(fl!("close-project"), Action::CloseProject),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("save"), Action::Save),
                menu_key(fl!("save-as"), "Ctrl + Shift + S", Action::Todo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("revert-all-changes"), Action::Todo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(
                    fl!("menu-document-statistics"),
                    Action::ToggleDocumentStatistics,
                ),
                menu_item(fl!("document-type"), Action::Todo),
                menu_item(fl!("encoding"), Action::Todo),
                menu_item(fl!("menu-git-management"), Action::ToggleGitManagement),
                menu_item(fl!("print"), Action::Todo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("quit"), Action::Quit),
            ],
        ),
        MenuTree::with_children(
            menu_root(fl!("edit")),
            vec![
                menu_item(fl!("undo"), Action::Undo),
                menu_item(fl!("redo"), Action::Redo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("cut"), Action::Cut),
                menu_item(fl!("copy"), Action::Copy),
                menu_item(fl!("paste"), Action::Paste),
                menu_item(fl!("select-all"), Action::SelectAll),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("find"), Action::Find),
                menu_item(fl!("replace"), Action::FindAndReplace),
                menu_item(fl!("find-in-project"), Action::ToggleProjectSearch),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("spell-check"), Action::Todo),
            ],
        ),
        MenuTree::with_children(
            menu_root(fl!("view")),
            vec![
                MenuTree::with_children(
                    menu_folder(fl!("indentation")),
                    vec![
                        menu_checkbox(
                            fl!("automatic-indentation"),
                            config.auto_indent,
                            Action::ToggleAutoIndent,
                        ),
                        MenuTree::new(horizontal_rule(1)),
                        menu_tab_width(1),
                        menu_tab_width(2),
                        menu_tab_width(3),
                        menu_tab_width(4),
                        menu_tab_width(5),
                        menu_tab_width(6),
                        menu_tab_width(7),
                        menu_tab_width(8),
                        MenuTree::new(horizontal_rule(1)),
                        menu_item(fl!("convert-indentation-to-spaces"), Action::Todo),
                        menu_item(fl!("convert-indentation-to-tabs"), Action::Todo),
                    ],
                ),
                MenuTree::new(horizontal_rule(1)),
                menu_checkbox(fl!("word-wrap"), config.word_wrap, Action::ToggleWordWrap),
                menu_checkbox(
                    fl!("show-line-numbers"),
                    config.line_numbers,
                    Action::ToggleLineNumbers,
                ),
                menu_checkbox(fl!("highlight-current-line"), false, Action::Todo),
                menu_item(fl!("syntax-highlighting"), Action::Todo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("menu-settings"), Action::ToggleSettingsPage),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("menu-keyboard-shortcuts"), Action::Todo),
                MenuTree::new(horizontal_rule(1)),
                menu_item(fl!("about-cosmic-text-editor"), Action::Todo),
            ],
        ),
    ])
    .item_height(ItemHeight::Dynamic(40))
    .item_width(ItemWidth::Uniform(240))
    .spacing(4.0)
    .into()
}
