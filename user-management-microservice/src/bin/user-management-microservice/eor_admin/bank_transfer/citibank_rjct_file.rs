use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankRJCTFile {
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
    pub TxInfAndSts: TxInfAndSts,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TxInfAndSts {
    pub OrgnlEndToEndId: String,
    pub TxSts: String,
    pub StsRsnInf: Vec<StsRsnInf>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct StsRsnInf {
    pub AddtlInf: Vec<String>,
}
