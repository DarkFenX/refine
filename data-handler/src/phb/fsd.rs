use struson::reader::simple::{SimpleJsonReader, ValueReader};

use crate::util::Error;

pub(in crate::phb) type FsdId = i32;

pub(in crate::phb) trait FsdMerge<T> {
    fn fsd_merge(self, id: FsdId) -> Vec<T>;
}

// pub(in crate::phb) struct FsdItem {
//     pub(in crate::phb) id: String,
//     pub(in crate::phb) item: serde_json::Value,
// }
//
// pub(in crate::phb) fn handle<T, U>(
//     unprocessed: serde_json::Value,
//     suffix: &str,
// ) -> rc::ed::EResult<rc::ed::EDataCont<U>>
// where
//     T: serde::de::DeserializeOwned + FsdMerge<U>,
// {
//     let decomposed = decompose(unprocessed, suffix)?;
//     Ok(convert::<T, U>(decomposed))
// }
//
// fn decompose(json: serde_json::Value, suffix: &str) -> Result<Vec<FsdItem>, Error> {
//     match json {
//         serde_json::Value::Object(map) => Ok(map.into_iter().map(|(id, item)| FsdItem { id, item }).collect()),
//         _ => Err(Error::PhbUnexpectedFsdTopEntity(suffix.to_string())),
//     }
// }
//
// fn convert<T, U>(decomposed: Vec<FsdItem>) -> rc::ed::EDataCont<U>
// where
//     T: serde::de::DeserializeOwned + FsdMerge<U>,
// {
//     let mut e_cont = rc::ed::EDataCont::new();
//     for fsd_item in decomposed {
//         match fsd_item.id.parse::<FsdId>() {
//             Ok(id) => match serde_json::from_value::<T>(fsd_item.item) {
//                 Ok(p_item) => e_cont.data.extend(p_item.fsd_merge(id)),
//                 Err(e) => e_cont
//                     .warns
//                     .push(format!("failed to parse FSD item with key \"{id}\": {e}")),
//             },
//             Err(_) => e_cont
//                 .warns
//                 .push(format!("failed to cast FSD key \"{}\" to integer", fsd_item.id)),
//         }
//     }
//     e_cont
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: New implementation
////////////////////////////////////////////////////////////////////////////////////////////////////

pub(in crate::phb) fn handle_one<F, E>(
    reader: impl std::io::Read,
    suffix: &str,
) -> rc::ed::EResult<rc::ed::EDataCont<E>>
where
    F: serde::de::DeserializeOwned + FsdMerge<E>,
{
    let mut e_cont = rc::ed::EDataCont::new();
    let json_reader = SimpleJsonReader::new(reader);
    match json_reader.read_object_borrowed_names(|mut member_reader| {
        let id_str = match member_reader.read_name() {
            Ok(id_str) => id_str,
            Err(e) => return Err(e.into()),
        };
        let id = match id_str.parse::<FsdId>() {
            Ok(id) => id,
            Err(e) => return Err(e.into()),
        };
        let x = match member_reader.read_deserialize::<F>() {
            Ok(x) => x,
            Err(e) => return Err(e.into()),
        };
        e_cont.data.extend(x.fsd_merge(id));
        Ok(())
    }) {
        Ok(x) => (),
        Err(e) => (),
    }
    Ok(e_cont)
}
