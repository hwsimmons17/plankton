use std::path::Path;

use heck::ToSnakeCase;

use crate::Files;

/// Create a program with multiple files for instructions, state...
fn create_program_template(name: &str, program_path: &Path) -> Files {
    let src_path = program_path.join("src");
    vec![
        (
            src_path.join("lib.rs"),
            format!(
                r#"pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("{}");

#[program]
pub mod {} {{
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {{
        initialize::handler(ctx)
    }}
}}
"#,
                get_or_create_program_id(name),
                name.to_snake_case(),
            ),
        ),
        (
            src_path.join("constants.rs"),
            r#"use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";
"#
            .into(),
        ),
        (
            src_path.join("error.rs"),
            r#"use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
}
"#
            .into(),
        ),
        (
            src_path.join("instructions").join("mod.rs"),
            r#"pub mod initialize;

pub use initialize::*;
"#
            .into(),
        ),
        (
            src_path.join("instructions").join("initialize.rs"),
            r#"use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
"#
            .into(),
        ),
        (src_path.join("state").join("mod.rs"), r#""#.into()),
    ]
}
