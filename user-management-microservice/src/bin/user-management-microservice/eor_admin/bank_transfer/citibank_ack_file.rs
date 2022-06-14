use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CitiBankACKFile {
    #[serde(rename = "CstmrPmtStsRpt")]
    pub cstmr_pmt_sts_rpt: CstmrPmtStsRpt,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CstmrPmtStsRpt {
    #[serde(rename = "OrgnlGrpInfAndSts")]
    pub orgnl_grp_inf_and_sts: OrgnlGrpInfAndSts,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct StsRsnInf {
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrgnlGrpInfAndSts {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: String,
    #[serde(rename = "GrpSts")]
    pub grp_sts: String,
    #[serde(rename = "StsRsnInf")]
    pub sts_rsn_inf: StsRsnInf,
}
