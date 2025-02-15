use super::delimited::{from_delimited_data, trim_from_str};

use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Value};

#[derive(Clone)]
pub struct FromTsv;

impl Command for FromTsv {
    fn name(&self) -> &str {
        "from tsv"
    }

    fn signature(&self) -> Signature {
        Signature::build("from tsv")
            .switch(
                "noheaders",
                "don't treat the first row as column names",
                Some('n'),
            )
            .named(
                "trim",
                SyntaxShape::String,
                "drop leading and trailing whitespaces around headers names and/or field values",
                Some('t'),
            )
            .category(Category::Formats)
    }

    fn usage(&self) -> &str {
        "Parse text as .tsv and create table."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        from_tsv(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Create a tsv file with header columns and open it",
                example: r#"echo $'c1(char tab)c2(char tab)c3(char nl)1(char tab)2(char tab)3' | save tsv-data | open tsv-data | from tsv"#,
                result: None,
            },
            Example {
                description: "Create a tsv file without header columns and open it",
                example: r#"echo $'a1(char tab)b1(char tab)c1(char nl)a2(char tab)b2(char tab)c2' | save tsv-data | open tsv-data | from tsv -n"#,
                result: None,
            },
            Example {
                description: "Create a tsv file without header columns and open it, removing all unnecessary whitespaces",
                example: r#"echo $'a1(char tab)b1(char tab)c1(char nl)a2(char tab)b2(char tab)c2' | save tsv-data | open tsv-data | from tsv --trim all"#,
                result: None,
            },
            Example {
                description: "Create a tsv file without header columns and open it, removing all unnecessary whitespaces in the header names",
                example: r#"echo $'a1(char tab)b1(char tab)c1(char nl)a2(char tab)b2(char tab)c2' | save tsv-data | open tsv-data | from tsv --trim headers"#,
                result: None,
            },
            Example {
                description: "Create a tsv file without header columns and open it, removing all unnecessary whitespaces in the field values",
                example: r#"echo $'a1(char tab)b1(char tab)c1(char nl)a2(char tab)b2(char tab)c2' | save tsv-data | open tsv-data | from tsv --trim fields"#,
                result: None,
            },
        ]
    }
}

fn from_tsv(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let name = call.head;

    let noheaders = call.has_flag("noheaders");
    let trim: Option<Value> = call.get_flag(engine_state, stack, "trim")?;
    let trim = trim_from_str(trim)?;

    from_delimited_data(
        noheaders,
        '\t',
        trim,
        input,
        name,
        engine_state.get_config(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(FromTsv {})
    }
}
