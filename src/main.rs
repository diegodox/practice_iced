mod pages;
use crate::pages::Pages;
use iced::Sandbox;
fn main() {
    Pages::run(iced::Settings::default())
}
