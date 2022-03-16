#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusPayload {
    pub client_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Option<String>,
    pub daily_limit: Option<String>,
    pub daily_remaining: Option<String>,
    pub monthly_limit: Option<String>,
    pub monthly_remaining: Option<String>,
    pub cash_deposit_allowance: Option<String>,
    pub cellphone_number: Option<String>,
    pub cellphone_number_stored: Option<String>,
    pub email_stored: Option<String>,
    pub official_id: Option<String>,
    pub proof_of_residency: Option<String>,
    pub signed_contract: Option<String>,
    pub origin_of_funds: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub success: bool,
    pub payload: Balances,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balances {
    pub balances: Vec<AccountBalanceInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalanceInstance {
    pub currency: Option<String>,
    pub available: Option<String>,
    pub locked: Option<String>,
    pub total: Option<String>,
    pub pending_deposit: Option<String>,
    pub pending_withdrawal: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeesPayload {
    pub fees: Vec<BookFee>,
    pub withdrawal_fees: WithdrawalFees,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookFee {
    pub book: Option<String>,
    pub taker_fee_decimal: Option<String>,
    pub taker_fee_percent: Option<String>,
    pub maker_fee_decimal: Option<String>,
    pub maker_fee_percent: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalFees {
    pub btc: Option<String>,
    pub eth: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerInstance {
    pub eid: Option<String>,
    pub operation: Option<String>,
    pub created_at: Option<String>,
    pub balance_updates: Vec<BalanceUpdate>,
    pub details: BalanceDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceUpdate {
    pub currency: Option<String>,
    pub amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceDetails {
    pub tid: Option<u64>,
    pub oid: Option<String>,
    pub fid: Option<String>,
    pub wid: Option<String>,
    pub method: Option<String>,
    pub method_name: Option<String>,
    pub funding_address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalsPayload {
    pub wid: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub amount: Option<String>,
    pub details: WithdrawalDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalDetails {
    pub withdrawal_address: Option<String>,
    pub tx_hash: Option<String>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_bank: Option<String>,
    pub beneficiary_clabe: Option<String>,
    pub numeric_reference: Option<String>,
    pub concepto: Option<String>,
    pub clave_rastreo: Option<String>,
    pub cep: Option<Cep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cep {
    pub r#return: Option<CepReturn>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CepReturn {
    pub cda: Option<Cda>,
    pub estado_consulta: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cda {
    pub cadena_original: Option<String>,
    pub concepto_pago: Option<String>,
    pub cuenta_beneficiario: Option<String>,
    pub cuenta_ordenante: Option<String>,
    pub fecha_captura: Option<String>,
    pub fecha_operacion: Option<String>,
    pub hora: Option<String>,
    pub iva: Option<String>,
    pub monto: Option<String>,
    pub nombre_beneficiario: Option<String>,
    pub nombre_inst_beneficiaria: Option<String>,
    pub nombre_inst_ordenante: Option<String>,
    pub nombre_ordenante: Option<String>,
    pub sello_digital: Option<String>,
    pub serie_certificado: Option<String>,
    pub tipo_operacion: Option<String>,
    pub tipo_pago: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingsPayload {
    pub fid: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub amount: Option<String>,
    pub details: FundingDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingDetails {
    pub funding_address: Option<String>,
    pub tx_hash: Option<String>,
    pub sender_name: Option<String>,
    pub sender_bank: Option<String>,
    pub sender_clabe: Option<String>,
    pub receive_clabe: Option<String>,
    pub numeric_reference: Option<String>,
    pub concepto: Option<String>,
    pub clave_rastreo: Option<String>,
    pub beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserTradesPayload {
    pub book: Option<String>,
    pub major: Option<String>,
    pub created_at: Option<String>,
    pub minor: Option<String>,
    pub fees_amount: Option<String>,
    pub fees_currency: Option<String>,
    pub price: Option<String>,
    pub tid: Option<u64>,
    pub oid: Option<String>,
    pub side: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderTradesPayload {
    pub book: Option<String>,
    pub major: Option<String>,
    pub created_at: Option<String>,
    pub minor: Option<String>,
    pub fees_amount: Option<String>,
    pub fees_currency: Option<String>,
    pub price: Option<String>,
    pub tid: Option<u64>,
    pub oid: Option<String>,
    pub client_id: Option<String>,
    pub side: Option<String>,
    pub maker_side: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenOrdersPayload {
    pub book: Option<String>,
    pub original_amount: Option<String>,
    pub unfilled_amount: Option<String>,
    pub original_value: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub price: Option<String>,
    pub oid: Option<String>,
    pub client_id: Option<String>,
    pub side: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LookupOrdersPayload {
    pub book: Option<String>,
    pub original_amount: Option<String>,
    pub unfilled_amount: Option<String>,
    pub original_value: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub price: Option<String>,
    pub oid: Option<String>,
    pub side: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceOrderPayload {
    pub oid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingDestination {
    pub account_identifier_name: Option<String>,
    pub account_identifier: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Withdrawal<T> {
    pub wid: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub amount: Option<String>,
    pub details: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoWithdrawal {
    pub withdrawal_address: Option<String>,
    pub tx_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SPEIWithdrawal {
    pub sender_name: Option<String>,
    pub receive_clabe: Option<String>,
    pub sender_clabe: Option<String>,
    pub numeric_reference: Option<String>,
    pub concepto: Option<String>,
    pub clave_rastreo: Option<String>,
    pub beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebitWithdrawal {
    pub sender_name: Option<String>,
    pub receive_clabe: Option<String>,
    pub sender_clabe: Option<String>,
    pub numeric_reference: Option<String>,
    pub concepto: Option<String>,
    pub clave_rastreo: Option<String>,
    pub beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhoneWithdrawal {
    pub sender_name: Option<String>,
    pub receive_clabe: Option<String>,
    pub sender_clabe: Option<String>,
    pub numeric_reference: Option<String>,
    pub concepto: Option<String>,
    pub clave_rastreo: Option<String>,
    pub beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankCode {
    pub code: Option<String>,
    pub name: Option<String>,
}
