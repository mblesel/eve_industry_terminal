use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::fs::{self, File};
use std::path::Path;
use std::convert::TryInto;

use yaml_rust::Yaml;
use serde::{Serialize, Deserialize};
use num_format::{Locale, ToFormattedString};

use crate::utils;

pub struct Database
{
    data_base_dir: &'static str,
    pub blueprints:  Yaml,
    pub type_ids:  Yaml,
    pub known_blueprints: HashMap<String, T1Blueprint>,
    pub known_items: HashMap<String, Item>,
    pub productionruns: HashMap<String, T1ProductionRun>,
}

#[allow(dead_code)]
impl Database
{
    pub fn new(data_base_dir: &'static str) -> Database
    {
        println!("Loading resources");
        let blueprints = utils::load_yaml(&format!("{}/sde/fsd/blueprints.yaml",data_base_dir))
            .remove(0);
        let type_ids = utils::load_yaml(&format!("{}/sde/fsd/typeIDs.yaml",data_base_dir))
            .remove(0);
        let(known_blueprints, known_items, productionruns) = load_resources(data_base_dir);

        Database {data_base_dir, blueprints, type_ids, known_blueprints,
            known_items, productionruns}
    }

    pub fn get_blueprint(&self, name: &str) -> Option<&T1Blueprint>
    {
        self.known_blueprints.get(name)
    }

    pub fn get_item(&self, name: &str) -> Option<&Item>
    {
        self.known_items.get(name)
    }

    pub fn get_productionrun(&self, name: &str) -> Option<&T1ProductionRun>
    {
        self.productionruns.get(name)
    }

    pub fn get_blueprint_vec(&self) -> Vec<String>
    {
        let mut ret = Vec::<String>::new();

        for iter in self.known_blueprints.iter()
        {
            ret.push(iter.0.clone());
        }
        ret
    }

    pub fn get_productionrun_vec(&self) -> Vec<String>
    {
        let mut ret = Vec::<String>::new();

        for iter in self.productionruns.iter()
        {
            ret.push(iter.0.clone());
        }
        ret
    }

    pub fn get_item_iter(&self) -> std::collections::hash_map::Iter<String,Item>
    {
        self.known_items.iter()
    }

    pub fn get_productionrun_iter(&self) 
        -> std::collections::hash_map::Iter<String,T1ProductionRun>
    {
        self.productionruns.iter()
    }

    pub fn set_item_buy_price(&mut self,item_name: &str, new_price: u64)
    {
        let item = self.known_items.get_mut(item_name).unwrap();
        item.buy_price = new_price;
        self.save_item(item_name);
    }

    pub fn set_item_sell_price(&mut self,item_name: &str, new_price: u64)
    {
        let item = self.known_items.get_mut(item_name).unwrap();
        item.sell_price = new_price;
        self.save_item(item_name);
    }

    pub fn add_blueprint(&mut self, bp_id: usize, material_research: u8, time_research: u8)
    {
        let bp = T1Blueprint::new(bp_id as usize, material_research, time_research, self);
        let bp_name = bp.name.clone();
        self.known_blueprints.insert(bp.name.clone(), bp);
        self.save_blueprint(&bp_name);
    }

    pub fn add_productionrun(&mut self, pr_name: &str, jobruns: u64, installation_cost: u64)
    {
        let pr = T1ProductionRun::new(pr_name, jobruns, installation_cost, self);
        self.productionruns.insert(pr.blueprint.clone(), pr);
        self.save_prodcutionrun(pr_name);
    }

    pub fn save_blueprint(&self, bp_name: &str)
    {
        let base_path = &format!("{}/blueprints", self.data_base_dir); 

        if Path::new(&base_path).exists() == false
        {
            eprintln!("Directory: {} does not exist", base_path);
            panic!();
        }

        let bp = self.known_blueprints.get(bp_name).expect("Blueprint not found in database");

        let file_path = format!("{}/{}.json", base_path, bp.name);
        let serialzed = serde_json::to_string(&bp).unwrap();
        fs::write(&file_path, serialzed).expect("Could not write blueprint file");
    }

    pub fn save_item(&self, item_name: &str)
    {
        let base_path = &format!("{}/items", self.data_base_dir); 

        if Path::new(&base_path).exists() == false
        {
            eprintln!("Directory: {} does not exist", base_path);
            panic!();
        }

        let item = self.known_items.get(item_name).expect("Item not found in database");

        let file_path = format!("{}/{}.json", base_path, item.name);

        let serialzed = serde_json::to_string(&item).unwrap();
        fs::write(&file_path, serialzed).expect("Could not write item file");
    }

    pub fn save_prodcutionrun(&self, pr_name: &str)
    {
        let base_path = &format!("{}/productionruns", self.data_base_dir); 

        if Path::new(&base_path).exists() == false
        {
            eprintln!("Directory: {} does not exist", base_path);
            panic!();
        }

        let item = self.productionruns.get(pr_name).expect("Productionrun not found in database");

        let file_path = format!("{}/{}.json", base_path, pr_name);

        let serialzed = serde_json::to_string(&item).unwrap();
        fs::write(&file_path, serialzed).expect("Could not write productionrun file");
    }

    pub fn search_ids(&self, query: &str) -> Vec<(&str, i64)>
    {
        let type_ids_hash = self.type_ids.as_hash().unwrap();
        let mut ret = Vec::<(&str, i64)>::new();

        for iter in type_ids_hash.iter()
        {
            match iter.1["name"]["en"].as_str()
            {
                Some(name) =>
                {
                    if name.to_lowercase().contains(&query.to_lowercase())
                    {
                        ret.push((name, iter.0.as_i64().unwrap()));
                    }
                }
                None => (),
            }
        }
        ret
    }

    pub fn print_blueprints(&self)
    {
        println!("KNOWN BLUEPRINTS:");
        for iter in self.known_blueprints.iter()
        {
            println!("{}", iter.1);
        }
    }

    pub fn print_items(&self)
    {
        println!("KNOWN ITEMS:");
        for iter in self.known_items.iter()
        {
            println!("{}", iter.1);
        }
    }

    pub fn has_blueprint(&self, name: &str) -> bool
    {
        self.known_blueprints.contains_key(name)
    }

    pub fn has_item(&self, name: &str) -> bool
    {
        self.known_items.contains_key(name)
    }

    pub fn has_productionrun(&self, name: &str) -> bool
    {
        self.productionruns.contains_key(name)
    }
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item
{
    pub name: String,
    pub id: i64,
    pub buy_price: u64,
    pub sell_price: u64,
    pub produced: bool,
}

impl fmt::Display for Item
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} ({})\n  buy price: {}ISK\n  sell price: {}ISK\n  produced: {}",
            self.name, self.id, self.buy_price.to_formatted_string(&Locale::en), 
            self.sell_price.to_formatted_string(&Locale::en), self.produced)
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct T1Blueprint
{
    pub name: String,
    pub bp_id: i64,
    pub manufacturing_mats: Vec<(String, u64)>,
    pub material_research: u8,
    pub time_research: u8,
    pub produced_item: (String, i64),
}

