use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

use crate::error::MyProgramError;

pub trait StateDefinition {
    const LEN: usize;
    const SEED: &'static str;
}

pub trait Initialized {
    fn is_initialized(&self) -> bool;
}

#[inline(always)]
pub unsafe fn load_acc<T: StateDefinition + Initialized>(bytes: &[u8]) -> Result<&T, ProgramError> {
    load_acc_unchecked::<T>(bytes).and_then(|acc| {
        if acc.is_initialized() {
            Ok(acc)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

#[inline(always)]
pub unsafe fn load_acc_unchecked<T: StateDefinition>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

#[inline(always)]
pub unsafe fn load_acc_mut<T: StateDefinition + Initialized>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    load_acc_mut_unchecked::<T>(bytes).and_then(|acc| {
        if acc.is_initialized() {
            Ok(acc)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

#[inline(always)]
pub unsafe fn load_acc_mut_unchecked<T: StateDefinition>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[inline(always)]
pub unsafe fn load_ix_data<T: StateDefinition>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(MyProgramError::InvalidInstructionData.into());
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

pub unsafe fn to_bytes<T: StateDefinition>(data: &T) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, T::LEN)
}

pub unsafe fn to_mut_bytes<T: StateDefinition>(data: &mut T) -> &mut [u8] {
    core::slice::from_raw_parts_mut(data as *mut T as *mut u8, T::LEN)
}

pub unsafe fn try_from_account_info<T: StateDefinition>(acc: &AccountInfo) -> Result<&T, ProgramError> {
    if acc.owner() != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }
    let bytes = acc.try_borrow_data()?;

    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

pub unsafe fn try_from_account_info_mut<T: StateDefinition>(
    acc: &AccountInfo,
) -> Result<&mut T, ProgramError> {
    if acc.owner() != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    let mut bytes = acc.try_borrow_mut_data()?;

    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}
