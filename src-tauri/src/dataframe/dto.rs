use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize)]
#[typeshare]
pub struct DataExplorerDataframe {
    #[typeshare(serialized_as = "Array<Map<string, any>>")]
    pub columns: Vec<MuiTableColumns>,
    #[typeshare(serialized_as = "Array<Map<string, any>>")]
    pub data: Vec<Map<String, Value>>,
}

impl From<DataExplorerDataframe> for String {
    fn from(df: DataExplorerDataframe) -> Self {
        serde_json::to_string(&df).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[typeshare]
pub struct MuiTableColumns {
    pub accessorKey: String,
    pub header: String,
}

impl From<&String> for MuiTableColumns {
    fn from(s: &String) -> Self {
        MuiTableColumns {
            accessorKey: s.to_string(),
            header: s.to_string(),
        }
    }
}
