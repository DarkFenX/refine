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
    /// Command modifies solar system state, but will never fail (e.g. adding a fit)
    Infallible,
    /// Command modifies solar system state, but in case of failure reverts its effect
    FallibleClean,
    /// Command modifies solar system state, but in case of failure may not revert its effect
    FallibleDirty,
}

pub(crate) struct ResidueResolver {
    seen_mutable: bool,
    needs_backup: bool,
}
impl ResidueResolver {
    pub(crate) fn new() -> Self {
        Self {
            seen_mutable: false,
            needs_backup: false,
        }
    }
    pub(crate) fn add_cmd(&mut self, residue: CmdResidue) -> bool {
        match residue {
            CmdResidue::None => self.needs_backup,
            // Infallible command just changes context for fallible commands
            CmdResidue::Infallible => {
                self.seen_mutable = true;
                self.needs_backup
            }
            // Even if command reverts its action cleanly, it cannot revert already executed
            // commands; in this case, sol backup is needed
            CmdResidue::FallibleClean => {
                match self.seen_mutable {
                    true => self.needs_backup = true,
                    false => self.seen_mutable = true,
                }
                self.needs_backup
            }
            // Commands which can fail without proper recovery need sol backup regardless
            CmdResidue::FallibleDirty => {
                self.needs_backup = true;
                self.needs_backup
            }
        }
    }
    pub(crate) fn add_cmds(&mut self, residues: impl Iterator<Item = CmdResidue>) -> bool {
        for residue in residues {
            if self.add_cmd(residue) {
                return true;
            }
        }
        self.needs_backup
    }
    pub(crate) fn into_needs_backup(self) -> bool {
        self.needs_backup
    }
}
