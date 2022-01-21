extern crate dotenv;
use dotenv::dotenv;
use std::{env};

pub fn check_env() -> bool {
    let mut used_positions = vec![];
    dotenv().ok();
    for (key, val) in env::vars() {
        if !check_env_var(&key, &val, &used_positions) {return false;}
        if is_position_path(&key) && val != "-1" {
            used_positions.push(val);
        }
    }
    true
}

pub fn print_env(){
    let all_var_names = ["PIPE".to_string(), "ELBOW".to_string(), "TEE".to_string(), "PIPE_PREFIX".to_string(), "SPACE_PREFIX".to_string(), "SHOW_FILE_METADATA".to_string(), "SHOW_DIR_METADATA".to_string(), "SPACING".to_string(), "TREE_LAYER_LIMIT".to_string(), "FILE_TIME_TYPE".to_string(), "FILE_TIME_FORMAT".to_string(), "FILE_EXTENSION_STYLE".to_string(), "FILE_PERMS_STYLE".to_string(), "FILE_OWNER_STYLE".to_string(), "FILE_SIZE_STYLE".to_string(), "FILE_TIME_STYLE".to_string(), "FILE_NAME_STYLE".to_string(), "DIR_NAME_STYLE".to_string(), "DASH_COLOR".to_string(),  "EXECUTE_COLOR".to_string(),  "WRITE_COLOR".to_string(), "READ_COLOR".to_string(), "SOCKET_COLOR".to_string(), "BLOCKD_COLOR".to_string(), "CHARD_COLOR".to_string(), "PIPE_COLOR".to_string(), "PATH_COLOR".to_string(), "SYMLINK_COLOR".to_string(), "DIR_COLOR".to_string(), "FILE_EXTENSION_COLOR".to_string(), "FILE_PERMS_COLOR".to_string(), "FILE_OWNER_COLOR".to_string(), "FILE_SIZE_COLOR".to_string(),"FILE_TIME_COLOR".to_string(), "FILE_NAME_COLOR".to_string(), "DIR_NAME_COLOR".to_string(), "FILE_EXTENSION_POSITION".to_string(), "FILE_TIME_POSITION".to_string(), "FILE_PERMS_POSITION".to_string(), "FILE_OWNER_POSITION".to_string(), "FILE_SIZE_POSITIONs".to_string()];
    for (key, val) in env::vars() {
        if all_var_names.contains(&key){
            println!("{} = {}", key, val);
        }
    }
}

pub fn get_used_positions() -> Vec<String> {
    let mut used_positions = vec![];
    for (key, val) in env::vars() {
        if is_position_path(&key) && val != "-1"{
            used_positions.push(val);
        }
    }
    return used_positions;
}