impl T1Blueprint
{
    pub fn new(id: usize, material_research: u8, time_research: u8, db: &mut Database) -> T1Blueprint 
    {
        let name = db.type_ids[id]["name"]["en"].as_str().unwrap().to_string();
        let bp_id = id as i64;
        let mut manufacturing_mats = Vec::<(String, u64)>::new();
        let mats_vec = &db.blueprints[id]["activities"]["manufacturing"]["materials"]
                .as_vec().unwrap();

        for i in mats_vec.iter()
        {
            let quantity = i["quantity"].as_i64().unwrap();
            let id = i["typeID"].as_i64().unwrap();
            let name = db.type_ids[id as usize]["name"]["en"].as_str().unwrap();
            if db.known_items.contains_key(name) == false
            {
                let item = Item{name: name.to_string(), id, buy_price: 0, sell_price: 0,
                        produced: false};
                db.known_items.insert(name.to_string(), item);
                println!("{}", name);
                db.save_item(name);
            }
            manufacturing_mats.push((name.to_string(), quantity.try_into().unwrap()));
        }

        let produces = &db.blueprints[id]["activities"]["manufacturing"]["products"]
                .as_vec().unwrap();
        let id2 = &produces[0]["typeID"];
        let produced_name = db.type_ids[id2.as_i64().unwrap() as usize]["name"]["en"]
                .as_str().unwrap();
        if db.known_items.contains_key(produced_name) == false
        {
            let item = Item{name: produced_name.to_string(), id: id2.as_i64().unwrap(),
                buy_price: 0, sell_price: 0, produced: true};
            db.known_items.insert(produced_name.to_string(), item);
            db.save_item(&produced_name);
        }
        T1Blueprint {name, bp_id, manufacturing_mats, material_research, time_research,
            produced_item: (produced_name.to_string(),id2.as_i64().unwrap())}
    }
}

