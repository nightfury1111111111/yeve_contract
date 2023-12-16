use anchor_lang::prelude::*;

use crate::state::YevepoolsConfig;

#[derive(Accounts)]
pub struct SetRewardEmissionsSuperAuthority<'info> {
    #[account(mut)]
    pub yevepools_config: Account<'info, YevepoolsConfig>,

    #[account(address = yevepools_config.reward_emissions_super_authority)]
    pub reward_emissions_super_authority: Signer<'info>,

    /// CHECK: safe, the account that will be new authority can be arbitrary
    pub new_reward_emissions_super_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetRewardEmissionsSuperAuthority>) -> Result<()> {
    Ok(ctx
        .accounts
        .yevepools_config
        .update_reward_emissions_super_authority(
            ctx.accounts.new_reward_emissions_super_authority.key(),
        ))
}
