pub mod syntax_checker;
pub mod semantic_checker;

pub fn parser(file: String) -> Result<(Vec<Vec<syntax_checker::Token>>, Vec<syntax_checker::Token>), syntax_checker::ParseError> {
    let syntax = syntax_checker::parser(file);
    if let Ok(s) = syntax {
        if let Err(semantic_checker) = semantic_checker::semantic_checker(&s.0) {
            return Err(semantic_checker)
        } else {
            return Ok(s)
        }
    } else {
        return syntax
    }

}
