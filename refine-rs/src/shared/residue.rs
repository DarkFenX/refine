/// This enum is used to describe how a command changes solar system state.
///
/// The change here is not used in sense of rust mutation, it only concerns user data. For example,
/// modifying attribute cache (which happens when calculating stats or fetching item's attributes)
/// is not counted as a change. Adding and removing an item bumps counters used for allocation of
/// new IDs, but also is not considered a change. Neither of those change IDs assigned to user data,
/// or user data itself directly.
pub(crate) enum CmdResidue {
    /// Command does not modify solar system state (e.g. info/stats getters)
    None,
    /// Command modifies solar system state, but will never fail (e.g. adding a fit without fleet
    /// set)
    Infallible,
    /// Command modifies solar system state, but in case of failure reverts its effect
    FallibleClean,
    /// Command modifies solar system state, but in case of failure may not revert its effect
    FallibleDirty,
}

#[derive(Copy, Clone)]
pub(crate) enum SolBackup {
    Needed,
    NotNeeded,
}

pub(crate) struct ResidueResolver {
    seen_mutable: bool,
    backup: SolBackup,
}
impl ResidueResolver {
    pub(crate) fn new() -> Self {
        Self {
            seen_mutable: false,
            backup: SolBackup::NotNeeded,
        }
    }
    pub(crate) fn add_cmd(&mut self, residue: CmdResidue) -> SolBackup {
        match residue {
            CmdResidue::None => self.backup,
            // Infallible command just changes context for fallible commands
            CmdResidue::Infallible => {
                self.seen_mutable = true;
                self.backup
            }
            // Even if command reverts its action cleanly, it cannot revert already executed
            // commands; in this case, sol backup is needed
            CmdResidue::FallibleClean => {
                match self.seen_mutable {
                    true => self.backup = SolBackup::Needed,
                    false => self.seen_mutable = true,
                }
                self.backup
            }
            // Commands which can fail without proper recovery need sol backup regardless
            CmdResidue::FallibleDirty => {
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
