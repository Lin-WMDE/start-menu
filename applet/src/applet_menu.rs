use std::path::PathBuf;

use cosmic::cosmic_theme::Spacing;
use cosmic::iced::{
    Alignment, Length,
    widget::{column, row},
};
use cosmic::iced::{ContentFit, Font, Limits};
use cosmic::widget::text;
use cosmic::widget::{container, menu};
use cosmic::{Element, theme};

use crate::applet::{Applet, Message};
use crate::config::HorizontalPosition;
use crate::model::system_tool::SystemTool;
use crate::fl;
use crate::model::power_action::PowerAction;
use crate::widgets::VirtualizedAppList;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextMenuAction {
    LaunchApplication(usize),
    LaunchApplicationWithAction(usize, usize),
    PinToPanel(usize, bool),
}

impl menu::Action for ContextMenuAction {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            ContextMenuAction::LaunchApplication(index) => Message::LaunchApplicationAt(*index),
            ContextMenuAction::LaunchApplicationWithAction(app_index, action_index) => {
                Message::LaunchApplicationWithActionAt(*app_index, *action_index)
            }
            ContextMenuAction::PinToPanel(index, favorites) => {
                Message::PinToAppTrayIndex(*index, *favorites)
            }
        }
    }
}

pub struct AppletMenu;

impl AppletMenu {
    pub const POPUP_MAX_WIDTH: f32 = 720.0;
    pub const POPUP_MIN_WIDTH: f32 = 440.0;
    pub const POPUP_MAX_HEIGHT: f32 = 680.0;
    pub const POPUP_MIN_HEIGHT: f32 = 300.0;

    const SYSTEM_LOCKSCREEN_SYMBOLIC_ICON: &[u8] =
        include_bytes!("../../res/icons/bundled/system-lock-screen-symbolic.svg");
    const SYSTEM_LOGOUT_SYMBOLIC_ICON: &[u8] =
        include_bytes!("../../res/icons/bundled/system-log-out-symbolic.svg");
    const SYSTEM_REBOOT_SYMBOLIC_ICON: &[u8] =
        include_bytes!("../../res/icons/bundled/system-reboot-symbolic.svg");
    const SYSTEM_SHUTDOWN_SYMBOLIC_ICON: &[u8] =
        include_bytes!("../../res/icons/bundled/system-shutdown-symbolic.svg");
    const SYSTEM_SUSPEND_SYMBOLIC_ICON: &[u8] =
        include_bytes!("../../res/icons/bundled/system-suspend-symbolic.svg");
    const USER_IDLE_SYMBOLIC: &[u8] =
        include_bytes!("../../res/icons/bundled/user-idle-symbolic.svg");

