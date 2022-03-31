use std::fs::{self, File};
use std::io::{self,Write};
use std::io::{BufRead, BufReader};

fn main() {

    load_menu();
}

fn input(){
    let input = get_input();
    match input.as_str() {
        "1" => create_file(),
        "2" => append_to_file(),
        "3" => load_file(),
        "4" => exit_app(),
        _ => exit_app()
    }
}

fn get_input() -> String{
    let mut input = String::new();
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("flush failed");
    match io::stdin().read_line(&mut input){
        Ok(_) => String::from(input.trim()),
        Err(_) => String::from("Error, Wrong Input")
    }
}

fn load_menu(){
    println!("####SIMPLE DATABASE####");
    println!("---MENU---");
    print!(" 1)Create new Data \n 2)Add to file \n 3)Load and Display data \n 4)Exit \n\n");

    input();

}

fn exit_app(){
    println!("Shuting down");
}
fn append_to_file(){
    print!("Name file to add data to");
    let file_name = get_input();
    let file = fs::OpenOptions::new().append(true).open(&file_name);

    match file{
        Ok(mut file) => {
            print!("ADD DATA TO {}",&file_name);
            let user_input = format!("\n {} \n",get_input());

            match file.write_all(user_input.as_bytes()){
                Ok(_) => {
                    println!("Data has been writen into the file !")
                },
                Err(_) => {
                    println!("ERROR: Can't write into file")
                }
            };
            load_menu();
        },
        Err(_) => {
            println!("ERROR CAN'T OPEN {}",&file_name);
            load_menu();
        }
    }
}
fn create_file(){

    print!("File name ");
    let file_name = get_input();
    let file = File::create(&file_name);

    match file{
        Ok(mut file) => {
            println!("##### CREATING FILE  {} #####", &file_name);

            let user_input = get_input();
            match file.write_all(user_input.as_bytes()){
                Ok(_) => {
                    println!("Data has been writen into the file !")
                },
                Err(_) => {
                    println!("ERROR: Can't write into file")
                }
            };
            load_menu();
        },
        Err(e) => {
            println!("{}",e);
            println!("\n ####### ERROR CAN'T CREATE FILE ####### \n");
            load_menu();
        }
    };
}

fn load_file(){

    print!("File to load ");
    let file_name = get_input();
    let file = File::open(&file_name);

    match file{
        Ok(f) => {
            let reader = BufReader::new(f);

            println!("##### DISPLAYING DATA FROM FILE {} #####", &file_name);
            for (_,line) in reader.lines().enumerate(){
                let line = line.unwrap();
                println!("{}",line);
            }

            println!("\n ");
            load_menu();
        },
        Err(_) => {
            println!("\n ####### ERROR CAN'T LOAD FILE ####### \n");
            load_menu();
        }
    }
}
