use pinocchio::pubkey::Pubkey;
use pinocchio::account_info::AccountInfo;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 1;

    pub fn from_account_info(account_info: &AccountInfo) -> &mut self {
        // let mut data = account_info.try_borrow_mut_data().unwrap();
        // let data = &mut data[0..Self::LEN];

        assert!(account_info.data_len() == Self::LEN);

        unsafe {
            assert!(account_info.owner() == &crate::ID);

            // account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self;

            &mut *(account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self)
        }
    }

    pub fn from_account_info_readable(account_info: &AccountInfo) -> &self {
        // let mut data = account_info.try_borrow_mut_data().unwrap();
        // let data = &mut data[0..Self::LEN];

        assert!(account_info.data_len() == Self::LEN);

        unsafe {
            assert!(account_info.owner() == &crate::ID);

            // account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self;

            &*(account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self)
        }
    }
}


