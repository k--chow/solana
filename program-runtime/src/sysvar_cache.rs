use solana_sdk::instruction::Instruction;

#[allow(deprecated)]
use solana_sdk::sysvar::{fees::Fees, recent_blockhashes::RecentBlockhashes};
use {
    crate::invoke_context::InvokeContext,
    solana_sdk::{
        account::{AccountSharedData, ReadableAccount},
        instruction::InstructionError,
        keyed_account::KeyedAccount,
        pubkey::Pubkey,
        sysvar::{
            clock::Clock, epoch_schedule::EpochSchedule, instructions::Instructions, rent::Rent,
            slot_hashes::SlotHashes, stake_history::StakeHistory, Sysvar, SysvarId,
        },
        transaction_context::{InstructionContext, TransactionContext},
    },
    std::sync::Arc,
};

#[cfg(RUSTC_WITH_SPECIALIZATION)]
impl ::solana_frozen_abi::abi_example::AbiExample for SysvarCache {
    fn example() -> Self {
        // SysvarCache is not Serialize so just rely on Default.
        SysvarCache::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SysvarCache {
    clock: Option<Arc<Clock>>,
    epoch_schedule: Option<Arc<EpochSchedule>>,
    instructions: Option<Arc<Instructions>>,
    #[allow(deprecated)]
    fees: Option<Arc<Fees>>,
    rent: Option<Arc<Rent>>,
    slot_hashes: Option<Arc<SlotHashes>>,
    #[allow(deprecated)]
    recent_blockhashes: Option<Arc<RecentBlockhashes>>,
    stake_history: Option<Arc<StakeHistory>>,
}

impl SysvarCache {
    pub fn get_clock(&self) -> Result<Arc<Clock>, InstructionError> {
        self.clock
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_clock(&mut self, clock: Clock) {
        self.clock = Some(Arc::new(clock));
    }

    pub fn get_instructions(&self) -> Result<Arc<Instructions>, InstructionError> {
        self.instructions
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_instructions(&mut self, instructions: Instructions) {
        self.instructions = Some(Arc::new(instructions));
    }

    pub fn get_epoch_schedule(&self) -> Result<Arc<EpochSchedule>, InstructionError> {
        self.epoch_schedule
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_epoch_schedule(&mut self, epoch_schedule: EpochSchedule) {
        self.epoch_schedule = Some(Arc::new(epoch_schedule));
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn get_fees(&self) -> Result<Arc<Fees>, InstructionError> {
        self.fees.clone().ok_or(InstructionError::UnsupportedSysvar)
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn set_fees(&mut self, fees: Fees) {
        self.fees = Some(Arc::new(fees));
    }

    pub fn get_rent(&self) -> Result<Arc<Rent>, InstructionError> {
        self.rent.clone().ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_rent(&mut self, rent: Rent) {
        self.rent = Some(Arc::new(rent));
    }

    pub fn get_slot_hashes(&self) -> Result<Arc<SlotHashes>, InstructionError> {
        self.slot_hashes
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_slot_hashes(&mut self, slot_hashes: SlotHashes) {
        self.slot_hashes = Some(Arc::new(slot_hashes));
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn get_recent_blockhashes(&self) -> Result<Arc<RecentBlockhashes>, InstructionError> {
        self.recent_blockhashes
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn set_recent_blockhashes(&mut self, recent_blockhashes: RecentBlockhashes) {
        self.recent_blockhashes = Some(Arc::new(recent_blockhashes));
    }

    pub fn get_stake_history(&self) -> Result<Arc<StakeHistory>, InstructionError> {
        self.stake_history
            .clone()
            .ok_or(InstructionError::UnsupportedSysvar)
    }

    pub fn set_stake_history(&mut self, stake_history: StakeHistory) {
        self.stake_history = Some(Arc::new(stake_history));
    }

    pub fn fill_missing_entries<F: FnMut(&Pubkey) -> Option<AccountSharedData>>(
        &mut self,
        mut load_sysvar_account: F,
    ) {
        if self.get_clock().is_err() {
            if let Some(clock) = load_sysvar_account(&Clock::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_clock(clock);
            }
        }
        if self.get_instructions().is_err() {
            if let Some(instructions) = load_sysvar_account(&Instructions::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_instructions(instructions);
            }
        }
        if self.get_epoch_schedule().is_err() {
            if let Some(epoch_schedule) = load_sysvar_account(&EpochSchedule::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_epoch_schedule(epoch_schedule);
            }
        }
        #[allow(deprecated)]
        if self.get_fees().is_err() {
            if let Some(fees) = load_sysvar_account(&Fees::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_fees(fees);
            }
        }
        if self.get_rent().is_err() {
            if let Some(rent) = load_sysvar_account(&Rent::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_rent(rent);
            }
        }
        if self.get_slot_hashes().is_err() {
            if let Some(slot_hashes) = load_sysvar_account(&SlotHashes::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_slot_hashes(slot_hashes);
            }
        }
        #[allow(deprecated)]
        if self.get_recent_blockhashes().is_err() {
            if let Some(recent_blockhashes) = load_sysvar_account(&RecentBlockhashes::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_recent_blockhashes(recent_blockhashes);
            }
        }
        if self.get_stake_history().is_err() {
            if let Some(stake_history) = load_sysvar_account(&StakeHistory::id())
                .and_then(|account| bincode::deserialize(account.data()).ok())
            {
                self.set_stake_history(stake_history);
            }
        }
    }

    pub fn reset(&mut self) {
        *self = SysvarCache::default();
    }
}

/// These methods facilitate a transition from fetching sysvars from keyed
/// accounts to fetching from the sysvar cache without breaking consensus. In
/// order to keep consistent behavior, they continue to enforce the same checks
/// as `solana_sdk::keyed_account::from_keyed_account` despite dynamically
/// loading them instead of deserializing from account data.
pub mod get_sysvar_with_account_check {
    use super::*;

    fn check_sysvar_keyed_account<S: Sysvar>(
        keyed_account: &KeyedAccount,
    ) -> Result<(), InstructionError> {
        if !S::check_id(keyed_account.unsigned_key()) {
            return Err(InstructionError::InvalidArgument);
        }
        Ok(())
    }

    pub fn clock(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<Clock>, InstructionError> {
        check_sysvar_keyed_account::<Clock>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_clock()
    }

    pub fn instructions(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<Instructions>, InstructionError> {
        check_sysvar_keyed_account::<Instructions>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_instructions()
    }

    pub fn rent(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<Rent>, InstructionError> {
        check_sysvar_keyed_account::<Rent>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_rent()
    }

    pub fn slot_hashes(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<SlotHashes>, InstructionError> {
        check_sysvar_keyed_account::<SlotHashes>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_slot_hashes()
    }

    #[allow(deprecated)]
    pub fn recent_blockhashes(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<RecentBlockhashes>, InstructionError> {
        check_sysvar_keyed_account::<RecentBlockhashes>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_recent_blockhashes()
    }

    pub fn stake_history(
        keyed_account: &KeyedAccount,
        invoke_context: &InvokeContext,
    ) -> Result<Arc<StakeHistory>, InstructionError> {
        check_sysvar_keyed_account::<StakeHistory>(keyed_account)?;
        invoke_context.get_sysvar_cache().get_stake_history()
    }
}

pub mod get_sysvar_with_account_check2 {
    use super::*;

    fn check_sysvar_account<S: Sysvar>(
        transaction_context: &TransactionContext,
        instruction_context: &InstructionContext,
        instruction_account_index: usize,
    ) -> Result<(), InstructionError> {
        if !S::check_id(
            instruction_context
                .get_instruction_account_key(transaction_context, instruction_account_index)?,
        ) {
            return Err(InstructionError::InvalidArgument);
        }
        Ok(())
    }

    pub fn clock(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<Clock>, InstructionError> {
        check_sysvar_account::<Clock>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_clock()
    }

    pub fn instructions(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<Instructions>, InstructionError> {
        check_sysvar_account::<Instructions>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_instructions()
    }

    pub fn rent(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<Rent>, InstructionError> {
        check_sysvar_account::<Rent>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_rent()
    }

    pub fn slot_hashes(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<SlotHashes>, InstructionError> {
        check_sysvar_account::<SlotHashes>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_slot_hashes()
    }

    #[allow(deprecated)]
    pub fn recent_blockhashes(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<RecentBlockhashes>, InstructionError> {
        check_sysvar_account::<RecentBlockhashes>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_recent_blockhashes()
    }

    pub fn stake_history(
        invoke_context: &InvokeContext,
        instruction_context: &InstructionContext,
        index_in_instruction: usize,
    ) -> Result<Arc<StakeHistory>, InstructionError> {
        check_sysvar_account::<StakeHistory>(
            invoke_context.transaction_context,
            instruction_context,
            index_in_instruction,
        )?;
        invoke_context.get_sysvar_cache().get_stake_history()
    }
}
