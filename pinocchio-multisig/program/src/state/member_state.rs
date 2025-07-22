use pinocchio::{pubkey::Pubkey, ProgramResult};
use crate::error::MyProgramError;
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permission {
    Readonly = 0,
    Vote = 1,
    VoteAndExecute = 2,
}

impl Permission {
    pub fn from_u8(val: u8) -> Self {
        match val {
            1 => Permission::Vote,
            2 => Permission::VoteAndExecute,
            _ => Permission::Readonly,
        }
    }
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Member {
    pub key: Pubkey,
    pub permissions: Permission,
    pub is_active: u8,
}

impl Default for Member {
    fn default() -> Self {
        Member {
            key: Pubkey::default(),
            permissions: Permission::Readonly,
            is_active: 0,
        }
    }
}

pub fn update_member(member_key: Pubkey, members: &mut [Member], update_type: u8, permission: u8, is_active: u8, members_to_add: [Member; 10],
    index: u8, threshold: u8, current_index: &mut i8, roaming_index: &mut i8) -> ProgramResult {

    match update_type {
        0 => { // add member
            if *current_index == *roaming_index && *current_index >= (threshold -1) as i8 {
                return Err(MyProgramError::Overflow.into());
            }

            let mut search_idx = if *roaming_index == -1 { 0 } else { *roaming_index };
            while search_idx < threshold as i8 {
                if members[search_idx as usize].key == Pubkey::default() {
                    break;
                }
                search_idx += 1;
            }

            if search_idx >= threshold as i8 {
                return Err(MyProgramError::Overflow.into());
            }

            let member = Member {
                key: member_key,
                permissions: Permission::from_u8(permission),
                is_active: 1,
            };
            members[search_idx as usize] = member;

            *roaming_index = search_idx;
            if *roaming_index > *current_index {
                *current_index = *roaming_index;
            }
            
            Ok(())           
        }
        1 => { // remove member
            if *current_index == *roaming_index && *current_index == -1 as i8 {
                return Err(MyProgramError::Underflow.into());
            }

            if index >= threshold as u8 {
                return Err(MyProgramError::Overflow.into());
            }

            if members[index as usize].key == Pubkey::default() {
                return Err(MyProgramError::MemberNotFound.into());
            }

            members[index as usize] = Member::default();

            if *roaming_index > index as i8 {
                *roaming_index = index as i8;
            }

            if (index as i8) == *current_index {
                let mut i = *current_index - 1;
                while i >= 0 {
                    if members[i as usize].key != Pubkey::default() {
                        break;
                    }
                    i -= 1;
                }
                *current_index = i;

                if *current_index == -1 {
                    *roaming_index = -1;
                }
            }

            Ok(())
        }
        2 => { // update member permission
            if index >= threshold as u8 {
                return Err(MyProgramError::Overflow.into());
            }

            if members[index as usize].key == Pubkey::default() {
                return Err(MyProgramError::MemberNotFound.into());
            }
            members[index as usize].permissions = Permission::from_u8(permission);
            Ok(())
        }
        3 => { // update member key
            if index >= threshold as u8 {
                return Err(MyProgramError::Overflow.into());
            }

            if members[index as usize].key == Pubkey::default() {
                return Err(MyProgramError::MemberNotFound.into());
            }

            members[index as usize].key = member_key;
            Ok(())
        }
        4 => { // update member active status
            if index >= threshold as u8 {
                return Err(MyProgramError::Overflow.into());
            }

            if members[index as usize].key == Pubkey::default() {
                return Err(MyProgramError::MemberNotFound.into());
            }

            members[index as usize].is_active = if is_active == 1 { 1 } else { 0 };
            Ok(())
        }
        5 => { // batch add members
            if *current_index == *roaming_index && *current_index == -1 as i8 {
                members.copy_from_slice(&members_to_add);
                *current_index = threshold as i8 - 1;
                *roaming_index = threshold as i8 - 1;
                Ok(())
            }
            else {
                return Err(MyProgramError::Overflow.into());
            }
        }
        6 => { // reset members
            if *current_index >= 0 {
                for m in &mut members[0..(*current_index as usize)] {
                    *m = Member::default();
                }
            }
            *current_index = -1;
            *roaming_index = -1;
            Ok(())
        }

        _ => Err(MyProgramError::InvalidInstructionData.into()),
    }
}