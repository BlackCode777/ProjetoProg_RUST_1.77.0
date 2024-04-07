
use std::collections::HashMap;

struct Conection {
    driver: String,
    str_conection: String
}

fn main(){

    let mut config_db: HashMap<&str, Conection> = HashMap::new();

    config_db.insert("string_postgres", Conection{
        driver: String::from("Postgres"),
        str_conection: "Server=IP address;Port=5432;Database=database_name;User Id=user_name;Password=password;".to_string()
    });

    config_db.insert("string_sqlserver", Conection{
        driver: String::from("SQL Server"),
        str_conection: "Server=IP address;Port=1433;Database=database_name;User Id=user_name;Password=password;".to_string()
    });

    for (key, value) in &config_db {
        println!("{}: Driver: {}, Str: {}", key, value.driver, value.str_conection);
    }

}