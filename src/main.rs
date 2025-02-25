use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fs::read, io, vec};
use std::{io::{stdin,stdout,Write}};
use rusqlite::{params, Connection, Result};
mod complements;
use complements::{action, insert_info_per, parse_info_user, show_info, update_user_action};
mod inject;
use inject::Injecting as sqli;



//comandos -- input -- config 
fn main() {
    commands_line(env::args().collect());
}

//funcion para verificar la entrada de comandos
pub fn commands_line(args_env:Vec<String>){
    const COMMANDS:[&str;2] = ["input","config"];
    if COMMANDS.iter().any(|&com| args_env[1].contains(com)){ //com inidica el valor a comparar
        match args_env[1].as_str() {
        "input" => {
            input_layout();            
        },
        "config" => {
            parse_info_user(action::inpt_user_upd(("config> ", " ")));
            
        }
        _=> println!("[-] Argumento Invalido")
        }
    }else {
        println!("[-] Error argumento invalido -> (input,config)")
    }
}

fn input_layout() -> Result<()>{
    const LINE:[&str;4] = ["insert_info","show_info","update", "updt"];
    let parse_conf:ParseConfig = ParseConfig::pars_name_config(lecter_config().unwrap());
    let db_connect:Connection = Connection::open(parse_conf.db_namedconf())?;
    loop {
        let split_d:Vec<String> = action::inpt_user_upd(("invoice> "," "));
    
        if LINE.contains(&split_d[0].to_lowercase().as_str()){
            match split_d[0].to_lowercase().as_str(){
                "show_info" =>{
                    let c = sqli::form_payload_action(parse_conf.compl_inject(), "info");
                    let _ = show_info(split_d[1].to_string(), &db_connect, c.payload_sql());
                    break;
                },
                "update"|"updt" => {
                    let commd_ejecute:Vec<String> = vec![split_d[1].to_string(),split_d[2].to_string()]; 
                    let inject:usize = update_user_action(
                        &db_connect,
                        sqli::form_payload_action(parse_conf.compl_inject(), "update").payload_sql().as_str(), 
                        commd_ejecute)?;
                    if inject == 1 {println!("[+] UPDATE VALIDO")} else {println!("[-] UPDATE INVALIDO")};

                }
                "insert_info" => {
                    let mut finaladd_vec:Vec<String> = parse_conf.insert_ittf();
                    finaladd_vec.push(parse_conf.compl_inject()[0].to_string());
                    finaladd_vec.push(split_d[1].to_string());
                    //sqli::form_payload_action(finaladd_vec, "insert").payload_sql();
                    insert_info_per(&db_connect, sqli::form_payload_action(finaladd_vec, "insert").payload_sql());
                    //println!("{}",sqli::form_payload_action(finaladd_vec, "insert").payload_sql());

                }
                _ => println!("[-] ERROR COMMANDO INVALIDO")
            }
        }else {
            println!("[X] COMANDO NO ENCONTRADO")
        }
    }
    Ok(())
    
}

fn lecter_config() -> std::io::Result<HashMap<String,String>>{
    let op_file:File = File::open("config.txt")?;
    let mut map_config: HashMap<String,String> = HashMap::new();
    let buffereding:BufReader<File> = BufReader::new(op_file);
    for line in buffereding.lines(){
        let vrt:Vec<&str> = line.as_ref().unwrap().split("=").collect();
        if vrt.len() == 2{
            map_config.insert(String::from(vrt[0]), String::from(vrt[1]));
        }
    }
    Ok(map_config)
}

#[warn(dead_code)]
struct ParseConfig{
    db_use: String,
    table_name:String,
    rows:String,
    set_upd: String,
    ttif:String
}

impl ParseConfig{
    //retorna el nombre de la base de datos
    fn db_namedconf(&self) 
    -> String
    {
        return self.db_use.to_string();
    }

    fn compl_inject(&self) -> Vec<String>{ // crea un vector apartir de la configuración
        let mut sql_vector:Vec<String> =  Vec::with_capacity(2);
        sql_vector.push(self.table_name.to_string());
        sql_vector.push(self.rows.to_string());
        sql_vector.push(self.set_upd.to_string());
        return sql_vector;

    }

    //Obtiene los nombres de las columnas  
    fn insert_ittf(&self) -> Vec<String>{
        let parts_commn:Vec<String> = self.ttif.split(",").map(|s| s.to_string()).collect();
        return parts_commn;
    }

    fn pars_name_config(config_dict:HashMap<String,String>) -> ParseConfig{
        ParseConfig { //obtiene los paramatros de la configuración
            db_use: config_dict.get("DBD").unwrap().trim_matches(&['[', ']', '\''][..]).to_string(),
            table_name: config_dict.get("TABLE").unwrap().trim_matches(&['[', ']', '\''][..]).to_string(),
            rows: config_dict.get("ROWSD").unwrap().trim_matches(&['[', ']', '\''][..]).to_string(),
            set_upd: config_dict.get("SETTUPD").unwrap().trim_matches(&['[', ']', '\''][..]).to_string(),
            ttif:config_dict.get("TTIF").unwrap().trim_matches(&['[', ']', '\''][..]).to_string()
        }
    }
    
}