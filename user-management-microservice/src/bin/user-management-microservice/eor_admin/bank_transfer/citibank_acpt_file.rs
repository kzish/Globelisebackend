use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankACPTFile {
    #[serde(rename = "CstmrPmtStsRpt")]
    pub cstmr_pmt_sts_rpt: CstmrPmtStsRpt,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CstmrPmtStsRpt {
    #[serde(rename = "OrgnlPmtInfAndSts")]
    pub orgnl_pmt_inf_and_sts: Vec<OrgnlPmtInfAndSts>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgnlPmtInfAndSts {
    #[serde(rename = "OrgnlPmtInfId")]
    pub orgnl_pmt_inf_id: String,
}
