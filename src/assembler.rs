use std::{fs, collections::HashMap, process::exit};
use phf::phf_map;

// liste des op_codes disponibles
static OP_CODES: phf::Map<&'static str, u32> = phf_map! {
    "stop" => 0, "add" => 1,
	"sub" => 2, "mul" => 3,
	"div" => 4, "and" => 5,
	"or" => 6, "xor" => 7,
	"shl" => 8, "shr" => 9,
	"slt" => 10, "sle" => 11,
	"seq" => 12, "load" => 13,
	"store" => 14, "jmp" => 15,
	"braz" => 16, "branz" => 17,
	"scall" => 18,
};

// lis le contenu d'un fichier et récupère les instructions ainsi que les
// labels dans un vecteur de String
pub fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path)
        .expect(&format!("Le fichier {} est introuvable.", path));

    // itération sur les lignes du fichier
    let lines: Vec<String> = content
        .split("\n")
        // on retire les espaces en trop
        .map(|line| line.trim()) 
        // on retire les lignes vides
        .filter(|line| line.ne(&"")) 
        // on retire les lignes commentaires
        .filter(|line| line.chars().next().unwrap() != ';')
        // on retire enfin les commentaires dans les lignes 
        .map(|line| line.split(";").next().unwrap()
                            .trim()
                            .to_string()) // on retire le commentaire de la ligne
        .collect();

    lines
}

// lis les lignes d'un fichier traité et retourne un dictionnaire
// des adresses correspondantes aux labels
pub fn get_label_adresses(lines: &Vec<String>) -> HashMap<String, u32> {
    let mut addresses: HashMap<String, u32> = HashMap::new();
    let mut index: u32 = 0;

    lines.iter().for_each(|line| {
        // si c'est un label
        if line.chars().last().unwrap().eq(&':') {
            let mut label = line.clone();
            label.pop();

            addresses.insert(label, index);
        } 
        // sinon on incrémente juste l'index
        else { index += 1; }
    });

    addresses
}

fn split_instruction(instruction: &String) -> (String, Option<Vec<String>>) {
    let data: Vec<&str> = instruction.split(" ").collect();
    
    let op_name = data[0].to_string();
    let mut args: Option<Vec<String>> = None;
    
    // si il y a des arguments on les ajoute
    if data.len() > 1 {
        let args_vec: Vec<String> = data
            .into_iter().skip(1)
            .collect::<Vec<&str>>().join("")
            .split(",").map(|arg| arg.to_string()).collect();
        args = Some(args_vec);
    }

    (op_name, args)
}

fn parse_argument(argument: &str, labels: &HashMap<String, u32>) -> (u32, bool) {
    // on retire le "r" du registre si il est présent
    if argument.chars().next().unwrap().eq(&'r') {
        // on essaie de parser l'argument
        let value = argument.chars().skip(1).collect::<String>().parse::<u32>()
            .expect(&format!("Le numéro de registre: '{}' n'est pas un numéro", argument));
        return (value, true);
    }

    // si c'est un label on retourne son adresse
    if labels.contains_key(argument) { return (labels[argument], false); }

    // on essaie de parser l'argument
    let value = argument.parse::<u32>()
        .expect(&format!("Erreur en tentant de parser l'imm : {}", argument));
    (value, false)
}

pub fn lines_to_instr(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut instructions: Vec<Vec<u32>> = Vec::new();

    let mut has_stop: bool = false;
    let mut pc = 0;
    let labels = get_label_adresses(lines);

    for line in lines.iter() {
        // skip if the line is a label
        if line.chars().last().unwrap().eq(&':') { continue; }

        let (op_name, args_opt) = split_instruction(line);
        if !OP_CODES.contains_key(&op_name) {
            println!("Instruction non reconnue: {}", op_name);
            exit(1);
        }
        if op_name.eq("stop") { has_stop = true; }
        let op_code = *OP_CODES.get(&op_name).unwrap();

        // vecteur des codes d'instruction
        let mut num_instr: Vec<u32> = Vec::new();
        num_instr.push(op_code);

        if let Some(args) = args_opt {
            match args.len() {
                1 => {
                    let (value, _) = parse_argument(&args[0], &labels);
                    num_instr.push(value);
                },
                2 => {
                    if op_name == "jmp" {
                        let (value, is_reg) = parse_argument(&args[0], &labels);
                        let res = if !is_reg { 1 } else { 0 };

                        num_instr.push(res);
                        num_instr.push(value);
                    } else {
                        let (value, _) = parse_argument(&args[0], &labels);
                        num_instr.push(value);

                    }

                    let (value, _) = parse_argument(&args[1], &labels);
                    num_instr.push(value);
                },
                3 => {
                    // r1
                    let (value, _) = parse_argument(&args[0], &labels);
                    num_instr.push(value);

                    // o
                    // 0 si un registre, 1 si valeur immédiate
                    let (value, is_reg) = parse_argument(&args[1], &labels);
                    let imm = if !is_reg { 1 } else { 0 };

                    num_instr.push(imm);
                    num_instr.push(value);

					// r2
                    let (value, _) = parse_argument(&args[2], &labels);
                    num_instr.push(value);
                }
                _ => {
                    println!("Mauvais nombre d'arguments (#{}) : {}", args.len(), line);
                }
            }
        }

        println!("{}: {:?} | {}", pc, num_instr, line);

        instructions.push(num_instr);
        pc += 1;
    }

    if !has_stop {
        println!("Le programme n'a pas d'instruction 'stop'. Cela peut causer un comportement imprévu.");
    }

    instructions
}

pub fn instr_to_machine_code(instructions: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut dec_instructions: Vec<u32> = Vec::new();

    for (pc, instr) in instructions.iter().enumerate() {
        let mut dec_instr = instr[0] << 27;

        match instr.len() {
            1 => {},
            2 => {
                // scall
                dec_instr += instr[1] // num
            },
            3 => {
                // braz
                dec_instr += instr[1] << 22; // reg
                dec_instr += instr[2];       // address
            },
            4 => {
                // jmp
                dec_instr += instr[1] << 26;    // imm
                dec_instr += instr[2] << 5;     // o
                dec_instr += instr[3];          // r
            }
            5 => {
                // add, load, store
                dec_instr += instr[1] << 22;    // reg
                dec_instr += instr[2] << 21;    // imm
                dec_instr += instr[3] << 5;     // o
                dec_instr += instr[4];          // reg
            }
            _ => {}
        }

        dec_instructions.push(dec_instr);
        println!("pc: {} | hex: 0x{:08x}", pc, dec_instr);
    }

    dec_instructions
}