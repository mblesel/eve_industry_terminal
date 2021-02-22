use std::fmt;
use std::process;
use std::collections::HashMap;

use num_format::{Locale, ToFormattedString};

use crate::utils;
use crate::evedata::Database;

pub struct ChoiceMenu
{
    pub header: String,
    pub options: Vec<String>,
}

impl ChoiceMenu
{
    pub fn new(header: &str, options: Vec<String>) -> ChoiceMenu
    {
        ChoiceMenu {header: header.to_string(), options}
    }

    pub fn show(&self) -> i64
    {
        println!("{}", self);

        utils::parse_input::<i64>("CHOICE: ", 0, self.options.len() as i64 -1)
    }
}

impl fmt::Display for ChoiceMenu
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut message = format!("{}:", self.header);
        for i in 0..self.options.len()
        {
            message.push_str(&format!("\n  {}) {}", i, self.options[i]));
        }
        write!(f, "{}", message)
    }
}


pub fn main_menu(db: &mut Database)
{
    let choices = vec!["Exit".to_string(), "List blueprints".to_string(), 
            "List items".to_string(), "Add blueprint".to_string(), 
            "Add production run".to_string(), "Manage buy prices".to_string(),
            "Manage sell prices".to_string(), "List production runs".to_string(),
            "Create shopping list".to_string()];

    let menu = ChoiceMenu::new("MAIN MENU", choices);

    loop
    {
        let choice = menu.show();
        match choice
        {
            0 => process::exit(0),
            1 => db.print_blueprints(),
            2 => db.print_items(),
            3 => add_blueprint_menu(db),
            4 => add_productionrun_menu(db),
            5 => buy_prices_menu(db),
            6 => sell_prices_menu(db),
            7 => productionrun_menu(db),
            8 => shopping_list_menu(db),
            _ => (),
        }
    }
}


pub fn add_blueprint_menu(db: &mut Database)
{
    println!("ADD NEW BLUEPRINT:");
    let mut choices = Vec::<String>::new();
    let mut query = Vec::<(&str,i64)>::new();

    while choices.is_empty()
    {
        let input = utils::read_input("Blueprint name:");
        query = db.search_ids(&input);
        for i in &query
        {
            if i.0.contains("Blueprint")
            {
                choices.push(i.0.to_string());
            }
        }
    }

    let menu = ChoiceMenu::new("Found blueprints:", choices.clone());
    let choice = menu.show();
    let idx = query.iter().position(|&x| x.0 == choices[choice as usize]).unwrap();
    let bp_id = query[idx].1.clone();
    let bp_name = query[idx].0.to_string();
    
    if db.has_blueprint(&bp_name)
    {
        println!("This blueprint is already known.");
    }
    else
    {
        let material_research = utils::parse_input::<u8>("Material research: ", 0, 10);
        let time_research = utils::parse_input::<u8>("Time research: ", 0, 20);
        
        db.add_blueprint(bp_id as usize, material_research, time_research);
        println!("Added {} to known blueprints", bp_name);
    }

}


pub fn add_productionrun_menu(db: &mut Database)
{
    println!("ADD PRODUCTION RUN:");
    let choices = db.get_blueprint_vec();
    let menu = ChoiceMenu::new("Existing blueprints:", choices.clone());
    let choice = menu.show();
    let pr_name = &choices[choice as usize];

    if db.has_productionrun(pr_name)
    {
        println!("This blueprint already has a production run defined");
    }
    else
    {
        let jobruns = utils::parse_input::<u64>("Job runs: ", 1, std::u64::MAX);
        let installation_cost = utils::parse_input::<u64>("Installation cost: ", 1, std::u64::MAX);

        db.add_productionrun(pr_name, jobruns, installation_cost);
        println!("Added {} to production runs", pr_name);
    }
}

