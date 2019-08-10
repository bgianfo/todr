///!
///! Defines methods for rendering output to the console.
///!
///! The goal being that we separate text formatting
///! from the actual processing of data exposed by the API.
///!

// Use our internal types module.
use types;
use term;

fn project_color_id_map(color: u8) -> term::color::Color {

    // The color of the project (a number between 0 and 11, or between 0 and 21 for premium users).
    match color {

    // Normal Colors

        // #95ef63
        0 => term::color::BRIGHT_GREEN,
        // #ff8581 - A pinkish, red.
        1 => term::color::BRIGHT_RED,
        // #ffc471 - Orange
        2 => term::color::RED,
        // #f9ec75
        3 => term::color::YELLOW,
        // #a8c8e4 - A light blue.
        4 => term::color::BRIGHT_BLUE,
        // #d2b8a3 - A tan color
        5 => term::color::WHITE,
        // #e2a8e4 - Another pink.
        6 => term::color::BRIGHT_RED,
        // #cccccc - A gray, map to white
        7 => term::color::WHITE,
        // #fb886e - Orange, map to red
        8 => term::color::RED,
        // #ffcc00
        9 => term::color::YELLOW,
        // #74e8d3 - bright green
        10 => term::color::BRIGHT_GREEN,
        // #3bd5fb
        11 => term::color::BRIGHT_BLUE,

    // Premium Colors

        // #dc4fad - pinkish
        12 => term::color::BRIGHT_RED,
        // #ac193d - dark red
        13 => term::color::MAGENTA,
        // #d24726 - normal red
        14 => term::color::RED,
        // #82ba00 - medium green
        15 => term::color::GREEN,
        // #03b3b2
        16 => term::color::BRIGHT_CYAN,
        // #008299
        17 => term::color::CYAN,
        // #5db2ff
        18 => term::color::BRIGHT_BLUE,
        // #0072c6
        19 => term::color::BLUE,
        // #000000
        20 => term::color::BLACK,
        // #777777 - Gray, we don't have default to white.
        21 => term::color::WHITE,

        // Default handler.
        _ => unreachable!("Unknown color id! {}", color),
    }
}

pub fn render_project(project: types::ProjectStruct) {
    // Switch the term to the color the project has set.
    let mut terminal = term::stdout().unwrap();
    let color = project_color_id_map(project.color);

    // Double the indention level so we have a nice visual indent.
    let ident = usize::from(project.indent * 2);

    terminal.fg(color).unwrap();

    println!("{:ident$}{}", "", project.name, ident = ident);

    terminal.reset().unwrap();
}

pub fn render_item(item: types::ItemStruct) {
    // Double the indention level so we have a nice visual indent.
    let ident = usize::from(item.indent * 2);

     println!("{:ident$}{} ({})",
                 "",
                 item.content,
                 item.date_string,
                 ident = ident);
}
