use core::error;
use std::error::Error;
use std::iter;
use std::{clone, fmt::format, fs::read};
use std::{io::{stdin,stdout,Write}};
use rusqlite::{params, Connection, Statement};
use std::fs::{self, File, OpenOptions};



// crear un insert de la información


#[derive(Debug)]
struct Data{
    nombre:String,
    edad: i8,
    code_stud: i32,
    matricula:String
}

//función publica. Recibe 2 parametros -> (String,Connection)
//Obtiene Connection en forma de referencia
//Result<(),rusqlite::error> indica que se devuelve un () o rusqlite::error
pub fn show_info(
    data_where:String, 
    consult_db: &Connection,
    payload:String ) -> Result<(), rusqlite::Error>{

        let mut dat_found = false;
        //consulta indica que es de tipo mutable y usa la referencia de la conexión para hacer una consulta
        let mut consulta = consult_db.prepare(&payload)?;
            //info_row de tipo mutable indica que esta usando la variable consulta
            //para hacer uso de query_map() -> (1,2) donde -> (1 = indica el parametro de la consulta)
            //(2 = indica que |fila_query| usando Ok() hara uso de la estructura Data, la cuál contendra)
            //la información devuelta de la consulta
        consulta.query_map(params![data_where], |fila_query| Ok(Data{
            nombre: fila_query.get(0)?,
            edad: fila_query.get(1)?,
            code_stud: fila_query.get(2)?,
            matricula: fila_query.get(3)?
        }))?.for_each(|data_person| {
            dat_found = true;
            let info= {
                let drturn = format!("Nombre: {}\nEdad: {}\nMatricula: {}", 
                data_person.as_ref().unwrap().nombre,
                data_person.as_ref().unwrap().edad,
                data_person.as_ref().unwrap().matricula);
                drturn
            };
            println!("{}",info);
        });
        if !dat_found{println!("[-] La busqueda genero 0 resultados")}
        Ok(())
}

//crear una configuración 

pub fn parse_info_user(content:Vec<String>) -> std::io::Result<()>{
    let mut args_comm = String::new();
    const BCK:[&str;5] = ["DBD", "TABLE", "ROWSD", "SETTUPD", "TTIF"]; 
    content.iter().for_each(|filter:&String| { //itera sobre un Vector que contiene las variables de configuración
        let col_val:Vec<&str> = filter.split("=").collect(); //las divide usando = y obtiene el valor []
        if BCK.iter().any(|&filter| col_val.get(0).map_or(false,|indexs:&&str| indexs.contains(filter))){
            args_comm = format!("{}\n{}\n{}\n{}\n{}",content[0],content[1],content[2],content[3],content[4]);
        }else {
            println!("[-] Ingreso de variables invalidas.")
        }
    });
    if config_check("config2.txt".to_string()){
        let mut archv1 = File::open("config.txt")?;
        archv1.write_all(args_comm.as_bytes())?;
        println!("[**] Variables sobreescritas correctamente.")
    }else{
        let mut arch_file:File = File::create("config.txt")?;
        arch_file.write_all(args_comm.as_bytes())?;
        println!("[*+*] Variables escritas correctamente.")
    }
    Ok(())
}

fn config_check(name_file:String) -> bool{ //verifica si el archivo existe
    let arch:bool = match fs::metadata(name_file) {
        Ok(_) => true,
        Err(_) => false  
    };

    return arch;
}

//inserta un usuario
pub fn insert_info_per(connecting:&Connection, payload:String) -> Result<(),rusqlite::Error>{
    let resp:usize = connecting.execute(&payload, ()).expect("Error");
    if resp == 1 {println!("[+] INSERT VALIDO")} else {println!("[-] INSERT INVALIDO")} 
    Ok(())

}

//realiza un update a la base de datos, recibe 3 parametros
pub fn update_user_action(
    connecting:&Connection, 
    payload_updt:&str, 
    param:Vec<String>) //param indica que -> esta recibienddo los parametros (fila que afectara y a quien afectara)
    -> Result<usize, rusqlite::Error>{
        //UPDATE SET students = param[1] WHERE code = param[0];
        let update:usize = connecting.execute(payload_updt, params![param[1], param[0]])?;
        Ok(update)
}

pub mod action{
    use std::{io::{stdin, stdout,Write}};
    //interactua con el usuario
    pub fn inpt_user_upd(interac:(&str,&str)) -> Vec<String>{ //recibe una stuctura que contiene, el tipo a filtrar(1) y la interacción que muestra al usuario (0) 
        let mut raw_user:String = String::new();
        print!("{}", interac.0); 
        let _ = stdout().flush();
        stdin().read_line(&mut raw_user).expect("");
        return raw_user.as_str()
        .split(interac.1)// separa el texto del usuario
        .map(|stringf| stringf.to_string())// usa map para convertir a String
        .collect::<Vec<String>>(); //Convierte a un vecto de tipo string
    }  //se podria abreviar usando split().collect::<Vec<&str>>()
}



