use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeribitResponse {
    pub jsonrpc: String,
    pub result: Option<LastPriceData>,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorData {
    pub message: String,
    pub data: Option<HashMap<String, Value>>,
    pub code: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeribitErrorResponse {
    pub jsonrpc: String,
    pub error: ErrorData,

    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
    pub testnet: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastPriceDataStats {
    pub volume_usd: Decimal,
    pub volume: Decimal,
    pub price_change: Decimal,
    pub low: Decimal,
    pub high: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastPriceData {
    pub best_ask_amount: Decimal,
    pub best_ask_price: Decimal,
    pub best_bid_amount: Decimal,
    pub best_bid_price: Decimal,
    pub current_funding: Decimal,
    pub estimated_delivery_price: Decimal,
    pub funding_8h: Decimal,
    pub index_price: Decimal,
    pub instrument_name: String,
    pub interest_value: Decimal,
    pub last_price: Decimal,
    pub mark_price: Decimal,
    pub max_price: Decimal,
    pub min_price: Decimal,
    pub open_interest: Decimal,
    pub settlement_price: Decimal,
    pub state: String,
    pub stats: LastPriceDataStats,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastPriceDataDetails {
    pub jsonrpc: String,
    pub result: LastPriceData,

    // #[serde(rename = "camelCase")]
    // pub us_in: Option<u64>,
    // pub us_out: Option<u64>,
    // pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DepositAddressData {
    pub address: String,
    pub creation_timestamp: u64,
    pub currency: String,
    #[serde(alias = "type")]
    pub address_type: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressDetails {
    pub jsonrpc: String,
    pub result: Option<DepositAddressData>,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Deposit {
    pub address: String,
    pub amount: Decimal,
    pub currency: String,
    pub state: String,
    pub transaction_id: String,
    pub received_timestamp: u64,
    pub updated_timestamp: u64,
    pub note: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DepositData {
    pub count: u32,
    pub data: Vec<Deposit>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositDetails {
    pub jsonrpc: String,
    pub result: DepositData,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transfer {
    pub id: u64,
    pub amount: Decimal,
    pub currency: String,
    pub direction: String,
    pub state: String,
    #[serde(alias = "type")]
    pub transfer_type: String,
    pub other_side: String,
    pub created_timestamp: u64,
    pub updated_timestamp: u64,
    pub note: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransferData {
    pub count: u32,
    pub data: Vec<Transfer>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetails {
    pub jsonrpc: String,
    pub result: TransferData,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Withdrawal {
    pub id: u64,
    pub amount: Decimal,
    pub fee: Decimal,
    pub currency: String,
    pub address: String,
    pub priority: Decimal,
    pub state: String,
    pub transaction_id: String,
    pub created_timestamp: u64,
    pub confirmed_timestamp: u64,
    pub updated_timestamp: u64,
    pub note: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WithdrawalData {
    pub count: u32,
    pub data: Vec<Withdrawal>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalDetails {
    pub jsonrpc: String,
    pub result: WithdrawalData,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub amount: Decimal,
    pub order_type: String,
    pub order_state: String,
    pub label: String,
    pub price: Decimal,
    pub average_price: Decimal,
    pub filled_amount: Decimal,
    pub profit_loss: Decimal,
    pub reduce_only: bool,
    pub web: Option<bool>,
    pub time_in_force: String,
    pub replaced: Option<bool>,
    pub post_only: bool,
    pub order_id: String,
    pub max_show: Decimal,
    pub is_liquidation: bool,
    pub instrument_name: String,
    pub direction: String,
    pub commission: Decimal,
    pub api: bool,
    pub creation_timestamp: u64,
    pub last_update_timestamp: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    pub trade_seq: u64,
    pub trade_id: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub fee: Decimal,
    pub label: String,
    pub instrument_name: String,
    pub index_price: Decimal,
    pub fee_currency: String,
    pub direction: String,
    pub tick_direction: u64,
    pub state: String,
    pub self_trade: bool,
    pub reduce_only: bool,
    pub post_only: bool,
    pub order_type: String,
    pub order_id: String,
    pub matching_id: Option<String>,
    pub mark_price: Decimal,
    pub liquidity: String,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderData {
    pub order: Order,
    pub trades: Vec<Trade>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    pub jsonrpc: String,
    pub result: OrderData,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderStateDetails {
    pub jsonrpc: String,
    pub result: Order,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Position {
    pub average_price: Decimal,
    pub delta: Decimal,
    pub direction: String,
    pub estimated_liquidation_price: Decimal,
    pub floating_profit_loss: Decimal,
    pub index_price: Decimal,
    pub initial_margin: Decimal,
    pub instrument_name: String,
    pub interest_value: Decimal,
    pub leverage: Decimal,
    pub kind: String,
    pub maintenance_margin: Decimal,
    pub mark_price: Decimal,
    pub open_orders_margin: Decimal,
    pub realized_profit_loss: Decimal,
    pub settlement_price: Decimal,
    pub size: Decimal,
    pub size_currency: Decimal,
    pub total_profit_loss: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetails {
    pub jsonrpc: String,
    pub result: Position,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountSummary {
    pub balance: Decimal,
    pub options_session_upl: Decimal,
    pub deposit_address: String,
    pub options_gamma: Decimal,
    pub options_theta: Decimal,
    pub username: String,
    pub equity: Decimal,
    #[serde(alias = "type")]
    pub account_type: String,
    pub currency: String,
    pub delta_total: Decimal,
    pub futures_session_rpl: Decimal,
    pub portfolio_margining_enabled: bool,
    pub total_pl: Decimal,
    pub margin_balance: Decimal,
    pub security_keys_enabled: bool,
    pub options_session_rpl: Decimal,
    pub options_delta: Decimal,
    pub futures_pl: Decimal,
    pub referrer_id: Option<String>,
    pub id: u64,
    pub session_upl: Decimal,
    pub available_withdrawal_funds: Decimal,
    pub creation_timestamp: u64,
    pub options_pl: Decimal,
    pub system_name: String,
    pub limits: HashMap<String, Value>,
    pub initial_margin: Decimal,
    pub projected_initial_margin: Decimal,
    pub maintenance_margin: Decimal,
    pub projected_maintenance_margin: Decimal,
    pub session_rpl: Decimal,
    pub interuser_transfers_enabled: bool,
    pub options_vega: Decimal,
    pub projected_delta_total: Decimal,
    pub email: String,
    pub futures_session_upl: Decimal,
    pub available_funds: Decimal,
    pub options_value: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummaryDetails {
    pub jsonrpc: String,
    pub result: AccountSummary,

    pub us_in: Option<u64>,
    pub us_out: Option<u64>,
    pub us_diff: Option<u64>,
    pub testnet: Option<bool>,
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::Currency;

    use super::*;

    #[test]
    fn last_price_data() {
        let response_text =
            "{\"jsonrpc\":\"2.0\",\"result\":{\"timestamp\":1674461497839,\"stats\":{\"volume_usd\":359696880.0,\"volume\":15819.90886478,\"price_change\":-0.5074,\"low\":22133.0,\"high\":23331.0},\"state\":\"open\",\"settlement_price\":22700.49,\"open_interest\":4126543180,\"min_price\":22407.0,\"max_price\":23089.45,\"mark_price\":22746.89,\"last_price\":22745.0,\"interest_value\":0.0000648967073158397,\"instrument_name\":\"BTC-PERPETUAL\",\"index_price\":22740.69,\"funding_8h\":-0.00004213,\"estimated_delivery_price\":22740.69,\"current_funding\":0.0,\"best_bid_price\":22745.0,\"best_bid_amount\":42880.0,\"best_ask_price\":22745.5,\"best_ask_amount\":62660.0},\"usIn\":1674461498693343,\"usOut\":1674461498693470,\"usDiff\":127,\"testnet\":true}";
        let details = serde_json::from_str::<LastPriceDataDetails>(response_text).unwrap();
        assert_eq!(details.result.last_price, dec!(22745.0));
    }

    #[test]
    fn btc_on_chain_deposit_address_details_empty() {
        let response_text = "{\"jsonrpc\": \"2.0\", \"result\": null, \"usIn\": 1674807312116504, \"usOut\": 1674807312116808, \"usDiff\": 304, \"testnet\": true}";
        let details = serde_json::from_str::<DepositAddressDetails>(response_text).unwrap();
        assert!(details.result.is_none())
    }

    #[test]
    fn btc_on_chain_deposit_address_details() {
        let response_text = "{\"jsonrpc\": \"2.0\", \"id\": 3461, \"usIn\": 1674807312116504, \"usOut\": 1674807312116808, \"usDiff\": 304, \"result\": {\"address\": \"2N8udZGBc1hLRCFsU9kGwMPpmYUwMFTuCwB\", \"creation_timestamp\": 1550575165170, \"currency\": \"BTC\", \"type\": \"deposit\"}, \"testnet\": true}";
        let details = serde_json::from_str::<DepositAddressDetails>(response_text).unwrap();
        if let Some(data) = details.result {
            assert_eq!(data.address, "2N8udZGBc1hLRCFsU9kGwMPpmYUwMFTuCwB");
        } else {
            panic!()
        }
    }

    #[test]
    fn get_deposits_empty() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"data\": [],\"count\": 0},\"usIn\": 1675065077579453,\"usOut\": 1675065077579739,\"usDiff\": 286,\"testnet\": true}";
        let details = serde_json::from_str::<DepositDetails>(response_text).unwrap();
        assert_eq!(details.result.count, 0);
        assert!(details.result.data.is_empty());
    }

    #[test]
    fn get_deposits() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"data\": [{\"updated_timestamp\": 1675062738879,\"transaction_id\": \"878b71d6b5f2221bb6e52090c55f27dc330d838efc096731d481b86091358e51\",\"state\": \"completed\",\"received_timestamp\": 1675062700792,\"note\": \"\",\"currency\": \"BTC\",\"amount\": 0.9,\"address\": \"bcrt1qxqrpgcsneyt6zwgujakphyuqupl5g75rzlthxa\"}],\"count\": 1},\"usIn\": 1675063983490363,\"usOut\": 1675063983490583,\"usDiff\": 220,\"testnet\": true}";
        let details = serde_json::from_str::<DepositDetails>(response_text).unwrap();
        assert!(!details.result.data.is_empty());
        assert_eq!(
            details.result.data[0].address,
            "bcrt1qxqrpgcsneyt6zwgujakphyuqupl5g75rzlthxa"
        );
    }

    #[test]
    fn get_transfers_empty() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"data\": [],\"count\": 0},\"usIn\": 1675065077579453,\"usOut\": 1675065077579739,\"usDiff\": 286,\"testnet\": true}";
        let details = serde_json::from_str::<TransferDetails>(response_text).unwrap();
        assert_eq!(details.result.count, 0);
        assert!(details.result.data.is_empty());
    }

    #[test]
    fn get_transfers() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"count\": 1,\"data\": [{\"id\": 291951,\"amount\": 0.1,\"currency\": \"BTC\",\"direction\": \"payment\",\"state\": \"confirmed\",\"type\": \"subaccount\",\"other_side\": \"new_user_1_1\",\"created_timestamp\": 1675065819382,\"updated_timestamp\": 1675065819382,\"note\": \"\"}]},\"usIn\": 1675065835747899,\"usOut\": 1675065835748142,\"usDiff\": 243,\"testnet\": true}";
        let details = serde_json::from_str::<TransferDetails>(response_text).unwrap();
        assert!(!details.result.data.is_empty());
        assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }

    #[test]
    fn get_withdrawals_empty() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"data\": [],\"count\": 0},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let details = serde_json::from_str::<WithdrawalDetails>(response_text).unwrap();
        assert_eq!(details.result.count, 0);
        assert!(details.result.data.is_empty());
    }

    #[test]
    fn get_withdrawals() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"result\": {\"data\": [],\"count\": 0},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let _details = serde_json::from_str::<WithdrawalDetails>(response_text).unwrap();
        // assert!(!details.result.data.is_empty());
        // assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }

    #[test]
    fn order() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"id\": 5275,\"result\": {\"trades\": [{\"trade_seq\": 1966056,\"trade_id\": \"ETH-2696083\",\"timestamp\": 1590483938456,\"tick_direction\": 0,\"state\": \"filled\",\"self_trade\": false,\"reduce_only\": false,\"price\": 203.3,\"post_only\": false,\"order_type\": \"market\",\"order_id\": \"ETH-584849853\",\"matching_id\": null,\"mark_price\": 203.28,\"liquidity\": \"T\",\"label\": \"market0000234\",\"instrument_name\": \"ETH-PERPETUAL\",\"index_price\": 203.33,\"fee_currency\": \"ETH\",\"fee\": 0.00014757,\"direction\": \"buy\",\"amount\": 40}],\"order\": {\"web\": false,\"time_in_force\": \"good_til_cancelled\",\"replaced\": false,\"reduce_only\": false,\"profit_loss\": 0.00022929,\"price\": 207.3,\"post_only\": false,\"order_type\": \"market\",\"order_state\": \"filled\",\"order_id\": \"ETH-584849853\",\"max_show\": 40,\"last_update_timestamp\": 1590483938456,\"label\": \"market0000234\",\"is_liquidation\": false,\"instrument_name\": \"ETH-PERPETUAL\",\"filled_amount\": 40,\"direction\": \"buy\",\"creation_timestamp\": 1590483938456,\"commission\": 0.00014757,\"average_price\": 203.3,\"api\": true,\"amount\": 40}},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let _details = serde_json::from_str::<OrderDetails>(response_text).unwrap();
        // assert!(!details.result.data.is_empty());
        // assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }

    #[test]
    fn order_state() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"id\": 4316,\"result\": {\"time_in_force\": \"good_til_cancelled\",\"reduce_only\": false,\"profit_loss\": 0.051134,\"price\": 118.94,\"post_only\": false,\"order_type\": \"limit\",\"order_state\": \"filled\",\"order_id\": \"ETH-331562\",\"max_show\": 37,\"last_update_timestamp\": 1550219810944,\"label\": \"\",\"is_liquidation\": false,\"instrument_name\": \"ETH-PERPETUAL\",\"filled_amount\": 37,\"direction\": \"sell\",\"creation_timestamp\": 1550219749176,\"commission\": 0.000031,\"average_price\": 118.94,\"api\": false,\"amount\": 37},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let _details = serde_json::from_str::<OrderStateDetails>(response_text).unwrap();
        // assert!(!details.result.data.is_empty());
        // assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }

    #[test]
    fn get_position() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"id\": 404,\"result\": {\"average_price\": 0,\"delta\": 0,\"direction\": \"buy\",\"estimated_liquidation_price\": 0,\"floating_profit_loss\": 0,\"index_price\": 3555.86,\"initial_margin\": 0,\"instrument_name\": \"BTC-PERPETUAL\",\"interest_value\" : 1.7362511643080387,\"leverage\": 100,\"kind\": \"future\",\"maintenance_margin\": 0,\"mark_price\": 3556.62,\"open_orders_margin\": 0.000165889,\"realized_profit_loss\": 0,\"settlement_price\": 3555.44,\"size\": 0,\"size_currency\": 0,\"total_profit_loss\": 0},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let _details = serde_json::from_str::<PositionDetails>(response_text).unwrap();
        // assert!(!details.result.data.is_empty());
        // assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }

    #[test]
    fn get_account_summary() {
        let response_text = "{\"jsonrpc\": \"2.0\",\"id\": 2515,\"result\": {\"balance\": 118.72074005,\"options_session_upl\": 0,\"deposit_address\": \"2NC9eNLq1z3MFuZGVp2JgSCATeDzLqwpcY7\",\"options_gamma\": 0,\"options_theta\": 0,\"username\": \"user\",\"equity\": 118.77720303,\"type\": \"main\",\"currency\": \"BTC\",\"delta_total\": -11.1895,\"futures_session_rpl\": -0.00011454,\"portfolio_margining_enabled\": false,\"total_pl\": -3.46418369,\"margin_balance\": 118.77720303,\"security_keys_enabled\": false,\"options_session_rpl\": 0,\"options_delta\": 0,\"futures_pl\": -3.46418369,\"referrer_id\": null,\"id\": 3,\"session_upl\": 0.05657752,\"available_withdrawal_funds\": 118.38439069,\"creation_timestamp\": 1594388820315,\"options_pl\": 0,\"system_name\": \"user\",\"limits\": {\"non_matching_engine\": {\"rate\": 30,\"burst\": 400},\"matching_engine\": {\"rate\": 5,\"burst\": 20}},\"initial_margin\": 0.33634936,\"projected_initial_margin\": 0.33634936,\"maintenance_margin\": 0.24683366,\"projected_maintenance_margin\": 0.24683366,\"session_rpl\": -0.00011454,\"interuser_transfers_enabled\": false,\"options_vega\": 0,\"projected_delta_total\": -11.1895,\"email\": \"user@example.com\",\"futures_session_upl\": 0.05657752,\"available_funds\": 118.44085367,\"options_value\": 0},\"usIn\": 1675066247615127,\"usOut\": 1675066247615279,\"usDiff\": 152,\"testnet\": true}";
        let _details = serde_json::from_str::<AccountSummaryDetails>(response_text).unwrap();
        // assert!(!details.result.data.is_empty());
        // assert_eq!(details.result.data[0].currency, Currency::BTC.to_string(),);
    }
}
