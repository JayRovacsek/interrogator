// Example code that deserializes and serializes the model.
extern crate serde;
extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct OktaLog {
    #[serde(rename = "actor")]
    pub actor: Option<Actor>,

    #[serde(rename = "client")]
    pub client: Option<Client>,

    #[serde(rename = "authenticationContext")]
    pub authentication_context: Option<AuthenticationContext>,

    #[serde(rename = "displayMessage")]
    pub display_message: Option<String>,

    #[serde(rename = "eventType")]
    pub event_type: Option<String>,

    #[serde(rename = "outcome")]
    pub outcome: Option<Outcome>,

    #[serde(rename = "published")]
    pub published: Option<String>,

    #[serde(rename = "securityContext")]
    pub security_context: Option<SecurityContext>,

    #[serde(rename = "severity")]
    pub severity: Option<String>,

    #[serde(rename = "debugContext")]
    pub debug_context: Option<DebugContext>,

    #[serde(rename = "legacyEventType")]
    pub legacy_event_type: Option<String>,

    #[serde(rename = "transaction")]
    pub transaction: Option<Transaction>,

    #[serde(rename = "uuid")]
    pub uuid: Option<String>,

    #[serde(rename = "version")]
    pub version: Option<String>,

    #[serde(rename = "request")]
    pub request: Option<Request>,

    #[serde(rename = "target")]
    pub target: Option<Vec<Actor>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    pub actor_type: Option<String>,

    #[serde(rename = "alternateId")]
    pub alternate_id: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "detailEntry")]
    pub detail_entry: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationContext {
    #[serde(rename = "authenticationProvider")]
    pub authentication_provider: Option<String>,

    #[serde(rename = "credentialProvider")]
    pub credential_provider: Option<serde_json::Value>,

    #[serde(rename = "credentialType")]
    pub credential_type: Option<String>,

    #[serde(rename = "issuer")]
    pub issuer: Option<serde_json::Value>,

    #[serde(rename = "interface")]
    pub interface: Option<String>,

    #[serde(rename = "authenticationStep")]
    pub authentication_step: Option<i64>,

    #[serde(rename = "externalSessionId")]
    pub external_session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    #[serde(rename = "userAgent")]
    pub user_agent: Option<UserAgent>,

    #[serde(rename = "zone")]
    pub zone: Option<String>,

    #[serde(rename = "device")]
    pub device: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<serde_json::Value>,

    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,

    #[serde(rename = "geographicalContext")]
    pub geographical_context: Option<GeographicalContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeographicalContext {
    #[serde(rename = "city")]
    pub city: Option<String>,

    #[serde(rename = "state")]
    pub state: Option<String>,

    #[serde(rename = "country")]
    pub country: Option<String>,

    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,

    #[serde(rename = "geolocation")]
    pub geolocation: Option<Geolocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geolocation {
    #[serde(rename = "lat")]
    pub lat: Option<f64>,

    #[serde(rename = "lon")]
    pub lon: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAgent {
    #[serde(rename = "rawUserAgent")]
    pub raw_user_agent: Option<String>,

    #[serde(rename = "os")]
    pub os: Option<String>,

    #[serde(rename = "browser")]
    pub browser: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugContext {
    #[serde(rename = "debugData")]
    pub debug_data: Option<DebugData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugData {
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(rename = "delauthtimetotal")]
    pub delauthtimetotal: Option<String>,

    #[serde(rename = "requestUri")]
    pub request_uri: Option<String>,

    #[serde(rename = "delauthtimespentatdomaincontroller")]
    pub delauthtimespentatdomaincontroller: Option<String>,

    #[serde(rename = "delauthtimespentatagent")]
    pub delauthtimespentatagent: Option<String>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    #[serde(rename = "result")]
    pub result: Option<String>,

    #[serde(rename = "reason")]
    pub reason: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "ipChain")]
    pub ip_chain: Option<Vec<IpChain>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpChain {
    #[serde(rename = "ip")]
    pub ip: Option<String>,

    #[serde(rename = "geographicalContext")]
    pub geographical_context: Option<GeographicalContext>,

    #[serde(rename = "version")]
    pub version: Option<String>,

    #[serde(rename = "source")]
    pub source: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityContext {
    #[serde(rename = "asNumber")]
    pub as_number: Option<serde_json::Value>,

    #[serde(rename = "asOrg")]
    pub as_org: Option<serde_json::Value>,

    #[serde(rename = "isp")]
    pub isp: Option<serde_json::Value>,

    #[serde(rename = "domain")]
    pub domain: Option<serde_json::Value>,

    #[serde(rename = "isProxy")]
    pub is_proxy: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "detail")]
    pub detail: Option<Detail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
}
