mod interpreter;
fn main() {
    let mut interp = interpreter::Interpreter {
        registers: [Default::default(); 20],
        memory: vec![],
        zmemory: vec![],
        pc: 0,
        flags: interpreter::Flags {
            zero: 0,
            sign: 0,
            greater: 0,
        },
    };
    //interp.interpret();
}
