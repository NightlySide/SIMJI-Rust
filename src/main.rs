use args::Args;
use vm::VM;
use clap::StructOpt;

mod args;
mod assembler;
mod vm;

fn main() {
    // on parse les arguments du programme
    let args = Args::parse();
    
    // on transforme le fichier en programme
    let lines = assembler::read_file(&args.path);
    let instrs = assembler::lines_to_instr(&lines);
    let program = assembler::instr_to_machine_code(&instrs);

    // on crée la vm et on la fait tourner
    let mut vm = VM::new(&program);
    vm.run();
}
