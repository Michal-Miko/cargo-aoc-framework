use indoc::formatdoc;

fn capitalize_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

pub fn render_day_module(module_name: &str) -> String {
    let struct_name = capitalize_name(module_name);
    formatdoc! {r#"
        use std::path::PathBuf;
        
        use crate::BoxedError;
        use aoc_framework::{{traits::*, AocSolution, AocStringIter, AocTask}};
        
        pub struct {struct_name};
        
        impl AocTask for {struct_name} {{
            fn directory(&self) -> PathBuf {{
                "tasks/{module_name}".into()
            }}
        
            fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {{
                input.solved()
            }}
        }}
    "#}
}

pub fn render_main(module_names: &[String]) -> String {
    let struct_names = module_names.iter().map(|name| capitalize_name(name));
    let modules = module_names
        .iter()
        .map(|module| format!("mod {module};"))
        .collect::<Vec<_>>()
        .join("\n");
    let uses = module_names
        .iter()
        .zip(struct_names.clone())
        .map(|(module, r#struct)| format!("use {module}::{struct};"))
        .collect::<Vec<_>>()
        .join("\n");
    let boxes = struct_names
        .map(|r#struct| format!("{}Box::new({struct}),", " ".repeat(8)))
        .collect::<Vec<_>>()
        .join("\n");

    formatdoc! {r#"
        #![feature(lint_reasons)]
        #![expect(unused_variables)]

        {modules}

        {uses}

        use std::error::Error;

        use aoc_framework::{{check_solved_tasks, BoxedAocTask}};
        type BoxedError = Box<dyn Error + Send + Sync>;

        fn main() -> color_eyre::Result<()> {{
            color_eyre::install()?;

            let tasks: Vec<BoxedAocTask> = vec![
        {boxes}
            ];

            check_solved_tasks(tasks, 2)?;

            Ok(())
        }}
    "#}
}
