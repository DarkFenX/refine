use super::error::ReadParseFailReason;
use crate::{
    sde::data::{ExtractOne, ExtractTwo},
    shared::cap_warning_len,
};

pub(in crate::sde) fn extract_from_lines_one<SDE, EVE>(
    mut reader: impl std::io::BufRead,
) -> Result<rc::ed::EDataCont<EVE>, ReadParseFailReason>
where
    SDE: serde::de::DeserializeOwned + ExtractOne<EVE>,
{
    let mut e_cont = rc::ed::EDataCont::new();
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
        let sde = match serde_json::from_str::<SDE>(&line) {
            Ok(sde) => sde,
            // In case of malformed value - log error and skip line
            Err(err) => {
                let warning = cap_warning_len(format!("failed to parse value on line {lineno}: {err}"));
                e_cont.warnings.push(warning);
                continue;
            }
        };
        e_cont.data.extend(sde.extract());
    }
    Ok(e_cont)
}

pub(in crate::sde) fn extract_from_lines_two<SDE, EVE1, EVE2>(
    mut reader: impl std::io::BufRead,
) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), ReadParseFailReason>
where
    SDE: serde::de::DeserializeOwned + ExtractTwo<EVE1, EVE2>,
{
    let mut e_cont1 = rc::ed::EDataCont::new();
    let mut e_cont2 = rc::ed::EDataCont::new();
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
        let sde = match serde_json::from_str::<SDE>(&line) {
            Ok(sde) => sde,
            // In case of malformed value - log error and skip line
            Err(err) => {
                let warning = cap_warning_len(format!("failed to parse value on line {lineno}: {err}"));
                e_cont1.warnings.push(warning.clone());
                e_cont2.warnings.push(warning);
                continue;
            }
        };
        let (e_data1, e_data2) = sde.extract();
        e_cont1.data.extend(e_data1);
        e_cont2.data.extend(e_data2);
    }
    Ok((e_cont1, e_cont2))
}
