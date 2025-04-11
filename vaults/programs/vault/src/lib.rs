#![allow(unexpected_cfgs)] //it will hide unexpected error
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("DPcM5c9ZJtxkuCAGgakMthWTLPsehb4WYXGLx3bKkKjc");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        ctx.accounts.initialise(&ctx.bumps)

    }


    pub fn deposit(ctx: Context<Payment>, amount: u64)-> Result<()>{
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64)-> Result<()>{
        ctx.accounts.withdraw
        (amount)
    }

    pub fn close (ctx: Context<Close>) -> Result<()>{
        ctx.accounts.close()

    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] //its balance will change so it will be mut
    pub user: Signer<'info>,

    //how deterministically derived so Pda
    #[account(
        init,
        payer= user,
        seeds = [b"state", user.key().as_ref() ],//binary string and here user.key ,deriving ata (mintaddress + public key) | as_ref() changes the Pubkey into a &[u8] (byte slice).
        bump, 
        space = VaultState::INIT_SPACE,
     )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds= [b"vault",user.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,//in staring , when initialize ,we deal with system account , so it will automatically init
    
    pub system_program : Program<'info,System>
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state : Account<'info, VaultState>,

    #[account( 
        seeds= [b"vault",user.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,

    pub system_program : Program<'info, System>

}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut,
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close= user,
    )]
    pub vault_state : Account<'info, VaultState>,

    #[account( 
        seeds= [b"vault",user.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,

    pub system_program : Program<'info, System>

}

impl <'info> Close <'info> {
    pub fn close(&mut self, ) -> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from : self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.valut_bump],
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())

    }


}
impl <'info> Payment <'info> {

    pub fn deposit (&mut self, amount: u64)-> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from : self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    pub fn withdraw (&mut self, amount: u64)-> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from : self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.valut_bump],
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);

        transfer(cpi_ctx, amount)
    }
}


impl <'info> Initialize<'info> {

    pub fn initialise(&mut self, bumps : &InitializeBumps) -> Result<()>{

   self.vault_state.valut_bump = bumps.vault;
   self.vault_state.state_bump = bumps.vault_state;

   Ok(())
   
   }
}
#[account] //when we create account , so we use macro , it will provide necessary things to struct
pub struct VaultState {
    pub valut_bump: u8, //pda itself , here +1 bit
    pub state_bump: u8, //pdf for state , here +1 bit
}
//implement the space to vaultstate struct
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1; //for discriminator 8 bits
}


/*
Necessary things :
- we are not creating and find program address , it is expensive so i store bump inside chain
- with interacting with different program , so we use cpi (cross invocation program)
*/
