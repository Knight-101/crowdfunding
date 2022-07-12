use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("56CK4VC9iRf2Fn3jx9G2hoesvY8YGvhRmfSck8dEbcej");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn create_account(ctx: Context<Create>) -> ProgramResult {
        Ok(())
    }
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        description: String,
    ) -> ProgramResult {
        let campaigns = &mut ctx.accounts.campaigns;
        let id: u16;
        match campaigns.list.len() {
            0 => id = 0,
            _ => id = campaigns.list[campaigns.list.len() - 1].id + 1,
        }
        let new_campaign = Campaign {
            admin: *ctx.accounts.user.key,
            id,
            name,
            description,
            amount_donated: 0,
        };
        campaigns.list.push(new_campaign);
        Ok(())
    }

    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
    //     let campaign = &mut ctx.accounts.campaign;
    //     let user = &mut ctx.accounts.user;
    //     if campaign.admin != *user.key {
    //         return Err(ProgramError::IncorrectProgramId);
    //     }
    //     let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
    //     if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
    //         return Err(ProgramError::InsufficientFunds);
    //     }
    //     **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
    //     **user.to_account_info().try_borrow_mut_lamports()? += amount;
    //     Ok(())
    // }

    // pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
    //     let ix = anchor_lang::solana_program::system_instruction::transfer(
    //         &ctx.accounts.user.key(),
    //         &ctx.accounts.campaign.key(),
    //         amount,
    //     );
    //     anchor_lang::solana_program::program::invoke(
    //         &ix,
    //         &[
    //             ctx.accounts.user.to_account_info(),
    //             ctx.accounts.campaign.to_account_info(),
    //         ],
    //     );
    //     (&mut ctx.accounts.campaign).amount_donated += amount;
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN".as_ref(), user.key().as_ref()], bump)]
    pub campaigns: Account<'info, Campaigns>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub campaigns: Account<'info, Campaigns>,
    #[account(mut)]
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaigns: Account<'info, Campaigns>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaigns: Account<'info, Campaigns>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Campaign {
    pub admin: Pubkey,
    pub id: u16,
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
}
#[account]
pub struct Campaigns {
    pub list: Vec<Campaign>,
}
