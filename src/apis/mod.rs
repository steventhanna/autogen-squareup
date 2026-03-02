use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod configuration;

#[cfg(feature = "apple-pay")]
pub mod apple_pay_api;
#[cfg(feature = "bank-accounts")]
pub mod bank_accounts_api;
#[cfg(feature = "booking-custom-attributes")]
pub mod booking_custom_attributes_api;
#[cfg(feature = "bookings")]
pub mod bookings_api;
#[cfg(feature = "cards")]
pub mod cards_api;
#[cfg(feature = "cash-drawers")]
pub mod cash_drawers_api;
#[cfg(feature = "catalog")]
pub mod catalog_api;
#[cfg(feature = "channels")]
pub mod channels_api;
#[cfg(feature = "checkout")]
pub mod checkout_api;
#[cfg(feature = "customer-custom-attributes")]
pub mod customer_custom_attributes_api;
#[cfg(feature = "customer-groups")]
pub mod customer_groups_api;
#[cfg(feature = "customer-segments")]
pub mod customer_segments_api;
#[cfg(feature = "customers")]
pub mod customers_api;
#[cfg(feature = "devices")]
pub mod devices_api;
#[cfg(feature = "disputes")]
pub mod disputes_api;
#[cfg(feature = "employees")]
pub mod employees_api;
#[cfg(feature = "events")]
pub mod events_api;
#[cfg(feature = "gift-card-activities")]
pub mod gift_card_activities_api;
#[cfg(feature = "gift-cards")]
pub mod gift_cards_api;
#[cfg(feature = "inventory")]
pub mod inventory_api;
#[cfg(feature = "invoices")]
pub mod invoices_api;
#[cfg(feature = "labor")]
pub mod labor_api;
#[cfg(feature = "location-custom-attributes")]
pub mod location_custom_attributes_api;
#[cfg(feature = "locations")]
pub mod locations_api;
#[cfg(feature = "loyalty")]
pub mod loyalty_api;
#[cfg(feature = "merchant-custom-attributes")]
pub mod merchant_custom_attributes_api;
#[cfg(feature = "merchants")]
pub mod merchants_api;
#[cfg(feature = "mobile-authorization")]
pub mod mobile_authorization_api;
#[cfg(feature = "oauth")]
pub mod o_auth_api;
#[cfg(feature = "order-custom-attributes")]
pub mod order_custom_attributes_api;
#[cfg(feature = "orders")]
pub mod orders_api;
#[cfg(feature = "payments")]
pub mod payments_api;
#[cfg(feature = "payouts")]
pub mod payouts_api;
#[cfg(feature = "refunds")]
pub mod refunds_api;
#[cfg(feature = "sites")]
pub mod sites_api;
#[cfg(feature = "snippets")]
pub mod snippets_api;
#[cfg(feature = "subscriptions")]
pub mod subscriptions_api;
#[cfg(feature = "team")]
pub mod team_api;
#[cfg(feature = "terminal")]
pub mod terminal_api;
#[cfg(feature = "transactions")]
pub mod transactions_api;
#[cfg(feature = "transfer-order")]
pub mod transfer_order_api;
#[cfg(feature = "v1-transactions")]
pub mod v1_transactions_api;
#[cfg(feature = "vendors")]
pub mod vendors_api;
#[cfg(feature = "webhook-subscriptions")]
pub mod webhook_subscriptions_api;
