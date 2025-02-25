use std::{env::join_paths, error, fmt::format, vec::Vec};

pub struct Injecting{
    payload:String
}
//creamos una implementación llamada igual que la estructura
impl Injecting{
    //metodo principal retorna la carga sql
    pub fn payload_sql(&self) -> String{
        return self.payload.to_string();
    }
    
    //creación del payload -> recibe 2 parametros
    pub fn form_payload_action(complements_sql:Vec<String>, confirm:&str) -> Injecting{
        let mut carga_sql:String = String::new();
        match confirm { //confirm indica el tipo de payload que va crear
            "info" => {
                let create_sql:String = format!("SELECT * FROM {} WHERE {} = ?1",complements_sql[0],complements_sql[1]);
                carga_sql = create_sql;
            },
            "update" => {
                let create_update:String = format!("UPDATE {} SET {} = ?1 WHERE {} = ?2", complements_sql[0],complements_sql[2], complements_sql[1]); 
                carga_sql = create_update;           
            }
            "insert" => {
                let parts_info:Vec<String> = complements_sql.clone();
                let valores_insert:String= parts_info[5].split(",").map(|itr_vl| {
                    if itr_vl.trim().chars().all(|chr| chr.is_numeric()){
                        return itr_vl.to_string(); //todas las ramas tiene que retornar el mismo valor
                    }else {
                        return format!("'{}'",itr_vl);
                    }
                }).collect::<Vec<String>>().join(","); //Verifica que los valores de tipo String esten entre comillas simples
                                                           // si no se usa las comillas simples, entonces sqlite toma como columna el valor a insertar
                                                           //eso genera un error, valores_insert evita ese error  
                carga_sql = format!("INSERT INTO {}({},{},{},{}) VALUES ({})",parts_info[4],parts_info[0],parts_info[1],parts_info[2],parts_info[3],valores_insert);
            }
            _ => println!("[-] ARGUMENTOS INCORRECTOS")
        };
        Injecting{ //
            payload:carga_sql
        }
    }
}