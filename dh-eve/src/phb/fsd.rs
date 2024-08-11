use crate::util::Error;

pub(in crate::phb) type FsdId = i32;

pub(in crate::phb) trait FsdMerge<T> {
    fn fsd_merge(self, id: FsdId) -> Vec<T>;
}

pub(in crate::phb) struct FsdItem {
    pub(in crate::phb) id: String,
    pub(in crate::phb) item: serde_json::Value,
}
impl FsdItem {
    pub(in crate::phb) fn new(id: String, item: serde_json::Value) -> Self {
        Self { id, item }
    }
}

pub(in crate::phb) fn handle<T, U>(
    unprocessed: serde_json::Value,
    suffix: &str,
) -> rc::ed::EResult<rc::ed::EDataCont<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let decomposed = decompose(unprocessed, suffix)?;
    Ok(convert::<T, U>(decomposed))
}

fn decompose(json: serde_json::Value, suffix: &str) -> Result<Vec<FsdItem>, Error> {
    match json {
        serde_json::Value::Object(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
        _ => Err(Error::PhbUnexpectedFsdTopEntity(suffix.to_string())),
    }
}

fn convert<T, U>(decomposed: Vec<FsdItem>) -> rc::ed::EDataCont<U>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let mut e_cont = rc::ed::EDataCont::new();
    for fsd_item in decomposed {
        match fsd_item.id.parse::<FsdId>() {
            Ok(id) => match serde_json::from_value::<T>(fsd_item.item) {
                Ok(p_item) => e_cont.data.extend(p_item.fsd_merge(id)),
                Err(e) => e_cont
                    .warns
                    .push(format!("failed to parse FSD item with key \"{id}\": {e}")),
            },
            Err(_) => e_cont
                .warns
                .push(format!("failed to cast FSD key \"{}\" to integer", fsd_item.id)),
        }
    }
    e_cont
}