pub fn check_env_var(key: &str, val: &str, used_positions: &Vec<String>) -> bool{
    let all_var_names = ["PIPE".to_string(), "ELBOW".to_string(), "TEE".to_string(), "PIPE_PREFIX".to_string(), "SPACE_PREFIX".to_string(), "SHOW_FILE_METADATA".to_string(), "SHOW_DIR_METADATA".to_string()];
    let all_colors = ["BLACK".to_string(), "BLUE".to_string(), "CYAN".to_string(), "GREEN".to_string(), "LIGHTBLACK".to_string(), "LIGHTBLUE".to_string(), "LIGHTCYAN".to_string(), "LIGHTGREEN".to_string(), "LIGHTMAGENTA".to_string(), "LIGHTRED".to_string(), "LIGHTWHITE".to_string(), "LIGHTYELLOW".to_string(), "MAGENTA".to_string(), "RED".to_string(), "WHITE".to_string(), "YELLOW".to_string()];
    let all_styles = ["BOLD".to_string(), "UNDERLINE".to_string(), "DIMMED".to_string(), "ITALIC".to_string(), "BLINK".to_string(), "REVERSE".to_string(), "HIDDEN".to_string(), "STRICKEN".to_string(), "NORMAL".to_string()];
    let all_time_formats = ["CREATED".to_string(), "MODIFIED".to_string(), "ACCESSED".to_string()];
    let file_detail_num = 5;

    if key != "SPACE_PREFIX" && all_var_names.contains(&key.to_string()) && val.len() == 0 {
        println!("ERROR: Invalid ENV variable with key {}. ENV variable must have a value", key);
        return false;
    }

    if is_color_path(&key) {
        if !all_colors.contains(&val.to_uppercase()){
            println!("ERROR: ENV variable with invalid color name. {} for variable {} is not a valid color!", val, key);
            return false;
        }

        if is_valid_rgb(&val.to_uppercase(), &key) {
            println!("ERROR: ENV variable with invalid RGB value for color. {} for variable {} is not a valid RGB value!", val, key);
            return false;
        }
    }

    if is_style_path(&key) {
        if !all_styles.contains(&val.to_uppercase()){
            println!("ERROR: ENV variable with invalid style name. {} for variable {} is not a valid style!", val, key);
            return false;
        }
    }

    if is_metadata_path(&key) {
        if &val.to_uppercase() != "TRUE" &&  &val.to_uppercase() != "FALSE" {
            println!("ERROR: ENV variable with invalid metadata name. {} for variable {} is not a valid variable! It must be either TRUE or FALSE", val, key);
            return false;
        }
    }

    if is_limit_path(&key) {
        let key_int: i32 = val.parse::<i32>().ok().expect("INVALID integer for TREE_LAYER_LIMIT in env variable!");
        if key_int <= 0 {
            println!("ERROR: ENV variable with invalid tree layer limit. {} for variable {} is not a valid variable! It must be greater than 0", val, key);
            return false;
        }

        if key_int > 7 {
            println!("ERROR: ENV variable with invalid tree layer limit. {} for variable {} is not a valid variable! It must be less than 8", val, key);
            return false;
        }
    }

    if key == "FILE_TIME_TYPE" {
        if !all_time_formats.contains(&val.to_uppercase()){
            println!("ERROR: ENV variable with invalid time type. {} for variable {} is not a valid time type! Valid time types are CREATED or MODIFIED", val, key);
            return false;
        }
    }

    if key == "SHOW_SHORT" {
        if val.to_uppercase() != "TRUE" && val.to_uppercase() != "FALSE"{
            println!("ERROR: ENV variable with invalid show short type. {} for variable {} is not a valid show short type! Valid types are TRUE or FALSE", val, key);
            return false;
        }
    }

    if key == "SPACING" {
        let key_int: i32 = val.parse::<i32>().ok().expect("INVALID integer for TREE_LAYER_LIMIT in env variable!");
        if key_int < 0 {
            println!("ERROR: ENV variable with invalid spacing amount. {} for variable {} is not a valid spacing! Spacing must be greater than or equal to 0", val, key);
            return false;
        }

        if key_int > 7 {
            println!("ERROR: ENV variable with invalid spacing amount. {} for variable {} is not a valid spacing! Spacing must be less than 7", val, key);
            return false;
        }
    }

    if is_position_path(&key) {
        let key_int: i32 = val.parse::<i32>().ok().expect("INVALID integer in env variable!");
        if (key_int <= 0 && key_int != -1) || key_int > file_detail_num {
            println!("ERROR: ENV variable with invalid position range. Position {} for variable {} is out of range! Position should be -1, 1, 2, 3, 4, or 5!", val, key);
            return false;
        }
        if used_positions.contains(&key.to_string()) {
            println!("ERROR: ENV variable with invalid position. Position {} for variable {} has already been used! Please consider giving it a different position", val, key);
            return false;
        }
    }
    return true;
}

fn is_color_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"COLOR");
}

fn is_metadata_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"METADATA");
}

fn is_valid_rgb(color: &str, path: &str) -> bool {
    let uppercased_no_space: String = color.to_uppercase().replace(" ", "");
    if &uppercased_no_space[..4] != "RGB(" || &uppercased_no_space[&uppercased_no_space.len()-1..] != ")" {return false};
    let substring: &str = &uppercased_no_space[4..uppercased_no_space.len()];
    let split_values: Vec<&str> = substring.split(",").collect();
    if split_values.len() != 3 { return false };
    for value in split_values {
        let rgb_number : i32 = value.parse::<i32>().ok().expect("Invalid RGB number value in env");
        if rgb_number < 0 || rgb_number > 255 {
            println!("INVALID RGB number value of {} for {}", rgb_number, path);
            return false;
        }
    }
    return true;
}

fn is_style_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"STYLE");
}

fn is_position_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"POSITION");
}

fn is_limit_path(path: &str) -> bool {
    let string_vec: Vec<&str> = path.split("_").collect();
    return string_vec.contains(&"LIMIT");
}