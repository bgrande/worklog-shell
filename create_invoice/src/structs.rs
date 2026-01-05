use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Customer {
    id: String,
    customer_number: String,
    status: String,
    name: String,
    company_name: String,
    contacts: Vec<Contact>,
    addresses: Vec<Address>,
    payments: Option<Vec<Payment>>,
    description: String,
    data_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Contact {
    id: String,
    person_name: String,
    is_main: bool,
    email: String,
    tel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Address {
    id: String,
    for_invoice: bool,
    street: String,
    addition: String,
    zip: String,
    city: String,
    state: String,
    country: String,
    valid_from: Option<NaiveDateTime>,
    valid_to: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Position {
    pub(crate) customer_id: String,
    pub(crate) date: String,
    pub(crate) positions: Vec<PositionDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PositionDetail {
    pub(crate) summary: String,
    pub(crate) description: String,
    pub(crate) vat_id: String,
    pub(crate) unit_id: String,
    pub(crate) amount: f64,
    pub(crate) net_price: String,
    pub(crate) currency: String,
    #[serde(rename = "date_execution")]
    pub(crate) date_execution: Option<NaiveDateTime>,
    #[serde(rename = "date_delivery")]
    pub(crate) date_delivery: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Invoice {
    pub(crate) id: String,
    pub(crate) customer_id: String,
    #[serde(rename = "date_created")]
    pub(crate) date_created: NaiveDateTime,
    #[serde(rename = "date_execution")]
    pub(crate) date_execution: Option<NaiveDateTime>,
    pub(crate) date_delivery: Option<NaiveDateTime>,
    pub(crate) customer_number: String,
    pub(crate) positions: Vec<Position>,
    pub(crate) total_net_price: String,
    pub(crate) total_gross_price: String,
    pub(crate) vat_id: String,
    pub(crate) vat_value: f64,
    pub(crate) currency: String, // Like EUR (ISO)
    pub(crate) payment_text: Option<String>,
    pub(crate) payment_qr: Option<String> // todo: either base64 string or binary type or path to file
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) taxes: HashMap<String, Tax>,
    pub(crate) emails: HashMap<String, EMail>,
    pub(crate) units: Vec<Unit>,
    pub(crate) automation: Option<Automation>,
    pub(crate) company: Company,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Automation {
    pub(crate) enabled: bool,
    pub(crate) intervals: String // todo probably another type like HashMap
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EMail {
    pub(crate) name: String,
    pub(crate) from: String,
    pub(crate) reply_to: String,
    pub(crate) signature: String,
    pub(crate) credentials: Credentials,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Credentials {
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) user: String,
    pub(crate) password: String, // might use encryption here
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Tax {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) country: String,
    pub(crate) active: bool,
    pub(crate) is_default: bool,
    // other fields omitted as they're not needed for this calculation
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Unit {
    pub(crate) id: String,
    pub(crate) short_name: String,
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Payment {
    pub(crate) id: String,
    pub(crate) summary: String,
    pub(crate) holder_name: String,
    pub(crate) iban: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) bic: Option<String>,
    pub(crate) bank_name: String,
    pub(crate) is_default: bool,
    pub(crate) payment_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    pub(crate) name: String,
    pub(crate) vat_id: String,
    pub(crate) tax_id: String,
    pub(crate) payment: HashMap<String, Payment>,
    pub(crate) addresses: HashMap<String, Address>,
}
