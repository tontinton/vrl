use crate::compiler::prelude::*;

use crate::stdlib::casing::into_case;
use convert_case::Case;

#[derive(Clone, Copy, Debug)]
pub struct Snakecase;

impl Function for Snakecase {
    fn identifier(&self) -> &'static str {
        "snakecase"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "original_case",
                kind: kind::BYTES,
                required: false,
            },
        ]
    }

    fn compile(
        &self,
        state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let original_case = arguments
            .optional_enum("original_case", &super::variants(), state)?
            .map(|b| {
                into_case(
                    b.try_bytes_utf8_lossy()
                        .expect("cant convert to string")
                        .as_ref(),
                )
            })
            .transpose()?;

        Ok(SnakecaseFn {
            value,
            original_case,
        }
        .as_expr())
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "snakecase",
            source: r#"snakecase("InputString")"#,
            result: Ok("input_string"),
        }]
    }
}

#[derive(Debug, Clone)]
struct SnakecaseFn {
    value: Box<dyn Expression>,
    original_case: Option<Case>,
}

impl FunctionExpression for SnakecaseFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        super::convert_case(&value, Case::Snake, self.original_case)
    }

    fn type_def(&self, _: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        snakecase => Snakecase;

        simple {
            args: func_args![value: value!("camelCase"), original_case: "camelCase"],
            want: Ok(value!("camel_case")),
            tdef: TypeDef::bytes(),
        }

        no_case {
            args: func_args![value: value!("camelCase")],
            want: Ok(value!("camel_case")),
            tdef: TypeDef::bytes(),
        }
    ];
}
