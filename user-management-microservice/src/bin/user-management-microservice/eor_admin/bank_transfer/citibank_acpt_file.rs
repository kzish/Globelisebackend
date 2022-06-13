use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankACPTFile {
    pub CstmrPmtStsRpt: CstmrPmtStsRpt,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CstmrPmtStsRpt {
    pub OrgnlPmtInfAndSts: Vec<OrgnlPmtInfAndSts>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgnlPmtInfAndSts {
    pub OrgnlPmtInfId: String,
}
