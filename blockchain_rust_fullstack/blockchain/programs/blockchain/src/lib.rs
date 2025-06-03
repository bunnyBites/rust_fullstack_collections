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

    pub fn add_task(ctx: Context<AddTask>, content: String) -> Result<()> {
        let todos = &mut ctx.accounts.todo.todos;

        if todos.len() >= MAX_LIST_SIZE {
            return err!(MyError::ListFull);
        }

        if content.chars().count() >= MAX_LIST_CONTENT_LENGTH {
            return err!(MyError::ListContentExceedsLimit);
        }

        todos.push(TodoList {
            content,
            is_completed: false,
        });

        msg!("Content successfully pushed");

        Ok(())
    }

    pub fn toggle_state(ctx: Context<ToggleState>, toggle_index: u8) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo.todos;

        if toggle_index as usize > todo_list.len() {
            return err!(MyError::IndexNotFound);
        }

        if let Some(value) = todo_list.get_mut(toggle_index as usize) {
            value.is_completed = !value.is_completed;
        }

        msg!("Updated toggle state successfully");

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

#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds=[b"list", user.key().as_ref()], bump)]
    pub todo: Account<'info, Todo>,
}

#[derive(Accounts)]
pub struct ToggleState<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"list", user.key().as_ref()], bump)]
    pub todo: Account<'info, Todo>,
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

#[error_code]
pub enum MyError {
    #[msg("List is full")]
    ListFull,

    #[msg("Provided content exceeds the expected limit")]
    ListContentExceedsLimit,

    #[msg("Provided index is not present in the list")]
    IndexNotFound,
}
