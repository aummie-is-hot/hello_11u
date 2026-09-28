/*
By: <Aum markandey>
Date: 2026-09-22
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

//use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::preload_image::GifLoadingScreenInfo;
use crate::utils::scale::use_virtual_resolution; // If you want to add animated GIFs to loading screen
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hello_11u".to_string(),
        window_width: 1550,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_manager = TextureManager::new();
    texture_manager.preload_with_loading_screen(&["assets/age15.png", "assets/bhs.png", "assets/soccerball.png", "assets/hello.png","assets/name.png"], None, None).await;
    
    let mut img = StillImage::new(
        "",    // Empty string creates a transparent image
        400.0, // width
        400.0, // height
        600.0, // x position
        60.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let mut lbl_text = Label::new("Click a button to get \n the corresponding action", 50.0, 100.0, 30);
    let mut btn_school = TextButton::new(50.0, 600.0, 200.0, 60.0, "School", WHITE, RED, 30);
    let mut btn_name = TextButton::new(300.0, 600.0, 200.0, 60.0, "Name", WHITE, RED, 30);
    let mut btn_sport = TextButton::new(550.0, 600.0, 200.0, 60.0, "Sport", WHITE, RED, 30);
    let mut btn_age = TextButton::new(800.0, 600.0, 200.0, 60.0, "Age", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(1300.0, 600.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
    let mut btn_hello = TextButton::new(1050.0, 600.0, 200.0, 60.0, "Hello", WHITE, RED, 30);
    btn_name.with_text_color(BLACK); // Sets the normal text color
    btn_name.with_hover_text_color(WHITE);
    btn_sport.with_text_color(BLACK); // Sets the normal text color
    btn_sport.with_hover_text_color(WHITE);
    btn_age.with_text_color(BLACK); // Sets the normal text color
    btn_age.with_hover_text_color(WHITE);
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    btn_hello.with_text_color(BLACK); // Sets the normal text color
    btn_hello.with_hover_text_color(WHITE);
    btn_school.with_text_color(BLACK); // Sets the normal text color
    btn_school.with_hover_text_color(WHITE);

    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        use_virtual_resolution(1024.0, 768.0);
        if btn_name.click() {
            lbl_text.set_text("Aum markandey");
            img.set_preload(texture_manager.get_preload("assets/name.png").unwrap());
        };
        if btn_sport.click() {
            lbl_text.set_text("My fav sport is soccer");
            img.set_preload(texture_manager.get_preload("assets/soccerball.png").unwrap());
        };
        if btn_age.click() {
            lbl_text.set_text("im 15 years old turning 16 in october");
            img.set_preload(texture_manager.get_preload("assets/age15.png").unwrap());
        };
        if btn_exit.click() {
            break;
        };
        if btn_school.click() {
            lbl_text.set_text("I attend bowmaville highschool");
            img.set_preload(texture_manager.get_preload("assets/bhs.png").unwrap());
        };
        if btn_hello.click() {
            lbl_text.set_text("Hello!");
            img.set_preload(texture_manager.get_preload("assets/hello.png").unwrap());
        };
        img.draw();

        lbl_text.draw();

        next_frame().await;
    }
}
