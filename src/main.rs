use std::{thread};
use raylib::{prelude::*, ffi::{InitAudioDevice, CloseAudioDevice, CloseWindow}};
use chrono::{Utc, Datelike, Timelike};
// use std::{thread, time};

macro_rules! attempt { // `try` is a reserved keyword
    (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
    };
    (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
    };
    ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($e) { $($tail)* } $($handler)* }
    };
 }

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(1920, 1080)
    .title("Jake is Cool")
    .fullscreen()
    .build();

    
    rl.hide_cursor();
    rl.set_target_fps(75);
    
    
    // Init
    unsafe{
        InitAudioDevice();
    }

    
    let mut i = 0;
    let time_match: i32 = 75;
    get_hour(true);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);


        if i < time_match {
            d.clear_background(Color::from_hex("31313A").unwrap());
            d.draw_text("Made by Messycode :: Jake", 960, 540, 40, Color::WHITE);
            d.draw_text(format!("{} :: {}", i, time_match).as_str(), 0, 0, 30, Color::WHITE);
            d.draw_fps(1830,0);
        } else {game_loop(d)}
        
        i += 1;
    }

    // deinit
    unsafe {
        CloseAudioDevice();
        CloseWindow();
    }
}

fn game_loop(mut d: RaylibDrawHandle) {
    let background_color = "181818";

    d.draw_fps(1830,0);

    d.clear_background(Color::from_hex(format!("{}", background_color).as_str()).unwrap());
    d.draw_text("and you know it", 560, 60, 20, Color::from_hex("c9c9b7").unwrap());
    d.draw_text(format!(
        "Jake is COOL\n{}-{}-{}   {}:{}:{}",
        Utc::now().month(),
        Utc::now().day(),
        Utc::now().year(),
        get_hour(false),
        Utc::now().minute(),
        Utc::now().second()
        ).as_str(), 12, 12, 80, Color::from_hex("c9c9b7").unwrap());

    if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
        thread::spawn(|| {
            std::process::Command::new("vlc")
            .arg("src/audio/Jake_is_cool.mp3")
            .arg("--play-and-exit")
            .arg("-Irc")
            .status()
            .expect("Something Failed!!!");
        });
    }
}

fn get_hour(print_hour_set: bool) -> u32 {
    if print_hour_set == true{
        println!("\nHOUR:: {}\n", Utc::now().hour());
        println!("\nHOUR:: {}\n", Utc::now().hour() + 5);
    
        attempt!{{
            println!("\nHOUR:: {}\n", Utc::now().hour() - 7);
        } catch(e) {
            println!("aheelo");
        }}
    }

    if Utc::now().hour() <= 7 {
        return Utc::now().hour() + 5;
    }else {
        return Utc::now().hour() - 7;
    }
}