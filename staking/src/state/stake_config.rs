use pinocchio::account_info::AccountInfo;

#[repr(C)]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_period: u32,
    pub reward_bump: u8,
    pub bump: u8,
}

impl StakeConfig {
    pub const SIZE: usize = 1 + 1 + 4 + 1 + 1;

    pub unsafe fn from_account_info_unchecked(account_info: &AccountInfo) -> &mut Self {
        &mut *(account_info.borrow_mut_data_unchecked().as_ptr() as *mut Self)
    }
}
