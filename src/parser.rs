pub mod semantic_checker;
pub mod syntax_checker;

pub fn parser(
    file: String,
) -> Result<Vec<i32>, syntax_checker::ParseError>
{
    let syntax = syntax_checker::parser(file);
    match syntax {
        Ok(s) => {
            match semantic_checker::semantic_checker(&s.0, &s.1) {
                Err(e) => return Err(e),
                Ok(m) => return Ok(m),
            }
        },
        Err(s) => return Err(s),
    }
}
