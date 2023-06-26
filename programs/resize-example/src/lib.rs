use anchor_lang::prelude::*;

declare_id!("9WddkrNSnwUWWuk4zkyQg7eHBVHsaei1avP9YgsSQhZw");

#[program]
pub mod resize_example {
    use anchor_lang::system_program::{self, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn resize(ctx: Context<Resize>, size: u32) -> Result<()> {
        let resize_acc = &ctx.accounts.resize_account.to_account_info();
        let signer = &ctx.accounts.payer.to_account_info();
        let rent_exempt_minimum : u64 = Rent::get()?.minimum_balance(size as usize);
        let current: u64 = ctx.accounts.resize_account.to_account_info().lamports();
        
        // the vec needs to be resized to still deserialize if the account has been shrunk below the vec length:
        ctx.accounts.resize_account.my_data.resize(size.saturating_sub(8 + 4) as usize, 0);

        // we use system program transfer because program doesn't own signer
        if rent_exempt_minimum > current{
            let diff = rent_exempt_minimum.saturating_sub(current);
            system_program::transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{
                from: signer.to_account_info(),
                to: resize_acc.to_account_info()
            }), diff)?;
        }
        else if current > rent_exempt_minimum {
            let diff = current.saturating_sub(rent_exempt_minimum);
            **resize_acc.try_borrow_mut_lamports()? -= diff;
            **signer.try_borrow_mut_lamports()? += diff;
        }
        // zero init means the added space is forced to be zeroed,
        // see docs for realloc before deciding if it's needed or not
        // it may also be worth looking into zeroing bytes that'll be trimmed off
        // depending on program/functions
        ctx.accounts.resize_account.to_account_info().realloc(size as usize, false)?;
        Ok(())
    }

}

#[account]
pub struct AccountThing{
    pub my_data: Vec<u8>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space=12, seeds=[], bump)]
    pub resize_account: Account<'info, AccountThing>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Resize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub resize_account: Account<'info, AccountThing>,
    system_program: Program<'info, System>,
}
