// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use cosmic::cosmic_theme::Spacing;
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::widget::{column, row};
use cosmic::iced::window::Id;
use cosmic::iced::{Alignment, ContentFit, Length};
use cosmic::widget::text;
use cosmic::widget::{container, scrollable};
use cosmic::{Element, theme};

use crate::applet::{Applet, Message};
use crate::model::application_entry::ApplicationEntry;

/// Kickoff-style application grid: icon over label, responsive columns.
pub struct VirtualizedAppList;

impl VirtualizedAppList {
    /// Number of columns in the application grid
    pub const COLS: usize = 4;
    /// Tile height (icon + up to two label lines)
    pub const TILE_HEIGHT: f32 = 96.0;
    /// Extra rows rendered above/below the visible window
    const OVERSCAN_ROWS: usize = 2;
    /// Assumed viewport height for the first frame, before any ScrollUpdated arrives
    const FALLBACK_VIEWPORT: f32 = 800.0;

    pub fn view(applet: &Applet) -> Element<'_, Message> {
        let Spacing {
            space_xs, space_s, ..
        } = theme::active().cosmic().spacing;

        let spacing = space_xs as f32;
        let row_h = Self::TILE_HEIGHT + spacing;
        let apps = &applet.available_applications;
        let total_rows = apps.len().div_ceil(Self::COLS);

        // Windowed rendering: only build tiles (and clone their context menus)
        // for the rows near the viewport; spacers keep the scrollbar geometry.
        let viewport_h = if applet.scroll_viewport_height > 0.0 {
            applet.scroll_viewport_height
        } else {
            Self::FALLBACK_VIEWPORT
        };
        let first_row = ((applet.scroll_offset / row_h).floor().max(0.0) as usize)
            .saturating_sub(Self::OVERSCAN_ROWS)
            .min(total_rows);
        let last_row = ((((applet.scroll_offset + viewport_h) / row_h).ceil() as usize)
            + Self::OVERSCAN_ROWS)
            .min(total_rows);

        let mut grid = column![].spacing(spacing).width(Length::Fill);

        if first_row > 0 {
            // first_row rows plus the inter-row spacing the column would have added
            let h = first_row as f32 * row_h - spacing;
            grid = grid.push(
                cosmic::widget::Space::new()
                    .width(Length::Fill)
                    .height(Length::Fixed(h)),
            );
        }
        for (row_i, chunk) in apps
            .chunks(Self::COLS)
            .enumerate()
            .skip(first_row)
            .take(last_row.saturating_sub(first_row))
        {
            let mut r = row![].spacing(spacing).width(Length::Fill);
            for (col_i, app) in chunk.iter().enumerate() {
                r = r.push(Self::create_app_tile(applet, app, row_i * Self::COLS + col_i));
            }
            // pad the last row so columns stay aligned
            for _ in chunk.len()..Self::COLS {
                r = r.push(
                    cosmic::widget::Space::new()
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0)),
                );
            }
            grid = grid.push(r);
        }
        if last_row < total_rows {
            let h = (total_rows - last_row) as f32 * row_h - spacing;
            grid = grid.push(
                cosmic::widget::Space::new()
                    .width(Length::Fill)
                    .height(Length::Fixed(h)),
            );
        }

        scrollable(container(grid).padding([0., space_s as f32]))
            .height(Length::Fill)
            .width(Length::FillPortion(5))
            .id(applet.scrollable_id.clone())
            .on_scroll(|viewport| Message::ScrollUpdated(viewport))
            .into()
    }

    fn create_app_tile<'a>(
        applet: &'a Applet,
        app: &'a Arc<ApplicationEntry>,
        index: usize,
    ) -> Element<'a, Message> {
        let icon = Self::create_icon_widget(app, 40);
        let label = text(&app.name)
            .size(10.0)
            .width(Length::Fill)
            .align_x(Horizontal::Center);

        // Highlight the keyboard-selected tile
        let selected = applet.selected_item_index == Some(index);
        let tile = cosmic::widget::button::custom(
            column![icon, label]
                .spacing(4)
                .align_x(Alignment::Center)
                .width(Length::Fill),
        )
        .on_press(Message::ApplicationSelected(app.clone()))
        .class(if selected {
            cosmic::theme::Button::Suggested
        } else {
            cosmic::theme::Button::AppletMenu
        })
        .width(Length::Fill)
        .height(Length::Fixed(Self::TILE_HEIGHT));

        let context_menu = Self::create_context_menu(applet, app);

        cosmic::widget::context_menu(tile, context_menu)
            .close_on_escape(true)
            .on_surface_action(Message::ContextMenuAction)
            .window_id(applet.popup.unwrap_or_else(|| Id::NONE))
            .into()
    }

    pub fn create_icon_widget(app: &Arc<ApplicationEntry>, space_l: u16) -> Element<'_, Message> {
        let default_icon = crate::model::application_entry::IconHandle::default();
        let icon_handle = app.icon.as_ref().unwrap_or(&default_icon);

        match icon_handle {
            crate::model::application_entry::IconHandle::SvgHandle(handle) => container(
                cosmic::widget::svg(handle.clone())
                    .width(Length::Fixed(space_l.into()))
                    .height(Length::Fixed(space_l.into()))
                    .content_fit(ContentFit::Contain),
            )
            .into(),
            crate::model::application_entry::IconHandle::RasterHandle(handle) => container(
                cosmic::widget::image(handle.clone())
                    .width(Length::Fixed(space_l.into()))
                    .height(Length::Fixed(space_l.into()))
                    .content_fit(ContentFit::Contain),
            )
            .into(),
        }
    }

    fn create_context_menu<'a>(
        applet: &'a Applet,
        app: &'a Arc<ApplicationEntry>,
    ) -> Option<Vec<cosmic::widget::menu::Tree<Message>>> {
        applet.context_menus.get(&app.id).cloned()
    }
}
