use anchor_lang::prelude::*;

declare_id!("4sXU7Z9U3N1TFWiTU9ZZ9R5XAoiL633JGsQfCFKdTYQF");

const MAX_LIST_SIZE: usize = 10;
const MAX_LIST_CONTENT_LENGTH: usize = 200;

#[program]
pub mod blockchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let todo = &mut ctx.accounts.todo;

        todo.user = ctx.accounts.user.key();
        todo.bump = ctx.bumps.todo;
        todo.todos = Vec::new();

        msg!(
            "Your initialized account: user - {}, todos - {:?}",
            todo.user,
            todo.todos
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
      init,
      payer = user,
      space = 8 + 32 + 1 + 4 + (MAX_LIST_SIZE * MAX_LIST_CONTENT_LENGTH),
      seeds = [b"list", user.key().as_ref()], bump,
    )]
    todo: Account<'info, Todo>,
    system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TodoList {
    content: String,
    pub is_completed: bool,
}

#[account]
pub struct Todo {
    user: Pubkey,
    bump: u8,
    todos: Vec<TodoList>,
}
