use crate::{env::Env, expr::Expr, utils};
use alloc::string::{String, ToString};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
     name: String,
     val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("dison", s, "Expected `dison`")?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let s = utils::tag("kom", s, "Expected type keyword `kom`")?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, _) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s, "Expected initialisation keyword `laik`")?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(self.name.clone(), self.val.eval(&env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("dison a kom noma = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            )),
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("disonaaa kom noma=1+2"),
            Err("Error: Expected whitespace".to_string()),
        );
    }
}
