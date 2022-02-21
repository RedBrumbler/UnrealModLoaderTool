use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicMod {
    pub pak_number: u32,
    pub pak_name: String,
}

impl LogicMod {
    pub fn original_name(&self) -> String {
        format!("pakchunk{}-WindowsNoEditor.pak", self.pak_number)
    }

    pub fn destination_name(&self) -> String {
        format!("{}.pak", self.pak_name)
    }
}
