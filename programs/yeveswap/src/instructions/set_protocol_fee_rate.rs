use anchor_lang::prelude::*;

use crate::state::{Yevepool, YevepoolsConfig};

#[derive(Accounts)]
pub struct SetProtocolFeeRate<'info> {
    pub yevepools_config: Account<'info, YevepoolsConfig>,

    #[account(mut, has_one = yevepools_config)]
    pub yevepool: Account<'info, Yevepool>,

    #[account(address = yevepools_config.fee_authority)]
    pub fee_authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetProtocolFeeRate>, protocol_fee_rate: u16) -> Result<()> {
    Ok(ctx
        .accounts
        .yevepool
        .update_protocol_fee_rate(protocol_fee_rate)?)
}
