use bytes::Bytes;
use ckb_vm::{
    machine::SupportMachine, DefaultCoreMachine, DefaultMachineBuilder, Memory, Register,
    SparseMemory, WXorXMemory,
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    bin: PathBuf,
}

fn main() {
    let opts = Opts::from_args();

    let mut file = File::open(&opts.bin).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = Bytes::from(buffer);

    run_vm::<u64, SparseMemory<u64>>(&buffer, &[]);
}

pub fn run_vm<R: Register, M: Memory<R> + Default>(program: &Bytes, args: &[Bytes]) -> i8 {
    let core_machine = DefaultCoreMachine::<R, WXorXMemory<R, M>>::default();
    let builder = DefaultMachineBuilder::new(core_machine);
    let mut machine = builder.build();
    machine.load_program(program, args).unwrap();
    let exit_code = machine.run().unwrap();
    let consumed_cycles = machine.cycles();
    println!(
        "contract exit with {}, consume cycles {}",
        exit_code, consumed_cycles
    );
    exit_code
}
