// SPDX-License-Identifier: GPL-3.0-only

use crate::fl;
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, Config, CosmicConfigEntry},
    Application,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
#[id = "fun.wmde.start-menu"]
pub struct AppletConfig {
    pub app_menu_position: HorizontalPosition,
    pub applet_button_style: AppletButtonStyle,
    pub user_widget: UserWidgetStyle,
    pub button_label: String,
    pub button_icon: String,
    pub recent_applications: Vec<RecentApplication>,
}

impl Default for AppletConfig {
    fn default() -> Self {
        AppletConfig {
            app_menu_position: HorizontalPosition::default(),
            applet_button_style: AppletButtonStyle::default(),
            user_widget: UserWidgetStyle::default(),
            button_label: fl!("menu-label").to_owned(),
            button_icon: format!("/usr/share/{}/default.svg", applet_buttons_rel_dir()),
            recent_applications: vec![],
        }
    }
}

/// XDG data-dir relative directory holding the start-button icon set.
/// Single source of truth for the applet default, the settings picker and packaging.
pub fn applet_buttons_rel_dir() -> String {
    format!("wmde/{}/applet-buttons", crate::applet::Applet::APP_ID)
}

impl AppletConfig {
    pub fn config_handler() -> Option<Config> {
        Config::new(crate::applet::Applet::APP_ID, 1).ok()
    }

    pub fn config() -> AppletConfig {
        match Self::config_handler() {
            Some(config_handler) => AppletConfig::get_entry(&config_handler)
                .unwrap_or_else(|(_errs, config)| config),
            None => AppletConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]

pub enum AppletButtonStyle {
    IconOnly,
    LabelOnly,
    IconAndLabel,
    Auto,
}

impl Default for AppletButtonStyle {
    fn default() -> Self {
        AppletButtonStyle::Auto
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]

pub enum UserWidgetStyle {
    UsernamePrefered,
    RealNamePrefered,
    None,
}

impl Default for UserWidgetStyle {
    fn default() -> Self {
        UserWidgetStyle::UsernamePrefered
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]

pub enum HorizontalPosition {
    Left,
    Right,
}

impl Default for HorizontalPosition {
    fn default() -> Self {
        HorizontalPosition::Left
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecentApplication {
    pub app_id: String,
    pub launch_count: u32,
}