    pub fn view_main_menu_list(applet: &Applet) -> Element<'_, Message> {
        let Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let current_user = AppletMenu::create_logged_user_widget(applet);
        let search_field = AppletMenu::create_search_field(applet);
        let app_list = AppletMenu::create_app_list(applet);
        let categories_pane = AppletMenu::create_categories_pane(applet);
        let vertical_spacer =
            cosmic::applet::padded_control(cosmic::widget::divider::vertical::default())
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Shrink)
                .padding(8);

        // WMDE: static gear that always opens WMDE Settings, independent of the
        // (search/category filtered) application list.
        let settings_button: Element<'_, Message> = cosmic::widget::button::custom(
            cosmic::widget::icon::icon(
                cosmic::widget::icon::from_name("preferences-system-symbolic")
                    .size(24)
                    .handle(),
            )
            .size(24),
        )
        .on_press(Message::LaunchTool(SystemTool::SYSTEM_SETTINGS))
        .class(cosmic::theme::Button::AppletMenu)
        .into();

        let header = container(
            row![
                container(current_user).width(Length::FillPortion(3)),
                cosmic::widget::Space::new().width(Length::Fixed(17.0)),
                row![search_field, settings_button]
                    .spacing(space_xxs as f32)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(5)),
            ]
            .align_y(Alignment::Center),
        )
        .class(cosmic::theme::Container::Primary)
        .width(Length::Fill)
        .padding([2, space_xxs]);

        let dual_pane = match applet.config.app_menu_position {
            HorizontalPosition::Left => {
                row![app_list, vertical_spacer, categories_pane]
            }
            HorizontalPosition::Right => {
                row![categories_pane, vertical_spacer, app_list]
            }
        }
        .padding([space_xxs, space_xxs])
        .height(Length::Fixed(432.0));

        let top_divider =
            cosmic::widget::divider::horizontal::default();
        let bottom_divider =
            cosmic::widget::divider::horizontal::default();

        let footer = container(
            row![
                cosmic::widget::Space::new().width(Length::Fill),
                AppletMenu::create_power_menu(applet),
            ]
            .align_y(Alignment::Center),
        )
        .class(cosmic::theme::Container::Primary)
        .width(Length::Fill);

        let menu_layout =
            column![header, top_divider, dual_pane, bottom_divider, footer];

        applet
            .core
            .applet
            .popup_container(
                menu_layout
                    .width(Length::Fixed(640.))
                    .height(Length::Shrink),
            )
            .limits(
                Limits::NONE
                    .max_height(AppletMenu::POPUP_MAX_HEIGHT)
                    .min_height(AppletMenu::POPUP_MIN_HEIGHT)
                    .max_width(AppletMenu::POPUP_MAX_WIDTH)
                    .min_width(AppletMenu::POPUP_MIN_WIDTH),
            )
            .into()
    }

    fn create_power_menu(_applet: &Applet) -> Element<'_, Message> {
        container(
            row![
                cosmic::widget::button::icon(cosmic::widget::icon::from_svg_bytes(
                    AppletMenu::SYSTEM_LOGOUT_SYMBOLIC_ICON,
                ).symbolic(true))
                .on_press(Message::PowerOptionSelected(PowerAction::Logout)),
                cosmic::widget::button::icon(cosmic::widget::icon::from_svg_bytes(
                    AppletMenu::SYSTEM_SUSPEND_SYMBOLIC_ICON,
                ).symbolic(true))
                .on_press(Message::PowerOptionSelected(PowerAction::Suspend)),
                cosmic::widget::button::icon(cosmic::widget::icon::from_svg_bytes(
                    AppletMenu::SYSTEM_LOCKSCREEN_SYMBOLIC_ICON,
                ).symbolic(true))
                .on_press(Message::PowerOptionSelected(PowerAction::Lock)),
                cosmic::widget::button::icon(cosmic::widget::icon::from_svg_bytes(
                    AppletMenu::SYSTEM_REBOOT_SYMBOLIC_ICON,
                ).symbolic(true))
                .on_press(Message::PowerOptionSelected(PowerAction::Reboot)),
                cosmic::widget::button::icon(cosmic::widget::icon::from_svg_bytes(
                    AppletMenu::SYSTEM_SHUTDOWN_SYMBOLIC_ICON,
                ).symbolic(true))
                .on_press(Message::PowerOptionSelected(PowerAction::Shutdown)),
            ]
            .align_y(Alignment::Center),
        )
        .width(Length::Shrink)
        .padding([10, 8])
        .align_x(Alignment::Center)
        .into()
    }

    fn create_search_field(applet: &Applet) -> Element<'_, Message> {
        let Spacing { space_s, .. } = theme::active().cosmic().spacing;

        cosmic::widget::search_input(fl!("search-placeholder"), &applet.search_field)
            .always_active()
            .on_input(Message::SearchFieldInput)
            .on_clear(Message::SearchCleared)
            .width(Length::Fill)
            .padding([0, space_s])
            .into()
    }

    fn create_app_list(applet: &Applet) -> Element<'_, Message> {
        VirtualizedAppList::view(applet)
    }

    fn create_categories_pane(applet: &Applet) -> Element<'_, Message> {
        let mut buttons: Vec<Element<Message>> = applet
            .available_categories
            .iter()
            .map(|category| {
                cosmic::widget::button::custom(
                    row![
                        container(
                            cosmic::widget::icon::from_svg_bytes(category.icon_svg_bytes)
                                .symbolic(true)
                                .icon()
                        )
                        .padding([0, 6]),
                        text(category.get_display_name()),
                    ]
                    .align_y(Alignment::Center),
                )
                .on_press(Message::CategorySelected(category.clone()))
                .class(if applet.selected_category == Some(category.clone()) {
                    cosmic::theme::Button::Suggested
                } else {
                    cosmic::theme::Button::AppletMenu
                })
                .width(Length::Fill)
                .into()
            })
            .collect();

        let group2 = if buttons.len() > 2 {
            buttons.split_off(2)
        } else {
            Vec::new()
        };
        let group1 = buttons;

        let top = cosmic::widget::column::with_children(group1).spacing(8.0);
        let bottom = cosmic::widget::column::with_children(group2).spacing(8.0);
        let divider =
            cosmic::applet::padded_control(cosmic::widget::divider::horizontal::default())
                .padding(0);

        cosmic::widget::column::with_children(vec![top.into(), divider.into(), bottom.into()])
            .spacing(3.0)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .into()
    }

    pub fn create_logged_user_widget(applet: &Applet) -> Element<'_, Message> {
        if applet.config.user_widget == crate::config::UserWidgetStyle::None {
            return cosmic::widget::Space::new().width(0).height(0).into();
        }

        if let Some(user) = &applet.current_user {
            let profile_picture_widget: Element<Message> =
                if PathBuf::from(&user.profile_picture).exists() {
                    cosmic::widget::image(&user.profile_picture)
                        .width(Length::Fixed(28.))
                        .height(Length::Fixed(28.))
                        .content_fit(ContentFit::ScaleDown)
                        .border_radius([5.; 4])
                        .into()
                } else {
                    cosmic::widget::icon::from_svg_bytes(AppletMenu::USER_IDLE_SYMBOLIC)
                        .symbolic(true)
                        .icon()
                        .size(28)
                        .into()
                };

            let nametag_widget: Element<Message> = match &applet.config.user_widget {
                crate::config::UserWidgetStyle::UsernamePrefered => text(&user.username)
                    .font(Font {
                        weight: cosmic::iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .size(16)
                    .into(),
                crate::config::UserWidgetStyle::RealNamePrefered => {
                    if !&user.user_realname.is_empty() {
                        column![
                            text(&user.user_realname)
                                .font(Font {
                                    weight: cosmic::iced::font::Weight::Bold,
                                    ..Default::default()
                                })
                                .size(16),
                            text(&user.username).size(10),
                        ]
                        .into()
                    } else {
                        text(&user.username)
                            .font(Font {
                                weight: cosmic::iced::font::Weight::Bold,
                                ..Default::default()
                            })
                            .size(16)
                            .into()
                    }
                }
                crate::config::UserWidgetStyle::None => {
                    cosmic::widget::Space::new().width(0).height(0).into()
                }
            };

            row![
                profile_picture_widget,
                cosmic::widget::Space::new().width(5).height(Length::Shrink),
                nametag_widget
            ]
            .align_y(Alignment::Center)
            .padding([10., 0.])
            .into()
        } else {
            cosmic::widget::Space::new().width(0).height(0).into()
        }
    }
}