impl fmt::Display for T1Blueprint
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut message = format!("{} ({})\n  Material research: {}%\n  Time research: {}%\n",
            self.name, self.bp_id, self.material_research, self.time_research);

        message.push_str(&format!("  Manufacturing materials:\n"));
        for i in 0..self.manufacturing_mats.len()
        {
            message.push_str(&format!("    {}: {}\n", self.manufacturing_mats[i].0,
                self.manufacturing_mats[i].1));
        }

        write!(f, "{}", message)
    }
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct T1ProductionRun
{
    pub blueprint: String,
    pub materials: Vec<(String, u64)>,
    pub produces: String,
    pub jobruns: u64,
    pub installation_cost: u64,
}

impl T1ProductionRun
{
    pub fn new(blueprint: &str, jobruns: u64, installation_cost: u64, db: &mut Database) 
        -> T1ProductionRun
    {
        let bp = db.get_blueprint(blueprint).expect("Blueprint was not found");
        let produces = bp.produced_item.0.clone();
        let materials = bp.manufacturing_mats.clone();

        T1ProductionRun {blueprint: blueprint.to_string(), materials, produces,
            jobruns, installation_cost}
    }

    pub fn get_production_materials(&self, db: &Database) -> Vec<(String, u64)>
    {
        let mut ret = Vec::<(String, u64)>::new();
        let bp = db.get_blueprint(&self.blueprint).unwrap();

        for iter in self.materials.iter()
        {
            let count = ((iter.1 as f64 * self.jobruns as f64) / 100 as f64) 
                * (100 - bp.material_research as u64)as f64;
            ret.push((iter.0.clone(), count as u64));
        }
        ret
    }

    pub fn get_production_cost(&self, db: &Database) -> u64
    {
        let mut ret: u64 = 0;
        for iter in self.get_production_materials(db).iter()
        {
            let item = db.get_item(&iter.0).unwrap();
            ret += item.buy_price * iter.1
        }
        ret + self.installation_cost
    }

    pub fn get_sell_value(&self, db: &Database) -> u64
    {
        let item = db.get_item(&self.produces).unwrap();
        item.sell_price * self.jobruns
    }


}


pub fn load_resources(data_base_dir: &str) 
    -> (HashMap<String,T1Blueprint>, HashMap<String,Item>, HashMap<String, T1ProductionRun>)
{
    let mut known_blueprints = HashMap::<String,T1Blueprint>::new();
    let bp_path = &format!("{}/blueprints", data_base_dir);
    let bp_files = fs::read_dir(bp_path).unwrap();

    for path in bp_files
    {
        let file = File::open(path.unwrap().path()).unwrap();
        let reader = BufReader::new(file);
        let bp: T1Blueprint = serde_json::from_reader(reader).unwrap();
        known_blueprints.insert(bp.name.clone(), bp);
    }

    let mut known_items = HashMap::<String,Item>::new();
    let item_path = &format!("{}/items", data_base_dir);
    let item_files = fs::read_dir(item_path).unwrap();
    for path in item_files
    {
        let file = File::open(path.unwrap().path()).unwrap();
        let reader = BufReader::new(file);
        let bp: Item = serde_json::from_reader(reader).unwrap();
        known_items.insert(bp.name.clone(), bp);
    }

    let mut productionruns = HashMap::<String, T1ProductionRun>::new();
    let productionrun_path = &format!("{}/productionruns", data_base_dir);
    let productionrun_files = fs::read_dir(productionrun_path).unwrap();
    for path in productionrun_files
    {
        let file = File::open(path.unwrap().path()).unwrap();
        let reader = BufReader::new(file);
        let x: T1ProductionRun = serde_json::from_reader(reader).unwrap();
        productionruns.insert(x.blueprint.clone(), x);
    }
    (known_blueprints, known_items, productionruns)
}
