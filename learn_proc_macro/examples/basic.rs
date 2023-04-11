use learn_proc_macro::LearnProcMacro;

#[derive(LearnProcMacro)]
struct BasicStruct;

fn main() {
    BasicStruct::learn_proc_macro();
}
