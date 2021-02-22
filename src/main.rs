mod utils;
mod evedata;
mod menu;


fn main()
{
    let mut db = evedata::Database::new("/home/michael/Projects/eve_industry_terminal/data");

    menu::main_menu(&mut db);
}
