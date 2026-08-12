use super::error::ReadParseFailReason;
use crate::{
    sde::data::{ExtractOne, ExtractTwo},
    shared::cap_warning_len,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interface methods
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::sde) fn extract_from_lines_one<SDE, EVE>(
    reader: impl std::io::BufRead,
) -> Result<rc::ed::EDataCont<EVE>, ReadParseFailReason>
where
    SDE: serde::de::DeserializeOwned + ExtractOne<EVE>,
{
    let mut e_cont = rc::ed::EDataCont::new();
    for_each_line::<SDE>(reader, &mut e_cont.warnings, |sde| {
        sde.extract(&mut e_cont.data);
    })?;
    Ok(e_cont)
}

pub(in crate::sde) fn extract_from_lines_two<SDE, EVE1, EVE2>(
    reader: impl std::io::BufRead,
) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), ReadParseFailReason>
where
    SDE: serde::de::DeserializeOwned + ExtractTwo<EVE1, EVE2>,
{
    let mut e_cont1 = rc::ed::EDataCont::new();
    let mut e_cont2 = rc::ed::EDataCont::new();
    let mut warnings = Vec::new();
    for_each_line::<SDE>(reader, &mut warnings, |sde| {
        sde.extract(&mut e_cont1.data, &mut e_cont2.data);
    })?;
    e_cont1.warnings = warnings.clone();
    e_cont2.warnings = warnings;
    Ok((e_cont1, e_cont2))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
fn for_each_line<SDE>(
    mut reader: impl std::io::BufRead,
    warnings: &mut Vec<String>,
    mut process: impl FnMut(SDE),
) -> Result<(), ReadParseFailReason>
where
    SDE: serde::de::DeserializeOwned,
{
    let mut line = String::new();
    let mut lineno = 0;
    while {
        line.clear();
        reader.read_line(&mut line)?
    } > 0
    {
        lineno += 1;
        if line.trim().is_empty() {
            continue;
        }
        // In case of malformed value - log a warning and skip line
        let sde = match serde_json::from_str::<SDE>(&line) {
            Ok(sde) => sde,
            Err(err) => {
                let warning = cap_warning_len(format!("failed to parse value on line {lineno}: {err}"));
                warnings.push(warning);
                continue;
            }
        };
        process(sde);
    }
    Ok(())
}
