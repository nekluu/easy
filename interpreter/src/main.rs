mod interpreter;
fn main() {
    let mut interp = interpreter::Interpreter {
        registers: [Default::default(); 20],
        memory: [0; 65536],
        zmemory: [0; 300],
        pc: 0,
        flags: interpreter::Flags {
            zero: 0,
            sign: 0,
            greater: 0,
        },
    };
    //interp.interpret();
}
