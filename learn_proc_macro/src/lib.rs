// Re-export the `LearnProcMacro` derive macro from the `learn_proc_macro_derive` crate.
pub use learn_proc_macro_derive::LearnProcMacro;

pub trait LearnProcMacro {
    fn learn_proc_macro();
}
