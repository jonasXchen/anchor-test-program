use anchor_lang::prelude::*;

declare_id!("A31bjS1pFNcxkVNXP5LwwGNpejETipZJbXC3XGfGu5QY");

#[program]
pub mod metacamp_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, id: String) -> Result<()> {
        msg!("Create account id: {}", id);
        msg!("Creator: {}", ctx.accounts.initializer.key());
        msg!("Name: {}", name);



        let onchain_account = &mut ctx.accounts.onchain_account;
        onchain_account.name = name;
        onchain_account.creator = ctx.accounts.initializer.key();
        onchain_account.id = id;


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, id: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [id.as_bytes().as_ref(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 4 + id.len() + 4 + name.len() + 32 + 1000
    )]
    pub onchain_account: Account<'info, OnChainData>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct OnChainData {
    pub id: String,                 // 1
    pub name: String,           // 4 + len()// 1
    pub creator: Pubkey,        // 32
}