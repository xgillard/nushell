use super::super::SQLiteDatabase;
use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Spanned, SyntaxShape,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct OpenDb;

impl Command for OpenDb {
    fn name(&self) -> &str {
        "db open"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("query", SyntaxShape::Filepath, "SQLite file to be opened")
            .category(Category::Custom("database".into()))
    }

    fn usage(&self) -> &str {
        "Open a database"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["database", "open"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "",
            example: r#"""#,
            result: None,
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let path: Spanned<PathBuf> = call.req(engine_state, stack, 0)?;

        SQLiteDatabase::try_from_path(path.item.as_path(), path.span)
            .map(|db| db.into_value(call.head).into_pipeline_data())
    }
}
