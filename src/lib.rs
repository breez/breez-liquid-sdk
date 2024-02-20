#![allow(unused_variables, dead_code)]

mod simulator;
mod utils;

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use cln_rpc::{
        model::requests::{InvoiceRequest, PayRequest},
        primitives::AmountOrAny,
    };
    use dotenv::dotenv;
    use std::{sync::Arc, thread};

    use crate::{simulator::Simulator, utils::check_var_set};

    /// User swaps L-BTC for Lightning funds
    #[tokio::test]
    async fn normal_submarine_swap() -> Result<()> {
        env_logger::init();
        dotenv().ok();

        // Initialize the simulator
        let amount_sat = 10000u64;
        let sim = Arc::new(Simulator::try_init().await?);

        // Fund nodes
        let mut liquid_handle = None;
        if check_var_set("SWAPPER_FUND_LIQUID") {
            let sim_p = sim.clone();
            liquid_handle = Some(thread::spawn(move || {
                sim_p.fund_liquid(sim_p.swapper.liquid_wallet.clone())
            }));
        }

        if check_var_set("USER_FUND_LN") {
            sim.fund_ln(sim.user.ln_rpc.clone(), amount_sat + 2500)
                .await?;
        }

        if let Some(handle) = liquid_handle {
            handle.join().unwrap()?;
        }

        // Begin normal swap
        let mut user_ln = sim.user.ln_rpc.lock().await;
        let user_liquid = sim.user.liquid_wallet.lock().unwrap();

        let mut swapper_ln = sim.swapper.ln_rpc.lock().await;
        let mut swapper_liquid = sim.swapper.liquid_wallet.lock().unwrap();

        // User - Create lightning invoice and forward it to swapper
        let response = user_ln
            .call(cln_rpc::Request::Invoice(InvoiceRequest {
                amount_msat: AmountOrAny::Amount(cln_rpc::primitives::Amount::from_sat(amount_sat)),
                description: "swap me".to_string(),
                label: uuid::Uuid::new_v4().to_string(),
                expiry: None,
                fallbacks: None,
                preimage: None,
                cltv: None,
                deschashonly: None,
            }))
            .await?;

        let (bolt11, payment_hash) = match response {
            cln_rpc::Response::Invoice(res) => (res.bolt11, res.payment_hash),
            _ => return Err(anyhow!("Could not generate swap invoice")),
        };

        // Swapper - Use invoice payment hash to create redeem script for Liquid funds to specific address
        // Forward said address to the user
        let target_addr = Simulator::create_redeem_address(payment_hash.to_string())?;

        // User - Sends funds to the address
        user_liquid.send_lbtc(amount_sat, &target_addr, None)?;

        // Swapper - Poll for changes, wait for user to send Liquid funds (held)
        // Wait for one confirmation (or more)

        // TODO monitor target address for fund change

        sim.liquid_await_n_confirmations(1)?;

        // Swapper - Pay the invoice
        swapper_ln
            .call(cln_rpc::Request::Pay(PayRequest {
                bolt11,
                amount_msat: None,
                label: None,
                riskfactor: None,
                maxfeepercent: None,
                retry_for: None,
                maxdelay: None,
                exemptfee: None,
                localinvreqid: None,
                exclude: None,
                maxfee: None,
                description: None,
            }))
            .await?;

        // Verify, subtracting possible fees
        assert!(dbg!(sim.get_ln_balance(&mut user_ln).await?) > amount_sat - 1000);
        assert!(dbg!(sim.get_liquid_balance(&mut swapper_liquid)?) > amount_sat - 1000);

        Ok(())
    }

    /// User swaps Lightning funds for L-BTC
    #[tokio::test]
    async fn reverse_submarine_swap() -> Result<()> {
        Ok(())
    }
}
