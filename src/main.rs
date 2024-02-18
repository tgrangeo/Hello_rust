use console::Term;
use dialoguer::{console, theme::ColorfulTheme, Confirm, Input, Select};

fn main() {
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();
    let mut camel_count: i32 = 0;

    println!("In this CLI, I will ask you questions to determine the number of camels your wife is worth.🐪🐪🐪🐪🐪");
    Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact_on(&term)
        .unwrap();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Name of your wife ?")
        .interact_on(&term)
        .unwrap();


    let old: &[&str; 6] = &[
    "18 - 20",
    "21 - 25",
    "26 - 30",
    "31 - 35",
    "36 - 40",
    "41+"
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("How old is she ?")
        .items(old)
        .interact_on(&term)
        .unwrap();
    match selection{
        0 => camel_count += 18,
        1 => camel_count += 20,
        2 => camel_count += 18,
        3 => camel_count += 16,
        4 => camel_count += 14,
        5 => camel_count += 12,
        _ => {},
    }

    let tall: &[&str; 6] = &[
    "1m50 or less", 
    "1m51 - 1m60",
    "1m61 - 1m70",
    "1m71 - 1m80",
    "1m81 - 1m90",
    "1m91+"
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("How tall is she ?")
        .items(tall)
        .interact_on(&term)
        .unwrap();
    match selection{
        0 => camel_count += 10,
        1 => camel_count += 12,
        2 => camel_count += 14,
        3 => camel_count += 16,
        4 => camel_count += 14,
        5 => camel_count += 12,
        _ => {},
    }

    let hair: &[&str; 3] = &[
        "redhead",
        "blonde", 
        "brunette",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("What's her hair color ?")
        .items(hair)
        .interact_on(&term)
        .unwrap();
    match selection{
        0 => camel_count += 10,
        1 => camel_count += 8,
        2 => camel_count += 5,
        _ => {},
    }

    let hair: &[&str; 3] = &[
        "green",
        "blue", 
        "brown",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("What's her eyes color ?")
        .items(hair)
        .interact_on(&term)
        .unwrap();
    match selection{
        0 => camel_count += 10,
        1 => camel_count += 8,
        2 => camel_count += 5,
        _ => {},
    }

    let weight: &[&str; 5] = &[
    "Shrimp",
    "Rabbit",
    "Horse",
    "Bear",
    "Elephant",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt("Define her weigth with the good animal ?")
        .items(weight)
        .interact_on(&term)
        .unwrap();
    match selection{
        0 => camel_count += 18,
        1 => camel_count += 20,
        2 => camel_count += 18,
        3 => camel_count += 16,
        4 => camel_count += 14,
        5 => camel_count += 12,
        _ => {},
    }
    println!("{} worth {} camels 🐪", name,camel_count);
}