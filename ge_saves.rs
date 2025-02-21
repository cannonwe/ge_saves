use std::fs;
use std::fs::File;
use std::io::{stdin,stdout,Read,Write};

fn main() {
    let version = "1.0";//update this if you make a change you intend to distribute!!!
    println!("\n\n\t\t*~*~Goldeneye Savefile Checker~*~*\t{version}\n");
    let backup = "Save/Backup/GOLDENEYE.eep";
    loop {
        println!("\t q - Quit");
        println!("\t a - Check & Fix all savefiles");
        println!("\t 1 - 60 Goldeneye 007");
        println!("\t 2 - 60 BMW's Map Pack");
        println!("\t 3 - 60 Modern Multiplayer");
        println!("\t 4 - 60 Netplay LTK Cup Edition 1.2");
        println!("\t 5 - 60 NGPA 1.0 Blockfort");
        println!("\t 6 - 60 NSNA");
        println!("\t 7 - 60 Starting Weapons");
        println!("\t 8 - 60 Tournament Edition 1.04");
        println!("\t 9 - 60 Zoinkity Map Pack");
        println!("\t10 - Goldeneye X");
        println!("\t11 - Goldfinger");
        println!("\t12 - Perfect Dark");
        println!("\t13 - Vanilla Goldeneye");
        print!("\n\t\t>> ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid input.");
        let x = input.trim();
        match x {
            "q" => break,
            "a" => check_all(),
            "1" => {
                let savefile = "Save/GOLDENEYE-3E450D6358A41D9132CBA43C3CE86C46/GOLDENEYE.eep";
                handler(backup, savefile);
                },
            "2" => {
                let savefile = "Save/GOLDENEYE-CE724EA9F4621841073FAD7717D6A833/GOLDENEYE.eep";
                handler(backup, savefile);
                },
            "3" => {
                let savefile = "Save/GOLDENEYE-5E9BAD8AC489171DFF07EB566FD63C80/GOLDENEYE.eep";
                handler(backup, savefile);
                },
            "4" => {
                let savefile = "Save/GOLDENEYE-1BCD697FAB685E39FBD133E32C50F5D3/GOLDENEYE.eep"; 
                handler(backup, savefile);
                },
            "5" => {
                let savefile = "Save/GOLDENEYE-350DEFFF84BF9CEC837B297A87B0C683/GOLDENEYE.eep"; 
                handler(backup, savefile);
            },
            "6" => {
                let savefile = "Save/GOLDENEYE-B632BD9126165741A2D401370EA97C13/GOLDENEYE.eep"; 
                handler(backup, savefile);
                },
            "7" => {
                let savefile = "Save/GOLDENEYE-078F9FFEB68B45DB41EA2CD70CBD42F2/GOLDENEYE.eep"; 
                handler(backup, savefile);
                },
            "8" => {
                let savefile = "Save/GOLDENEYE-8462B5E86BEF179FC3703E1C6F255653/GOLDENEYE.eep"; 
                handler("Save/Backup/TE/GOLDENEYE.eep", savefile);
                },
            "9" => {
                let savefile = "Save/GOLDENEYE-A2FCD8529FC93EC42BFC08CCA47D71A4/GOLDENEYE.eep"; 
                handler(backup, savefile);
                },
            "10" => {
                let savefile = "Save/GoldenEye X-E03DA6B473C2FC27251F7BE960DE1018/GoldenEye X.eep"; 
                handler("Save/Backup/GoldenEye X.eep", savefile);
                },
            "11" => {
                let savefile = "Save/GOLDFINGER-EC82FC8931C1F526E014BCC529411516/GOLDFINGER.eep"; 
                handler("Save/Backup/GOLDFINGER.eep", savefile);
                },
            "12" => {
                let savefile = "Save/Perfect Dark-ECACDBDC93C3087627A775DCC16AEC7A/Perfect Dark.eep";
                handler("Save/Backup/Perfect Dark.eep",savefile);
                },
            "13" => {
                let savefile = "Save/GOLDENEYE-19060E240FFEA605ED18356CF6B14A42/GOLDENEYE.eep";
                handler(backup, savefile);
                },
              _ => println!("Invalid input."),
        };
    }
    println!("\nIt's KLOBBerin' time!");
}

fn check_all() {
    let backup = "Save/Backup/GOLDENEYE.eep";
    handler(backup,"Save/GOLDENEYE-3E450D6358A41D9132CBA43C3CE86C46/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-CE724EA9F4621841073FAD7717D6A833/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-5E9BAD8AC489171DFF07EB566FD63C80/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-1BCD697FAB685E39FBD133E32C50F5D3/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-350DEFFF84BF9CEC837B297A87B0C683/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-B632BD9126165741A2D401370EA97C13/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-078F9FFEB68B45DB41EA2CD70CBD42F2/GOLDENEYE.eep");
    handler("Save/Backup/TE/GOLDENEYE.eep","Save/GOLDENEYE-8462B5E86BEF179FC3703E1C6F255653/GOLDENEYE.eep");
    handler(backup,"Save/GOLDENEYE-A2FCD8529FC93EC42BFC08CCA47D71A4/GOLDENEYE.eep");
    handler("Save/Backup/GoldenEye X.eep","Save/GoldenEye X-E03DA6B473C2FC27251F7BE960DE1018/GoldenEye X.eep");
    handler("Save/Backup/GOLDFINGER.eep","Save/GOLDFINGER-EC82FC8931C1F526E014BCC529411516/GOLDFINGER.eep");
    handler("Save/Backup/Perfect Dark.eep","Save/Perfect Dark-ECACDBDC93C3087627A775DCC16AEC7A/Perfect Dark.eep");
    handler(backup,"Save/GOLDENEYE-19060E240FFEA605ED18356CF6B14A42/GOLDENEYE.eep");
    println!("\n\nAll files OK.");
}

fn handler(backup: &str, savefile: &str) {
    if check_save(backup, savefile) {
        println!("No further action needed.\n");
        return;
    }
    restore_save(backup,savefile);
    if check_save(backup, savefile) {
        println!("No further action needed.\n");
        return;
    }
    println!("\nUnknown Error - Whoopsie-daisy!");
}

fn restore_save(backup: &str, savefile: &str) {
    fs::copy(backup, savefile).unwrap();
    println!("Restored save from backup '{}'.",backup);
}

fn check_save(backup: &str, savefile: &str) -> bool {
    let backup_bytes = get_file_as_byte_vec(&backup);
    let save_bytes = get_file_as_byte_vec(&savefile);
    if backup_bytes == save_bytes {
        println!("Savefile '{}' OK", savefile);
        return true;
    }
    println!("Savefile mismatch!");
    return false;
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("Save file not found.");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    buf
}
