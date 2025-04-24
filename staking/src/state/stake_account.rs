use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[repr(C)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8,
}

impl StakeAccount {
    pub const SIZE: usize = 32 + 32 + 8 + 1;

    pub unsafe fn from_account_info_unchecked(account_info: &AccountInfo) -> &mut Self {
        &mut *(account_info.borrow_mut_data_unchecked().as_ptr() as *mut Self)
    }
}
