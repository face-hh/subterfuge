use std::collections::HashMap;

use console_engine::{
    events::Event,
    forms::{Checkbox, Form, FormField, FormOptions, FormStyle, FormValue, Radio},
    rect_style::BorderStyle,
    screen, ConsoleEngine, KeyCode, KeyModifiers,
};
use crossterm::{event::KeyEvent, style::Stylize};

use crate::structs::Feature;
use crate::structs::SaveFile;

pub fn display_errors(errors: Vec<String>) {
    println!(
        "{}",
        "𐂃  Hey there, seems like you're using locked features :)".cyan()
    );
    println!(
        "{}",
        "|********************************************************|".dark_grey()
    );
    println!(
        "{}",
        "|                       Error dump                       |".dark_grey()
    );
    println!(
        "{}",
        "|                                                        |".dark_grey()
    );

    for error in errors.iter().rev() {
        println!("{}", error);
    }

    println!(
        "{}",
        "|________________________________________________________|".dark_grey()
    );
}

pub fn display_shop(mut engine: ConsoleEngine, save_file: &SaveFile) {
    // Define a theme for the form
    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    // Create a new Form with two text inputs in it
    let mut form = Form::new(
        28,
        6,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );

    let mut check_choices = vec![];
    let features = &save_file.features;

    for feature in features {
        if feature.unlocked {
            continue;
        }

        check_choices.push(format!("{} - {}$", feature.item, feature.cost))
    }

    form.build_field::<Checkbox>(
        "checkbox",
        FormOptions {
            style: theme,
            label: Some("𐂃  Shop"),
            custom: HashMap::from([(
                String::from("choices"),
                FormValue::List(check_choices.clone()),
            )]),
            ..Default::default()
        },
    );

    form.set_active(true);

    while !form.is_finished() {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(1, 1, form.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // exit with Escape
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
                kind: _,
                state: _,
            }) => {
                break;
            }
            // exit with CTRL+C
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            }) => {
                break;
            }
            // Let the form handle the unhandled events
            event => form.handle_event(event),
        }
    }

    // we don't need the engine anymore, dropping it will close the fullscreen mode and bring us back to our terminal
    drop(engine);

    if form.is_finished() {
        // Get the output of each fields
        println!("{:?}", form.get_validated_field_output("checkbox"));
        if let Ok(FormValue::Vec(selection_list)) = form.get_validated_field_output("checkbox") {
            println!("{:?}", selection_list);
            if selection_list.is_empty() {
                println!("You selected nothing!");
            } else {
                let selection = selection_list
                    .iter()
                    .map(|x| {
                        if let FormValue::Index(id) = x {
                            check_choices[*id].clone()
                        } else {
                            check_choices[0].clone()
                        }
                    })
                    .collect::<Vec<String>>();

                println!("{:?}", selection);
            }
        }
    } else {
        println!("See you later!");
    }
}

pub fn init(arg: &String, save_file: &SaveFile) -> bool {
    let mut engine = ConsoleEngine::init(30, 8, 10).unwrap();

    if arg == "shop" {
        display_shop(engine, save_file);
        return false;
    }

    return true;
}
