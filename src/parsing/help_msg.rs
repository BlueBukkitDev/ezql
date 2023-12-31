use colored::Colorize;

const Y: (u8, u8, u8) = (241, 194, 50);
const C: (u8, u8, u8) = (0, 198, 254);

/**
 Processes help commands based on user input. 
 */
pub fn process_help(args: &[&str]) {
    match args.len() {
        0 => send_base_help_msg(),
        1 => process_one(args),
        2 => process_two(args),
        _ => return,
    }
}

fn process_one(args: &[&str]) {
    match args[0] {
        "db" => send_db_help_msg(),
        "find" => return,
        "read" => return,
        "get" => return,
        "write" => return,
        "put" => return,
        _ => {
            let options = ["db", "find", "read", "get", "write", "put"];
            send_unknown_cmd("help", args, &options);}
    }
}

fn process_two(args: &[&str]) {
    match args[0] {
        "select" => send_db_select_help_msg(),
        "create" => send_db_create_help_msg(),
        "delete" | "destroy"=> send_db_delete_help_msg(),
        _ => {
            let options = ["select", "create", "delete"];
            send_unknown_cmd("help", args, &options);}
    }
}
////////////////////////////////////////////////////0-1 args
pub fn send_base_help_msg() {
    println!("DBMS {} help \n
    {} - used to provide help for a given command\n
    {} - used to interact with a database and it's properties\n
    {} - used to search for an entry in a selected database\n
    {} - used to input a new entry into a selected database\n
    {} - used to find tags or entry IDs\n
    {} - used to read property data from an entry\n
    {} - used to write property data to an entry", 
    "command".truecolor(C.0, C.1, C.2), 
    "help".truecolor(Y.0, Y.1, Y.2),
    "db".truecolor(Y.0, Y.1, Y.2),
    "get".truecolor(Y.0, Y.1, Y.2),
    "put".truecolor(Y.0, Y.1, Y.2),
    "find".truecolor(Y.0, Y.1, Y.2),
    "read".truecolor(Y.0, Y.1, Y.2),
    "write".truecolor(Y.0, Y.1, Y.2)
    );
}

fn send_db_help_msg() {
    println!("DBMS {} help", "db".truecolor(C.0, C.1, C.2));
    println!("db {} - used to create a new database", "create".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to delete an existing database", "delete".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to select a database for future queries", "select <name>".truecolor(Y.0, Y.1, Y.2));
}

fn _send_find_help_msg() {
    println!("DBMS {} help", "find".truecolor(C.0, C.1, C.2));
    println!("find {} - searches for up to a specified number of entries matching the query", "<number>".truecolor(Y.0, Y.1, Y.2));
    println!("find {} - searches for all entries matching the query", "*".truecolor(Y.0, Y.1, Y.2));
    println!("find 1 {} - initiates a tag list", "of".truecolor(Y.0, Y.1, Y.2));
    println!("find 1 of {} - the tag list to search for", "<tag>".truecolor(Y.0, Y.1, Y.2));//I think now that of and where are arbitrary and useless. Fluff. 
    println!("{}", "Examples".green().bold());
    println!("{}", "find all of <this is unfinished>".bright_magenta());
}

fn _send_read_help_msg() {

}

fn _send_get_help_msg() {

}

fn _send_write_help_msg() {

}

fn _send_put_help_msg() {

}
/////////////////////////////////////////////////////2 args
fn send_db_select_help_msg() {
    println!("DBMS {} help", "db select".truecolor(C.0, C.1, C.2));
    println!("db select {} - input the name of the parent folder containing generated db files", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

fn send_db_create_help_msg() {
    println!("DBMS {} help", "db create".truecolor(C.0, C.1, C.2));
    println!("db create {} - input the name you wish to use to reference your new database", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

fn send_db_delete_help_msg() {
    println!("DBMS {} help", "db destroy".truecolor(C.0, C.1, C.2));
    println!("db delete {} - input the name of the database you wish to delete", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

pub fn send_unknown_cmd(cmd:&str, args: &[&str], options: &[&str]) {//Unknown command! You typed "&7cmd sub &esub"; valid options are "&a<list>"
    let mut op_string = String::new();
    for option in options {
        op_string = op_string+option+", ";
    }
    op_string.truncate(op_string.len() - 2);

    let mut args_string = String::new();
    args_string = args_string+cmd+" ";
    for arg in args {
        args_string = args_string+arg+" "
    }
    println!("{} you typed {}; valid options are {}", "Unknown command!".red(), args_string.trim().truecolor(Y.0, Y.1, Y.2), op_string.truecolor(C.0, C.1, C.2));
}