use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankACKFile {
    pub CstmrPmtStsRpt: CstmrPmtStsRpt,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CstmrPmtStsRpt {
    pub OrgnlGrpInfAndSts: OrgnlGrpInfAndSts,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct StsRsnInf {
    pub AddtlInf: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgnlGrpInfAndSts {
    pub OrgnlMsgId: String,
    pub GrpSts: String,
    pub StsRsnInf: StsRsnInf,
}
