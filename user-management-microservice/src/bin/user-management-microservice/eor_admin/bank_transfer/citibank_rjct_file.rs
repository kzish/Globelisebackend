use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankRJCTFile {
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
    #[serde(rename = "TxInfAndSts")]
    pub tx_inf_and_sts: TxInfAndSts,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TxInfAndSts {
    #[serde(rename = "OrgnlEndToEndId")]
    pub orgnl_end_to_end_id: String,
    #[serde(rename = "TxSts")]
    pub tx_sts: String,
    #[serde(rename = "StsRsnInf")]
    pub sts_rsn_inf: Vec<StsRsnInf>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct StsRsnInf {
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Vec<String>,
}
