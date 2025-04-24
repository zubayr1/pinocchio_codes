use pinocchio::account_info::AccountInfo;

#[repr(C)]
pub struct UserAccount {
    pub points: u64,
    pub amount_stake: u8,
    pub bump: u8,
}

impl UserAccount {
    pub const SIZE: usize = 8 + 1 + 1;

    pub unsafe fn from_account_info_unchecked(account_info: &AccountInfo) -> &mut Self {
        &mut *(account_info.borrow_mut_data_unchecked().as_ptr() as *mut Self)
    }
}
