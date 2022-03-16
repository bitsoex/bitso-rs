extern crate bitsors;
extern crate mockito;
#[macro_use]
extern crate lazy_static;

use bitsors::auth::BitsoCredentials;
use bitsors::client::{Bitso, OptionalOrderParams, OptionalParams};
use mockito::{mock, Matcher};
use std::sync::Mutex;

lazy_static! {
    // Set api_key and api_secret in .env file or
    // export API_KEY="key"
    // export API_SECRET="secret"
    static ref CLIENT_CREDENTIAL: Mutex<BitsoCredentials> = Mutex::new(BitsoCredentials::default().build());
}

/// *** PRIVATE API *** ///

/// Test unsuccessful request due to empty credentials
#[tokio::test]
async fn test_empty_credentials() {
    let bitso = Bitso::default().prefix("https://api-dev.bitso.com").build();
    let result = bitso.get_account_status().await;
    assert!(result.is_err()); // Empty credentials
    println!("{:?}", result);
}

/// Test successful request to get account status
#[tokio::test]
async fn test_account_status() {
    let _mock = mock("GET", "/v3/account_status/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "client_id": "1234",
                "first_name": "Claude",
                "last_name":  "Shannon",
                "status": "active",
                "daily_limit": "5300.00",
                "monthly_limit": "32000.00",
                "daily_remaining": "3300.00",
                "monthly_remaining": "31000.00",
                "cash_deposit_allowance": "5300.00",
                "cellphone_number": "verified",
                "cellphone_number_stored":"+525555555555",
                "email_stored":"shannon@maxentro.py",
                "official_id": "submitted",
                "proof_of_residency": "submitted",
                "signed_contract": "unsubmitted",
                "origin_of_funds": "unsubmitted"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_account_status().await;
    assert!(result.is_ok());
    println!("{:?}", result);
    // Test that the result's contents can be reached
    let client_id = result.unwrap().payload.client_id;
    assert_eq!(client_id, Some("1234".to_owned()));
}

/// Test successful request to get account balance
#[tokio::test]
async fn test_account_balance() {
    let _mock = mock("GET", "/v3/balance/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "balances": [{
                    "currency": "mxn",
                    "total": "100.1234",
                    "locked": "25.1234",
                    "available": "75.0000"
                }, {
                    "currency": "btc",
                    "total": "4.12345678",
                    "locked": "25.00000000",
                    "available": "75.12345678"
                }, {
                    "currency": "eth",
                    "total": "50.1234",
                    "locked": "40.1234",
                    "available": "10.0000"
                }]
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_account_balance().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get fees
#[tokio::test]
async fn test_fees() {
    let _mock = mock("GET", "/v3/fees/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "fees": [{
                    "book": "btc_mxn",
                    "taker_fee_decimal": "0.0001",
                    "taker_fee_percent": "0.01",
                    "maker_fee_decimal": "0.0001",
                    "maker_fee_percent": "0.01"
                }, {
                    "book": "eth_mxn",
                    "taker_fee_decimal": "0.0001",
                    "taker_fee_percent": "0.01",
                    "maker_fee_decimal": "0.0001",
                    "maker_fee_percent": "0.01"
                }],
                "withdrawal_fees": {
                    "btc": "0.001",
                    "eth": "0.0025"
                }
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_fees().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get ledger
#[tokio::test]
async fn test_ledger() {
    let _mock = mock("GET", "/v3/ledger/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "eid": "c4ca4238a0b923820dcc509a6f75849b",
                "operation": "trade",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "btc",
                    "amount": "-0.25232073"
                }, {
                    "currency": "mxn",
                    "amount": "1013.540958479115"
                }],
                "details": {
                    "tid": 51756,
                    "oid": "wri0yg8miihs80ngk"
                }
            }, {
                "eid": "6512bd43d9caa6e02c990b0a82652dca",
                "operation": "fee",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "mxn",
                    "amount": "-10.237787459385"
                }],
                "details": {
                    "tid": 51756,
                    "oid": "19vaqiv72drbphig"
                }
            }, {
                "operation": "trade",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "eth",
                    "amount": "4.86859395"
                }, {
                    "currency": "mxn",
                    "amount": "-626.77"
                }],
                "details": {
                    "tid": 51757,
                    "oid": "19vaqiv72drbphig"
                }
            }, {
                "eid": "698d51a19d8a121ce581499d7b701668",
                "operation": "fee",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "eth",
                    "amount": "0.04917771"
                }],
                "details": {
                    "tid": 51757,
                    "oid": "19vaqiv72drbphig"
                }
            }, {
                "eid": "b59c67bf196a4758191e42f76670ceba",
                "operation": "funding",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "btc",
                    "amount": "0.48650929"
                }],
                "details": {
                    "fid": "fc23c28a23905d8614499816c3ade455",
                    "method": "btc",
                    "funding_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp"
                }
            }, {
                "eid": "b0baee9d279d34fa1dfd71aadb908c3f",
                "operation": "funding",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "mxn",
                    "amount": "300.15"
                }],
                "details": {
                    "fid": "3ef729ccf0cc56079ca546d58083dc12",
                    "method": "sp"
                }

            }, {
                "eid": "96e79218965eb72c92a549dd5a330112",
                "operation": "withdrawal",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "mxn",
                    "amount": "-200.15"
                }],
                "details": {
                    "wid": "c5b8d7f0768ee91d3b33bee648318688",
                    "method": "sp"
                }
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_ledger(None, None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get ledger
/// with operation_type = trades and optional
/// parameters.
#[tokio::test]
async fn test_ledger_with_optional_params() {
    let _mock = mock("GET", "/v3/ledger/trades/")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("marker".into(), "51755".into()),
            Matcher::UrlEncoded("sort".into(), "asc".into()),
            Matcher::UrlEncoded("limit".into(), "1".into()),
        ]))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "eid": "c4ca4238a0b923820dcc509a6f75849b",
                "operation": "trade",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "balance_updates": [{
                    "currency": "btc",
                    "amount": "-0.25232073"
                }, {
                    "currency": "mxn",
                    "amount": "1013.540958479115"
                }],
                "details": {
                    "tid": 51756,
                    "oid": "wri0yg8miihs80ngk"
                }
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let optional_params = OptionalParams {
        marker: Some(&51755),
        sort: Some("asc"),
        limit: Some(&1),
    };
    let result = bitso
        .get_ledger(Some("trades"), Some(optional_params))
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get withdrawals
#[tokio::test]
async fn test_withdrawals() {
    let _mock = mock("GET", "/v3/withdrawals/")
        .with_status(200)
        .with_body(r#"{
            "success": true,
            "payload": [{
                "wid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "Bitcoin",
                "amount": "0.48650929",
                "details": {
                    "withdrawal_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp",
                    "tx_hash": "d4f28394693e9fb5fffcaf730c11f32d1922e5837f76ca82189d3bfe30ded433"
                }
            }, {
                "wid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "complete",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "2612.70",
                "details": {
                    "beneficiary_name": "BERTRAND RUSSELL",
                    "beneficiary_bank": "BANAMEX",
                    "beneficiary_clabe": "002320700708015728",
                    "numeric_reference": "99548",
                    "concepto": "Por los 🌮 del viernes",
                    "clave_rastreo": "BNET01001604080002076841",
                    "cep": {
                        "return": {
                            "cda": {
                                "cadenaOriginal": "||1|13062016|13062016|172053|40002|STP|Bitso - BERTRAND RUSSELL|40|646180115400000002|BIT140123U70|BANAMEX|BERTRAND RUSSELL|40|002320700708015728|ND|-|0.00|2612.70|00001000000401205824||",
                                "conceptoPago": "-",
                                "cuentaBeneficiario": "002320700708015728",
                                "cuentaOrdenante": "646180115400000002",
                                "fechaCaptura": "20160613",
                                "fechaOperacion": "20160613",
                                "hora": "17:08:42",
                                "iva": "0.00",
                                "monto": "2612.70",
                                "nombreBeneficiario": "BERTRAND RUSSELL",
                                "nombreInstBeneficiaria": "BANAMEX",
                                "nombreInstOrdenante": "STP",
                                "nombreOrdenante": "Bitso - Russell",
                                "referenciaNumerica": "99548",
                                "rfcCurpBeneficiario": "ND",
                                "rfcCurpOrdenante": "BIT140123U70",
                                "selloDigital": "cd7yUrnmUQ7CG6M+LX7WOZeizOpkTyMlEAunJaP2j5MAaNPZxy+vAJtgiVL73i1LNSrwK10eBb66Rh4\/RxU6AT2S03chQ\/BS1beknH5xPpGQg+wEXeANtnF2lp71lAD6QZ2O0NE4MIDvLhGGjTGklSP+2fS6joTAaV+tLbtrIp8JiR0MOX1rGPC5h+0ZHNvXQkcHJz3s68+iUAvDnQBiSu768b2C4zpHzteGEnJhU8sAdk83spiWogKALAVAuN4xfSXni7GTk9HObTTRdY+zehfWVPdE\/7uQSmMTzOKfPbQU02Jn\/5DdE3gYk6JZ5m70JsUSFBTF\/EVX8hhg0pu2iA==",
                                "serieCertificado": "",
                                "tipoOperacion": "C",
                                "tipoPago": "1"
                            },
                            "estadoConsulta": "1",
                            "url": "http:\/\/www.banxico.org.mx\/cep?i=90646&s=20150825&d=viAKjS0GVYB8qihmG9I%2B9O1VUvrR2td%2Fuo3GyVDn8vBp371tVx5ltRnk4QsWP6KP%2BQvlWjT%2BzfwWWTA3TMk4tg%3D%3D"
                        }
                    }
                }
            }, {
                "wid": "of40d7f0768ee91d3b33bee64831jg73",
                "status": "complete",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "500.00",
                "details": {
                    "beneficiary_name": "ALFRED NORTH WHITEHEAD",
                    "beneficiary_bank": "BANAMEX",
                    "beneficiary_clabe": "5204165009315197",
                    "numeric_reference": "30535",
                    "concepto": "-",
                    "clave_rastreo": "BNET01001604080002076841",
                    "cep": {
                        "return": {
                            "cda": {
                                "cadenaOriginal": "||1|07042016|07042016|095656|40002|STP|Bitso - Al|40|646180115400000002|BIT140123U70|BANAMEX|ALFRED NORTH WHITEHEAD|3|5204165009315197|ND|-|0.00|500.00|00001000000401205824||",
                                "conceptoPago": "-",
                                "cuentaBeneficiario": "5204165009315197",
                                "cuentaOrdenante": "646180115400000002",
                                "fechaCaptura": "20160407",
                                "fechaOperacion": "20160407",
                                "hora": "09:56:51",
                                "iva": "0.00",
                                "monto": "500.00",
                                "nombreBeneficiario": "ALFRED NORTH WHITEHEAD",
                                "nombreInstBeneficiaria": "BANAMEX",
                                "nombreInstOrdenante": "STP",
                                "nombreOrdenante": "Bitso - RUSSELL",
                                "referenciaNumerica": "30535",
                                "rfcCurpBeneficiario": "ND",
                                "rfcCurpOrdenante": "BIT140123U70",
                                "selloDigital": "GaXpeaKgkc+gc0w9XgBbRCMmKWLNdSTV5C4CNQ4DL4ZVT+1OBSqNtX\/pv2IGjI7bKjCkaNrKUdaCdFwG6SdZ0nS9KtYSx1Ewg2Irg6x4kSzeHdlzBDr6ygT+bb+weizxcXMARKkciPuSQlyltCrEwSi07yVzachKfcEN8amj2fsEzim7gSyUc3ecKA1n8DX89158fwukKTIg4ECfOLsgueKF8unwbICWHXwRaaxIAA6PVw7O6WwGXxMtMBTCdiT202c8I2SnULFqK9QVJlQ\/YDRXFI4IMMAwGQZWbbmk8gf\/J3Fixy+0lcQV35TBBrbHyFPiaHaRN95yK\/BUxPOhag==",
                                "serieCertificado": "",
                                "tipoOperacion": "C",
                                "tipoPago": "1"
                            },
                            "estadoConsulta": "1",
                            "url": "http:\/\/www.banxico.org.mx\/cep?i=90646&s=20150825&d=3AeATtn9mM9yySMqwClgSTnKIddFN7JVwo38kDBVjOBRtcYVENx1LblV%2BXOHnKEGTfp0g%2BVLM76C3ewQ0c9vpA%3D%3D"
                        }
                    },
                    "folio_origen": "BITSO4405016499736144"
                }
            }]
        }"#)
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_withdrawals(None, None, None, None, None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get withdrawals with optional params
/// This tests the case where a list of WIDs is provided.
#[tokio::test]
async fn test_withdrawals_wids() {
    let _mock = mock("GET", "/v3/withdrawals/")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("wids".into(), "c5b8d7f0768ee91d3b33bee648318688,p4u8d7f0768ee91d3b33bee6483132i8".into()),
        ]))
        .with_status(200)
        .with_body(r#"{
            "success": true,
            "payload": [{
                "wid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "Bitcoin",
                "amount": "0.48650929",
                "details": {
                    "withdrawal_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp",
                    "tx_hash": "d4f28394693e9fb5fffcaf730c11f32d1922e5837f76ca82189d3bfe30ded433"
                }
            }, {
                "wid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "complete",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "2612.70",
                "details": {
                    "beneficiary_name": "BERTRAND RUSSELL",
                    "beneficiary_bank": "BANAMEX",
                    "beneficiary_clabe": "002320700708015728",
                    "numeric_reference": "99548",
                    "concepto": "Por los 🌮 del viernes",
                    "clave_rastreo": "BNET01001604080002076841",
                    "cep": {
                        "return": {
                            "cda": {
                                "cadenaOriginal": "||1|13062016|13062016|172053|40002|STP|Bitso - BERTRAND RUSSELL|40|646180115400000002|BIT140123U70|BANAMEX|BERTRAND RUSSELL|40|002320700708015728|ND|-|0.00|2612.70|00001000000401205824||",
                                "conceptoPago": "-",
                                "cuentaBeneficiario": "002320700708015728",
                                "cuentaOrdenante": "646180115400000002",
                                "fechaCaptura": "20160613",
                                "fechaOperacion": "20160613",
                                "hora": "17:08:42",
                                "iva": "0.00",
                                "monto": "2612.70",
                                "nombreBeneficiario": "BERTRAND RUSSELL",
                                "nombreInstBeneficiaria": "BANAMEX",
                                "nombreInstOrdenante": "STP",
                                "nombreOrdenante": "Bitso - Russell",
                                "referenciaNumerica": "99548",
                                "rfcCurpBeneficiario": "ND",
                                "rfcCurpOrdenante": "BIT140123U70",
                                "selloDigital": "cd7yUrnmUQ7CG6M+LX7WOZeizOpkTyMlEAunJaP2j5MAaNPZxy+vAJtgiVL73i1LNSrwK10eBb66Rh4\/RxU6AT2S03chQ\/BS1beknH5xPpGQg+wEXeANtnF2lp71lAD6QZ2O0NE4MIDvLhGGjTGklSP+2fS6joTAaV+tLbtrIp8JiR0MOX1rGPC5h+0ZHNvXQkcHJz3s68+iUAvDnQBiSu768b2C4zpHzteGEnJhU8sAdk83spiWogKALAVAuN4xfSXni7GTk9HObTTRdY+zehfWVPdE\/7uQSmMTzOKfPbQU02Jn\/5DdE3gYk6JZ5m70JsUSFBTF\/EVX8hhg0pu2iA==",
                                "serieCertificado": "",
                                "tipoOperacion": "C",
                                "tipoPago": "1"
                            },
                            "estadoConsulta": "1",
                            "url": "http:\/\/www.banxico.org.mx\/cep?i=90646&s=20150825&d=viAKjS0GVYB8qihmG9I%2B9O1VUvrR2td%2Fuo3GyVDn8vBp371tVx5ltRnk4QsWP6KP%2BQvlWjT%2BzfwWWTA3TMk4tg%3D%3D"
                        }
                    }
                }
            }]
        }"#)
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .get_withdrawals(
            None,
            Some(vec![
                "c5b8d7f0768ee91d3b33bee648318688",
                "p4u8d7f0768ee91d3b33bee6483132i8",
            ]),
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get withdrawals with optional params
/// This tests the case where a WID is provided
#[tokio::test]
async fn test_withdrawals_wid() {
    let _mock = mock("GET", "/v3/withdrawals/c5b8d7f0768ee91d3b33bee648318688/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "wid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "Bitcoin",
                "amount": "0.48650929",
                "details": {
                    "withdrawal_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp",
                    "tx_hash": "d4f28394693e9fb5fffcaf730c11f32d1922e5837f76ca82189d3bfe30ded433"
                }
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .get_withdrawals(
            Some("c5b8d7f0768ee91d3b33bee648318688"),
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get fundings
#[tokio::test]
async fn test_fundings_fid() {
    let _mock = mock("GET", "/v3/fundings/c5b8d7f0768ee91d3b33bee648318688/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "fid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "btc",
                "amount": "0.48650929",
                "details": {
                    "funding_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp",
                    "tx_hash": "d4f28394693e9fb5fffcaf730c11f32d1922e5837f76ca82189d3bfe30ded433"
                }
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .get_fundings(
            Some("c5b8d7f0768ee91d3b33bee648318688"),
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get fundings with optional params
#[tokio::test]
async fn test_fundings() {
    let _mock = mock("GET", "/v3/fundings/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "fid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "btc",
                "amount": "0.48650929",
                "details": {
                    "funding_address": "18MsnATiNiKLqUHDTRKjurwMg7inCrdNEp",
                    "tx_hash": "d4f28394693e9fb5fffcaf730c11f32d1922e5837f76ca82189d3bfe30ded433"
                }
            }, {
                "fid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "complete",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "300.15",
                "details": {
                    "sender_name": "BERTRAND RUSSELL",
                    "sender_bank": "BBVA Bancomer",
                    "sender_clabe": "012610001967722183",
                    "receive_clabe": "646180115400467548",
                    "numeric_reference": "80416",
                    "concepto": "Para el 🐖",
                    "clave_rastreo": "BNET01001604080002076841",
                    "beneficiary_name": "ALFRED NORTH WHITEHEAD"
                }
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_fundings(None, None, None, None, None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get user_trades
#[tokio::test]
async fn test_user_trades() {
    let _mock = mock("GET", "/v3/user_trades/")
        .match_query(Matcher::UrlEncoded("book".into(), "btc_mxn".into()))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "major": "-0.25232073",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "minor": "1013.540958479115",
                "fees_amount": "-10.237787459385",
                "fees_currency": "mxn",
                "price": "4057.45",
                "tid": 51756,
                "oid": "g81d3y1ywri0yg8m",
                "side": "sell",
                "maker_side": "sell"
            }, {
                "book": "eth_mxn",
                "major": "4.86859395",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "minor": "-626.77",
                "fees_amount": "-0.04917771",
                "fees_currency": "btc",
                "price": "127.45",
                "tid": 51757,
                "oid": "19vaqiv72drbphig",
                "side": "buy",
                "maker_side": "sell"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_user_trades("btc_mxn", None, None, None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get order_trades
#[tokio::test]
async fn test_order_trades() {
    let _mock = mock("GET", "/v3/order_trades/Jvqrschkgdkc1go3/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                    "book": "btc_mxn",
                    "major": "-0.25232073",
                    "created_at": "2016-04-08T17:52:31.000+00:00",
                    "minor": "1013.540958479115",
                    "fees_amount": "-10.237787459385",
                    "fees_currency": "mxn",
                    "price": "4057.45",
                    "tid": 51756,
                    "oid": "Jvqrschkgdkc1go3",
                    "origin_id": "origin_id1",
                    "side": "sell",
                    "maker_side": "sell"
                },
                {
                    "book": "btc_mxn",
                    "major": "-0.25",
                    "created_at": "2016-04-08T17:52:31.000+00:00",
                    "minor": "513.540958479115",
                    "fees_amount": "-10.237787459385",
                    "fees_currency": "mxn",
                    "price": "4057.45",
                    "tid": 51755,
                    "oid": "Jvqrschkgdkc1go3",
                    "origin_id": "origin_id1",
                    "side": "sell",
                    "maker_side": "sell"
                }
            ]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_order_trades(Some("Jvqrschkgdkc1go3"), None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get order_trades with origin_id
#[tokio::test]
async fn test_order_trades_origin_id() {
    let _mock = mock("GET", "/v3/order_trades")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "origin_id".into(),
            "origin_id1".into(),
        )]))
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                    "book": "btc_mxn",
                    "major": "-0.25232073",
                    "created_at": "2016-04-08T17:52:31.000+00:00",
                    "minor": "1013.540958479115",
                    "fees_amount": "-10.237787459385",
                    "fees_currency": "mxn",
                    "price": "4057.45",
                    "tid": 51756,
                    "oid": "Jvqrschkgdkc1go3",
                    "origin_id": "origin_id1",
                    "side": "sell",
                    "maker_side": "sell"
                }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_order_trades(None, Some("origin_id1")).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get open_orders
#[tokio::test]
async fn test_open_orders() {
    let _mock = mock("GET", "/v3/open_orders")
        .with_status(200)
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "book".into(),
            "btc_mxn".into(),
        )]))
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "original_amount": "0.01000000",
                "unfilled_amount": "0.00500000",
                "original_value": "56.0",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:51.000+00:00",
                "price": "5600.00",
                "oid": "543cr2v32a1h68443",
                "origin_id": "origin_id1",
                "side": "buy",
                "status": "partial-fill",
                "type": "limit"
            }, {
                "book": "btc_mxn",
                "original_amount": "0.12680000",
                "unfilled_amount": "0.12680000",
                "original_value": "507.2",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:41.000+00:00",
                "price": "4000.00",
                "oid": "qlbga6b600n3xta7",
                "side": "sell",
                "status": "open",
                "type": "limit"
            }, {
                "book": "btc_mxn",
                "original_amount": "1.12560000",
                "unfilled_amount": "1.12560000",
                "original_value": "6892.66788",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:41.000+00:00",
                "price": "6123.55",
                "oid": "d71e3xy2lowndkfm",
                "side": "sell",
                "status": "open",
                "type": "limit"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_open_orders(Some("btc_mxn"), None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get open_orders with optional params
#[tokio::test]
async fn test_open_orders_optional_params() {
    let _mock = mock("GET", "/v3/open_orders")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("book".into(), "btc_mxn".into()),
            Matcher::UrlEncoded("marker".into(), "51755".into()),
            Matcher::UrlEncoded("sort".into(), "asc".into()),
            Matcher::UrlEncoded("limit".into(), "1".into()),
        ]))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "original_amount": "0.01000000",
                "unfilled_amount": "0.00500000",
                "original_value": "56.0",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:51.000+00:00",
                "price": "5600.00",
                "oid": "543cr2v32a1h68443",
                "origin_id": "origin_id1",
                "side": "buy",
                "status": "partial-fill",
                "type": "limit"
            }, {
                "book": "btc_mxn",
                "original_amount": "0.12680000",
                "unfilled_amount": "0.12680000",
                "original_value": "507.2",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:41.000+00:00",
                "price": "4000.00",
                "oid": "qlbga6b600n3xta7",
                "side": "sell",
                "status": "open",
                "type": "limit"
            }, {
                "book": "btc_mxn",
                "original_amount": "1.12560000",
                "unfilled_amount": "1.12560000",
                "original_value": "6892.66788",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:41.000+00:00",
                "price": "6123.55",
                "oid": "d71e3xy2lowndkfm",
                "side": "sell",
                "status": "open",
                "type": "limit"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let optional_params = OptionalParams {
        marker: Some(&51755),
        sort: Some("asc"),
        limit: Some(&1),
    };
    let result = bitso
        .get_open_orders(Some("btc_mxn"), Some(optional_params))
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

/// Test successful request to get lookup_orders with optional params
#[tokio::test]
async fn test_lookup_orders_with_optional_params() {
    let _mock = mock("GET", "/v3/orders/")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
            "oids".into(),
            "543cr2v32a1h6844,qlbga6b600n3xta7a".into(),
        )]))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "original_amount": "0.01000000",
                "unfilled_amount": "0.00500000",
                "original_value": "56.0",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:51.000+00:00",
                "price": "5600.00",
                "oid": "543cr2v32a1h6844",
                "side": "buy",
                "status": "partial-fill",
                "type": "limit"
            }, {
                "book": "btc_mxn",
                "original_amount": "0.12680000",
                "unfilled_amount": "0.12680000",
                "original_value": "507.2",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:41.000+00:00",
                "price": "4000.00",
                "oid": "qlbga6b600n3xta7a",
                "side": "sell",
                "status": "open",
                "type": "limit"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .get_lookup_orders(
            None,
            Some(vec!["543cr2v32a1h6844", "qlbga6b600n3xta7a"]),
            None,
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get lookup_orders
#[tokio::test]
async fn test_lookup_orders() {
    let _mock = mock("GET", "/v3/orders/543cr2v32a1h6844/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "original_amount": "0.01000000",
                "unfilled_amount": "0.00500000",
                "original_value": "56.0",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "updated_at": "2016-04-08T17:52:51.000+00:00",
                "price": "5600.00",
                "oid": "543cr2v32a1h6844",
                "side": "buy",
                "status": "partial-fill",
                "type": "limit"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .get_lookup_orders(Some("543cr2v32a1h6844"), None, None)
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to cancel_order
#[tokio::test]
async fn test_cancel_order() {
    let _mock = mock("DELETE", "/v3/orders/cME2F7uZKJcMKXqU/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload":[
                "cME2F7uZKJcMKXqU",
                "FwllxXRKvcgJmyFy",
                "zhDI9iBRglW9s9Vu"
            ]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .cancel_order(false, Some("cME2F7uZKJcMKXqU"), None, None)
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

// Test successful request to cancel_order
#[tokio::test]
async fn test_cancel_order_all() {
    let _mock = mock("DELETE", "/v3/orders/all")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload":[
                "cME2F7uZKJcMKXqU",
                "FwllxXRKvcgJmyFy",
                "zhDI9iBRglW9s9Vu"
            ]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.cancel_order(true, None, None, None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to place_order
#[tokio::test]
async fn test_place_order() {
    let _mock = mock("POST", "/v3/orders/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "oid": "qlbga6b600n3xta7"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.place_order("btc_mxn", "sell", "market", None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to place_with optional params
#[tokio::test]
async fn test_place_order_with_optional_params() {
    let _mock = mock("POST", "/v3/orders/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "oid": "qlbga6b600n3xta7"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let optional_order_params = OptionalOrderParams {
        major: Some("0.0001"),
        minor: None,
        price: None,
        stop: None,
        time_in_force: None,
        origin_id: None,
    };
    let result = bitso
        .place_order("btc_mxn", "sell", "market", Some(optional_order_params))
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to funding_destination
#[tokio::test]
async fn test_funding_destination() {
    let _mock = mock("GET", "/v3/funding_destination/")
        .match_query(Matcher::UrlEncoded("fund_currency".into(), "btc".into()))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "account_identifier_name": "SPEI CLABE",
                "account_identifier": "646180115400346012"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_funding_destination("btc").await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to make a crypto withdrawal
#[tokio::test]
async fn test_crypto_withdrawal() {
    let _mock = mock("POST", "/v3/crypto_withdrawal/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "wid": "c5b8d7f0768ee91d3b33bee648318688",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "btc",
                "method": "btc",
                "amount": "0.48650929",
                "details": {
                    "withdrawal_address": "3EW92Ajg6sMT4hxK8ngEc7Ehrqkr9RoDt7",
                    "tx_hash": null
                }
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .crypto_withdrawal(
            "btc",
            "0.001",
            "3EW92Ajg6sMT4hxK8ngEc7Ehrqkr9RoDt7",
            Some("0.001"),
            Some("some_tag"),
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to make a SPEI withdrawal
#[tokio::test]
async fn test_spei_withdrawal() {
    let _mock = mock("POST", "/v3/spei_withdrawal/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "wid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "300.15",
                "details": {
                    "sender_name": "JUAN ESCUTIA",
                    "receive_clabe": "012610001967722183",
                    "sender_clabe": "646180115400467548",
                    "numeric_reference": "80416",
                    "concepto": "Tacos del viernes",
                    "clave_rastreo": null,
                    "beneficiary_name": "FRANCISCO MARQUEZ"
                }
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .spei_withdrawal(
            "200",
            "alguien alguien",
            "guien guillen",
            "012610001967722183",
            Some("notes_ref"),
            Some("numeric_red"),
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get bank codes
#[tokio::test]
async fn test_bank_codes() {
    let _mock = mock("GET", "/v3/mx_bank_codes/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "code": "01",
                "name": "Banregio"
            }, {
                "code": "02",
                "name": "BBVA"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso.get_bank_codes().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to make a debit-card withdrawal
#[tokio::test]
async fn test_debit_card_withdrawal() {
    let _mock = mock("POST", "/v3/debit_card_withdrawal/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "wid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "300.15",
                "details": {
                    "sender_name": "JUAN ESCUTIA",
                    "receive_clabe": "012610001967722183",
                    "sender_clabe": "646180115400467548",
                    "numeric_reference": "80416",
                    "concepto": "Tacos del viernes",
                    "clave_rastreo": null,
                    "beneficiary_name": "FRANCISCO MARQUEZ"
                }
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .debit_card_withdrawal(
            "200",
            "alguien alguien",
            "guien guillen",
            "0123456789012345",
            "40138",
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to make a phone-number withdrawal
#[tokio::test]
async fn test_phone_number_withdrawal() {
    let _mock = mock("POST", "/v3/phone_withdrawal/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "wid": "p4u8d7f0768ee91d3b33bee6483132i8",
                "status": "pending",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "currency": "mxn",
                "method": "sp",
                "amount": "300.15",
                "details": {
                    "sender_name": "JUAN ESCUTIA",
                    "receive_clabe": "012610001967722183",
                    "sender_clabe": "646180115400467548",
                    "numeric_reference": "80416",
                    "concepto": "Tacos del viernes",
                    "clave_rastreo": null,
                    "beneficiary_name": "FRANCISCO MARQUEZ"
                }
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let result = bitso
        .phone_number_withdrawal(
            "200",
            "alguien alguien",
            "guien guillen",
            "0123456789",
            "40138",
        )
        .await;
    assert!(result.is_ok());
    println!("{:?}", result);
}
