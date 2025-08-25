/**************************************************************************************************
 * @file mod.rs
 * @authors Lucien
 * @brief
 * @n the app module provides the main application structure and logic.
 * @n It manages the application state, including the current page, sidebar mode,
 * @n body and footer pages, and window size.
 * @n It also handles user interactions and updates the view accordingly.
 * 
 * @version 0.1.0
 * @date 2025-06-24
 * 
 * @copyright
 * @n Copyright (c) 2021 by Loyss Studio., Division All rights reserved.
 * @n http://www.loyss.cn
 * 
**************************************************************************************************/

pub mod page;
pub mod sidebar;



/**************************************************************************************************
 * Import Internal Packages
**************************************************************************************************/
use crate::app::{
    theme::Theme,
    message::Message,
    types::{SIDEBAR_COMPACT_WIDTH, WINDOW_TITLE},
    page::Page,
    sidebar::{DisplayMode},
    widget::{sideitem::SideItemGroup, Element, Button, Container, Row},
};


/**************************************************************************************************
 * Import External Packages
**************************************************************************************************/
use std::mem::discriminant;

use iced::{
    Size, Subscription,
};


/**************************************************************************************************
 * Declaration App Struct
**************************************************************************************************/
pub struct App{
    current_page: Page,
    sidebar_mode: DisplayMode,
    sidebar_pages: Vec<SideItemGroup<Message>>,
    body_pages: Vec<SideItemGroup<Message>>,
    footer_pages: Vec<SideItemGroup<Message>>,
    window_size: Size,
    theme: Theme,
    explain: bool,
}


/**************************************************************************************************
 * Realize the APP Struct
**************************************************************************************************/
impl App<'_> {
    pub fn new() -> Self {
        Self {
            current_page: Page::Home,
            sidebar_mode: DisplayMode::Compact,
            sidebar_pages: Vec::new(),
            body_pages: Vec::new(),
            footer_pages: Vec::new(),
            window_size: Size::new(1000.0, 600.0),
            theme: Theme::default(),
            explain: false,
        }
    }
    pub fn title(&self) -> String {
        String::from(WINDOW_TITLE)
    }

    pub fn update(&mut self, message: Message) {
        match (message, &mut self.current_page) {
            (Message::PageSelected(page), _) => { self.select_page(page); }

            (Message::PageGroupToggled(label), _) => {
                if let Some(page_group) = self.body_pages
                    .iter_mut()
                    .find(|page_group| page_group.get_label() == label)
                {
                    if self.sidebar_mode == DisplayMode::Compact
                        || self.window_size.width < SIDEBAR_COMPACT_WIDTH
                    {
                        self.page_group_overlay_open = Some(page_group.get_label());
                    } else if self.sidebar_mode == DisplayMode::Full {
                        page_group.set_expanded(!page_group.get_expanded());
                    }
                }
            }

            (Message::PageGroupOverlayDismissed, _) => self.page_group_overlay_open = None,

            (Message::SidebarModeToggled, _) => {
                self.sidebar_mode = match self.sidebar_mode {
                    DisplayMode::Compact => DisplayMode::Full,
                    DisplayMode::Full => DisplayMode::Compact,
                }
            }

            (Message::WindowResized((_, size)), _) => {
                self.window_size = size;

                if size.width < SIDEBAR_COMPACT_WIDTH {
                    self.page_group_overlay_open = None;
                }
            }

            (Message::ThemeToggled, _) => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                }
            }

            (Message::ExplainToggled, _) => self.explain = !self.explain,

            (Message::DesPage(message), Page::DesClac(page)) => page.update(message),
            (Message::TripleDesPage(message), Page::TripleDesClac(page)) => page.update(message),
            (Message::AesPage(message), Page::AesClac(page)) => page.update(message),
            (Message::SM4Page(message), Page::SM4Clac(page)) => page.update(message),
            (Message::RabbitPage(message), Page::RabbitClac(page)) => page.update(message),
            (Message::BlowfishPage(message), Page::BlowfishClac(page)) => page.update(message),
            (Message::ChaCha20Clac(message), Page::ChaCha20Clac(page)) => page.update(message),

            (Message::RsaPage(message), Page::RsaClac(page)) => page.update(message),
            (Message::EccPage(message), Page::EccClac(page)) => page.update(message),
            (Message::SM2Page(message), Page::SM2Clac(page)) => page.update(message),

             _ => panic!("Message, Page pair not valid."),

        }
    }


    pub fn subscription(&self) -> Subscription<Message> {
        let window_resize_sub = iced::window::resize_events().map(Message::WindowResized);

        iced::Subscription::batch([window_resize_sub])
    }

    pub fn view(&self) -> Element<Message> {

    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn select_page(&mut self, page: Page) {
        self.page_group_overlay_open = None;

        if discriminant(&self.current_page) != discriminant(&page) {
            self.current_page = page
        }
    }
}