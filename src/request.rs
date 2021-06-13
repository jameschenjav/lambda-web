//!
//! Lambda event deserialize
//!

//use std::{borrow::Cow, collections::HashMap, fmt, io::Read, mem};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;

/// API Gateway payload format version 2.0
/// https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiGatewayV2<'a> {
    // version: Cow<'a, str>,
    // route_key: Cow<'a, str>,
    pub(crate) raw_path: Cow<'a, str>,
    pub(crate) raw_query_string: Cow<'a, str>,
    pub(crate) cookies: Option<Vec<Cow<'a, str>>>,
    pub(crate) headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    // #[serde(default)]
    // query_string_parameters: StrMap,
    // #[serde(default)]
    // path_parameters: StrMap,
    // #[serde(default)]
    // stage_variables: StrMap,
    pub(crate) body: Option<Cow<'a, str>>,
    #[serde(default)]
    pub(crate) is_base64_encoded: bool,
    pub(crate) request_context: ApiGatewayV2RequestContext<'a>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayV2RequestContext<'a> {
    /// The API owner's AWS account ID.
    pub account_id: String,
    /// The identifier API Gateway assigns to your API.
    pub api_id: String,
    /// The stringified value of the specified key-value pair of the context map returned from an API Gateway Lambda authorizer function.
    #[serde(default)]
    pub authorizer: HashMap<String, serde_json::Value>,
    /// The full domain name used to invoke the API. This should be the same as the incoming Host header.
    pub domain_name: String,
    /// The first label of the $context.domainName. This is often used as a caller/customer identifier.
    pub domain_prefix: String,
    /// The HTTP method used.
    pub http: Http<'a>,
    /// The ID that API Gateway assigns to the API request.
    pub request_id: String,
    /// Undocumented, could be resourcePath
    pub route_key: String,
    /// The deployment stage of the API request (for example, Beta or Prod).
    pub stage: String,
    /// Undocumented, could be requestTime
    pub time: String,
    /// Undocumented, could be requestTimeEpoch
    pub time_epoch: usize,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Http<'a> {
    /// The HTTP method used. Valid values include: DELETE, GET, HEAD, OPTIONS, PATCH, POST, and PUT.
    pub method: Cow<'a, str>,
    // The request path. For example, for a non-proxy request URL of
    // `https://{rest-api-id.execute-api.{region}.amazonaws.com/{stage}/root/child`,
    // the $context.path value is `/{stage}/root/child`.
    // pub path: Cow<'a, str>,
    // The request protocol, for example, HTTP/1.1.
    // pub protocol: Cow<'a, str>,
    /// The source IP address of the TCP connection making the request to API Gateway.
    pub source_ip: Cow<'a, str>,
    // The User-Agent header of the API caller.
    // pub user_agent: Cow<'a, str>,
}