pub fn productionrun_menu(db: &Database)
{
    for iter in db.get_productionrun_iter()
    {
        let production_cost = iter.1.get_production_cost(db);
        let sell_value = iter.1.get_sell_value(db);
        let raw_profit = sell_value - production_cost;
        let taxed_profit = raw_profit - (sell_value as f64 / 100 as f64 * 10 as f64) as u64;
        println!("{} x {}:\n  Production cost: {}\n  Sell value: {} x {} = {}\n  \
            raw profit: {}\n  minus fees (-10% overall sell value): {}",
            iter.0, iter.1.jobruns, production_cost.to_formatted_string(&Locale::en),
            (sell_value/iter.1.jobruns).to_formatted_string(&Locale::en), 
            iter.1.jobruns, sell_value.to_formatted_string(&Locale::en), 
            raw_profit.to_formatted_string(&Locale::en),
            taxed_profit.to_formatted_string(&Locale::en));
    }
}

pub fn buy_prices_menu(db: &mut Database)
{
    loop
    {
        println!("SET BUY PRICES:");
        let mut choices = vec!["Back".to_string()];
        let mut item_list = vec!["Back".to_string()];

        for iter in db.get_item_iter()
        {
            if iter.1.produced == false
            {
                item_list.push(iter.1.name.clone());
                let s = format!("{}: {}ISK",iter.1.name,
                    iter.1.buy_price.to_formatted_string(&Locale::en));
                choices.push(s);
            }
        }

        let menu = ChoiceMenu::new("Select item:", choices.clone());
        let choice = menu.show();
        if choice == 0
        {
            break;
        }

        let new_price = utils::parse_input::<u64>("New buy price: ", 1, std::u64::MAX);
        db.set_item_buy_price(&item_list[choice as usize], new_price);
    }
}

pub fn sell_prices_menu(db: &mut Database)
{
    loop
    {
        println!("SET sell PRICES:");
        let mut choices = vec!["Back".to_string()];
        let mut item_list = vec!["Back".to_string()];

        for iter in db.known_items.iter()
        {
            if iter.1.produced
            {
                item_list.push(iter.1.name.clone());
                let s = format!("{}: {}ISK",iter.1.name,
                    iter.1.sell_price.to_formatted_string(&Locale::en));
                choices.push(s);
            }
        }

        let menu = ChoiceMenu::new("Select item:", choices.clone());
        let choice = menu.show();
        if choice == 0
        {
            break;
        }

        let new_price = utils::parse_input::<u64>("New sell price: ", 1, std::u64::MAX);
        db.set_item_sell_price(&item_list[choice as usize], new_price);
    }
}

fn shopping_list_menu(db: &mut Database)
{
    println!("SHOPPING LIST:");
    let mut choices = vec!["Done".to_string()];
    let mut pr_list = vec!["Done".to_string()];
    let mut shopping_list = Vec::<String>::new();

    for iter in db.get_productionrun_iter()
    {
        pr_list.push(iter.0.clone());
        choices.push(format!("{} x {}", iter.0.clone(), iter.1.jobruns));
    }


    let menu = ChoiceMenu::new("Add production run:", choices.clone());

    loop
    {
        let choice = menu.show();
        if choice == 0
        {
            break;
        }
        shopping_list.push(pr_list[choice as usize].clone());
    }

    let mut item_list = HashMap::<String, u64>::new();
    println!("------------------------------------------------------");
    println!("Shopping list productions:");
    for iter in shopping_list.iter()
    {
        let pr = db.get_productionrun(iter).unwrap();
        let materials = pr.get_production_materials(db);

        println!("  {} x {}", iter, pr.jobruns);

        for iter in materials.iter()
        {
            if item_list.contains_key(&iter.0)
            {
                *item_list.get_mut(&iter.0).unwrap() += iter.1;
            }
            else
            {
                item_list.insert(iter.0.to_string(),iter.1);
            }
        }
    }

    println!("Item shopping list:");
    for iter in item_list.iter()
    {
        let item = db.get_item(iter.0).unwrap();
        println!("  {}  @{}  x  {}", item.name,
            item.buy_price.to_formatted_string(&Locale::en), iter.1);
    }
    println!("------------------------------------------------------");
}

