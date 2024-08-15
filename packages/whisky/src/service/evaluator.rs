use async_trait::async_trait;
use cardano_serialization_lib::JsError;

use sidan_csl_rs::model::{
    Action, Certificate, MintItem, Redeemer, RedeemerTag, ScriptTxIn, TxIn, UTxO, Withdrawal,
};

use crate::builder::MeshTxBuilder;

#[async_trait]
pub trait Evaluator: Send {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],
        additional_txs: &[String],
    ) -> Result<Vec<Action>, JsError>;
}

pub trait TxEvaluation {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self;
}

impl TxEvaluation for MeshTxBuilder {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self {
        let multiplier = self.core.tx_evaluation_multiplier_percentage;
        for redeemer_evaluation in tx_evaluation {
            match redeemer_evaluation.tag {
                RedeemerTag::Spend => {
                    let input = &mut self.core.mesh_tx_builder_body.inputs
                        [redeemer_evaluation.index as usize];
                    if let TxIn::ScriptTxIn(ScriptTxIn { script_tx_in, .. }) = input {
                        let redeemer: &mut Redeemer = script_tx_in.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Mint => {
                    let mint_item = &mut self.core.mesh_tx_builder_body.mints
                        [redeemer_evaluation.index as usize];
                    if let MintItem::ScriptMint(mint) = mint_item {
                        let redeemer: &mut Redeemer = mint.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Cert => {
                    let cert_item = &mut self.core.mesh_tx_builder_body.certificates
                        [redeemer_evaluation.index as usize];
                    if let Certificate::ScriptCertificate(cert) = cert_item {
                        let redeemer: &mut Redeemer = cert.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Reward => {
                    let withdrawal_item = &mut self.core.mesh_tx_builder_body.withdrawals
                        [redeemer_evaluation.index as usize];
                    if let Withdrawal::PlutusScriptWithdrawal(withdrawal) = withdrawal_item {
                        let redeemer: &mut Redeemer = withdrawal.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
            }
        }
        self
    }
}
