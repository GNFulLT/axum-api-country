use std::{fs,path::Path, sync::{Arc, Mutex}, thread, time};

use sqlx::{Pool, Postgres};

use crate::config::config;

async fn pexec_thread_safe(str : &str,pool: &Pool<Postgres>,any_error: Arc<Mutex<bool>>)
{
    // Make string array
    let exec_strings = str.split("--||");

    // Iterate
    for exec_string in exec_strings {
        // Check should exec or not
        {
            let any_error_ref = any_error.lock().unwrap();
            // If it is true no need to continue return directly
            if *any_error_ref {
                return;
            }
            // Drop the lock here
            drop(any_error_ref);
        }
        let mut local_failed = false;
        // Execute
        let res = sqlx::query(exec_string).execute(pool).await.map_err(|e| {
            let mut ref_failed = any_error.lock().unwrap();
            *ref_failed = true;
            local_failed = true;
            e});

        match res {
            Err(err) => {
                println!("=> {:<20} Error occured while trying to execute psql command. Error message : \n{err}\n WHILE EXECUTING : \n{}","PSQL FATAL",exec_string);
                println!("=> {:<20} Error occured while trying to execute psql command. DEBUG INSPECT : \n{:?}","PSQL FATAL DEBUG",err);
            }
            Ok(data) => {
                println!("=> {:<20} psql command executed successfully with message : \n{:#?}","PSQL OK",data);
            }

        }

        if local_failed {
            return;
        }
    }
}
// To try tokio. Async may be unnecessary. 
pub async fn init_db(pool: Pool<Postgres>) {
    if !config::config().RESET_DB {
        println!("No need to reset db");
        return;
    }
    // Init order functions -> schemas (tables)
    let base_path = Path::new("./sql");
    
    let functions_path: std::path::PathBuf = base_path.join("functions");
    let schemas_path: std::path::PathBuf = base_path.join("schemas");

    if !functions_path.exists() || !schemas_path.exists()
    {
        panic!("ONE OF SQL PATH DOESN'T EXIST");
    }

    let any_failed = Arc::new(Mutex::new(false));


    // FIRST DROP TABLES
    let drop_tables = base_path.join("drop_tables.sql");
    let create_tables = base_path.join("create_tables.sql");

    let drop_tables_file = fs::read_to_string(drop_tables).unwrap_or_else(|_| panic!("Couldn't read file drop_tables.sql"));
    let create_tables_file = fs::read_to_string(create_tables).unwrap_or_else(|_| panic!("Couldn't read file create_tables.sql"));

    pexec_thread_safe(drop_tables_file.as_str(),&pool,any_failed.clone()).await;
    // THEN CREATE TABLES (If any error occured in drop table stage, it will return immediately because of any_failed)
    pexec_thread_safe(create_tables_file.as_str(),&pool,any_failed.clone()).await;

    // CHECK TABLES ARE OK
    {
        let failed = any_failed.lock().unwrap();
        if *failed{
            panic!("=> {:<20} Could not drop tables or create tables","FATAL");
        }
        drop(failed);
    }

    // INIT FUNCTIONS
    println!("=> {:<20} Executing FUNCTION TASKS","SQL EXECUTION LIST BEGINNING");
    let mut tasks : Vec<_> = functions_path.read_dir().expect("Functions path is not dir or an unexpected error occurred").map(|entry_res| {
        let pool2: Pool<Postgres> = pool.clone();
        let any_failed2 = any_failed.clone();
        let t = tokio::spawn(async move {
            if let Ok(entry) = entry_res {
                if entry.metadata().unwrap().is_file()
                {
                    let file_name: String = entry.file_name().to_str().unwrap().to_string();
                    if file_name.ends_with(".sql")
                    {
                        let read_file = fs::read_to_string(entry.path()).unwrap_or_else(|_| panic!("Couldn't read file {file_name}"));
                        println!("=> {:<20} Executing file : {file_name}","SQL EXECUTION BEGINNING");
                        pexec_thread_safe(read_file.as_str(),&pool2,any_failed2).await;
                        println!("=> {:<20} File executed with name {file_name}","SQL EXECUTION ENDED");
                    }
                }
            }
        });
        // To make sure insert process is started
        thread::sleep(time::Duration::from_millis(5));
        return t;
    }).collect();

    for function_task in tasks
    {
        let _ = function_task.await;
    }
    println!("=> {:<20} Executing FUNCTION TASKS","SQL EXECUTION LIST ENDED");


    println!("=> {:<20} Executing SCHEMA TASKS","SQL EXECUTION LIST BEGINNING");
    tasks = schemas_path.read_dir().expect("Schema path is not dir or an unexpected error occurred").map(|entry_res| {
        let pool2: Pool<Postgres> = pool.clone();
        let any_failed2 = any_failed.clone();
        tokio::spawn(async move {
            if let Ok(entry) = entry_res {
                if entry.metadata().unwrap().is_file()
                {
                    let file_name = entry.file_name().to_str().unwrap().to_string();
                    if file_name.ends_with(".sql")
                    {
                        let read_file = fs::read_to_string(entry.path()).unwrap_or_else(|_| panic!("Couldn't read file {file_name}"));
                        println!("=> {:<20} Executing file : {file_name}","SQL EXECUTION BEGINNING");
                        pexec_thread_safe(read_file.as_str(),&pool2,any_failed2).await;
                        println!("=> {:<20} File executed with name {file_name} and with response","SQL EXECUTION ENDED");
                    }
                }
            }
        })
        
    }).collect();

    for schema_task in tasks
    {
        let _ = schema_task.await;
    }
    println!("=> {:<20} Executing SCHEMA TASKS","SQL EXECUTION LIST ENDED");
    

    let failed = any_failed.lock().unwrap();
    if *failed{
        panic!("=> {:<20} There is one or more failed sql commands!","FATAL");
    }
}