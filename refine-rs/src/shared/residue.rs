/// This enum is used to describe how a command behaves - does it change solar system state, and can
/// it fail or not.
///
/// The change here is not used in sense of rust mutation, it only concerns user data. For example,
/// modifying attribute cache (which happens when calculating stats or fetching item's attributes)
/// is not counted as a change. Adding and removing an item bumps counters used for allocation of
/// new IDs, but also is not considered a change. Neither of those change IDs assigned to user data,
/// or user data itself directly.
pub(crate) enum CmdResidue {
    /// Never modifies solar system state, and cannot fail.
    ImmutInfallible,
    /// Never modifies solar system state, and can fail.
    ImmutFallible,
    /// May modify solar system state, but will never fail.
    MutInfallible,
    /// May modify solar system state, but in case of failure reverts its effect.
    MutFallibleClean,
    /// May modify solar system state, but in case of failure may not revert its effect completely.
    MutFallibleDirty,
}

#[derive(Copy, Clone)]
pub(crate) enum SolBackup {
    Needed,
    NotNeeded,
}

pub(crate) struct ResidueResolver {
    seen_mutating: bool,
    backup: SolBackup,
}
impl ResidueResolver {
    pub(crate) fn new() -> Self {
        Self {
            seen_mutating: false,
            backup: SolBackup::NotNeeded,
        }
    }
    pub(crate) fn add_cmd(&mut self, residue: CmdResidue) -> SolBackup {
        match residue {
            CmdResidue::ImmutInfallible => self.backup,
            // If something could possibly mutate sol before and this command can fail, need backup
            CmdResidue::ImmutFallible => {
                if self.seen_mutating {
                    self.backup = SolBackup::Needed
                }
                self.backup
            }
            ,
            // Infallible mutable command just changes context for fallible commands
            CmdResidue::MutInfallible => {
                self.seen_mutating = true;
                self.backup
            }
            // Even if command reverts its action cleanly, it cannot revert already executed
            // commands; in this case, sol backup is needed
            CmdResidue::MutFallibleClean => {
                match self.seen_mutating {
                    true => self.backup = SolBackup::Needed,
                    false => self.seen_mutating = true,
                }
                self.backup
            }
            // Commands which can fail without proper recovery need sol backup regardless
            CmdResidue::MutFallibleDirty => {
                self.backup = SolBackup::Needed;
                self.backup
            }
        }
    }
    pub(crate) fn add_cmds(&mut self, residues: impl Iterator<Item = CmdResidue>) -> SolBackup {
        for residue in residues {
            // Once it is concluded that backup is needed, do not need to go though the rest of the
            // commands
            if let SolBackup::Needed = self.add_cmd(residue) {
                return SolBackup::Needed;
            }
        }
        self.backup
    }
    pub(crate) fn into_backup(self) -> SolBackup {
        self.backup
    }
}
