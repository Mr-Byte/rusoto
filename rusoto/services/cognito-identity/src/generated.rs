// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>A provider representing an Amazon Cognito Identity User Pool and its client ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitoIdentityProvider {
    /// <p>The client ID for the Amazon Cognito Identity User Pool.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The provider name for an Amazon Cognito Identity User Pool. For example, <code>cognito-idp.us-east-1.amazonaws.com/us-east-1_123456789</code>.</p>
    #[serde(rename = "ProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// <p>TRUE if server-side token validation is enabled for the identity provider’s token.</p>
    #[serde(rename = "ServerSideTokenCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_token_check: Option<bool>,
}

/// <p>Input to the CreateIdentityPool action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIdentityPoolInput {
    /// <p>TRUE if the identity pool supports unauthenticated logins.</p>
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    /// <p>An array of Amazon Cognito Identity user pools and their client IDs.</p>
    #[serde(rename = "CognitoIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
    /// <p>The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (<code>.</code>), underscore (<code>_</code>), and dash (<code>-</code>).</p> <p>Once you have set a developer provider name, you cannot change it. Please take care in setting this parameter.</p>
    #[serde(rename = "DeveloperProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<String>,
    /// <p>A string that you provide.</p>
    #[serde(rename = "IdentityPoolName")]
    pub identity_pool_name: String,
    /// <p>A list of OpendID Connect provider ARNs.</p>
    #[serde(rename = "OpenIdConnectProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_ar_ns: Option<Vec<String>>,
    /// <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity pool.</p>
    #[serde(rename = "SamlProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_ar_ns: Option<Vec<String>>,
    /// <p>Optional key:value pairs mapping provider names to provider app IDs.</p>
    #[serde(rename = "SupportedLoginProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Credentials for the provided identity ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Credentials {
    /// <p>The Access Key portion of the credentials.</p>
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>The date at which these credentials will expire.</p>
    #[serde(rename = "Expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    /// <p>The Secret Access Key portion of the credentials</p>
    #[serde(rename = "SecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// <p>The Session Token portion of the credentials</p>
    #[serde(rename = "SessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

/// <p>Input to the <code>DeleteIdentities</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIdentitiesInput {
    /// <p>A list of 1-60 identities that you want to delete.</p>
    #[serde(rename = "IdentityIdsToDelete")]
    pub identity_ids_to_delete: Vec<String>,
}

/// <p>Returned in response to a successful <code>DeleteIdentities</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteIdentitiesResponse {
    /// <p>An array of UnprocessedIdentityId objects, each of which contains an ErrorCode and IdentityId.</p>
    #[serde(rename = "UnprocessedIdentityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_identity_ids: Option<Vec<UnprocessedIdentityId>>,
}

/// <p>Input to the DeleteIdentityPool action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIdentityPoolInput {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Input to the <code>DescribeIdentity</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityInput {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
}

/// <p>Input to the DescribeIdentityPool action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityPoolInput {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Input to the <code>GetCredentialsForIdentity</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCredentialsForIdentityInput {
    /// <p>The Amazon Resource Name (ARN) of the role to be assumed when multiple roles were received in the token from the identity provider. For example, a SAML-based identity provider. This parameter is optional for identity providers that do not support role customization.</p>
    #[serde(rename = "CustomRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens.</p>
    #[serde(rename = "Logins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Returned in response to a successful <code>GetCredentialsForIdentity</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCredentialsForIdentityResponse {
    /// <p>Credentials for the provided identity ID.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

/// <p>Input to the GetId action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIdInput {
    /// <p>A standard AWS account ID (9+ digits).</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p><p>A set of optional name-value pairs that map provider names to provider tokens. The available provider names for <code>Logins</code> are as follows:</p> <ul> <li> <p>Facebook: <code>graph.facebook.com</code> </p> </li> <li> <p>Amazon Cognito Identity Provider: <code>cognito-idp.us-east-1.amazonaws.com/us-east-1_123456789</code> </p> </li> <li> <p>Google: <code>accounts.google.com</code> </p> </li> <li> <p>Amazon: <code>www.amazon.com</code> </p> </li> <li> <p>Twitter: <code>api.twitter.com</code> </p> </li> <li> <p>Digits: <code>www.digits.com</code> </p> </li> </ul></p>
    #[serde(rename = "Logins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Returned in response to a GetId request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIdResponse {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

/// <p>Input to the <code>GetIdentityPoolRoles</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIdentityPoolRolesInput {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Returned in response to a successful <code>GetIdentityPoolRoles</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIdentityPoolRolesResponse {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>How users for a specific identity provider are to mapped to roles. This is a String-to-<a>RoleMapping</a> object map. The string identifies the identity provider, for example, "graph.facebook.com" or "cognito-idp-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id".</p>
    #[serde(rename = "RoleMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<::std::collections::HashMap<String, RoleMapping>>,
    /// <p>The map of roles associated with this pool. Currently only authenticated and unauthenticated roles are supported.</p>
    #[serde(rename = "Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Input to the <code>GetOpenIdTokenForDeveloperIdentity</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOpenIdTokenForDeveloperIdentityInput {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens. Each name-value pair represents a user from a public provider or developer provider. If the user is from a developer provider, the name-value pair will follow the syntax <code>"developer_provider_name": "developer_user_identifier"</code>. The developer provider is the "domain" by which Cognito will refer to your users; you provided this domain while creating/updating the identity pool. The developer user identifier is an identifier from your backend that uniquely identifies a user. When you create an identity pool, you can specify the supported logins.</p>
    #[serde(rename = "Logins")]
    pub logins: ::std::collections::HashMap<String, String>,
    /// <p>The expiration time of the token, in seconds. You can specify a custom expiration time for the token so that you can cache it. If you don't provide an expiration time, the token is valid for 15 minutes. You can exchange the token with Amazon STS for temporary AWS credentials, which are valid for a maximum of one hour. The maximum token duration you can set is 24 hours. You should take care in setting the expiration time for a token, as there are significant security implications: an attacker could use a leaked token to access your AWS resources for the token's duration.</p>
    #[serde(rename = "TokenDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_duration: Option<i64>,
}

/// <p>Returned in response to a successful <code>GetOpenIdTokenForDeveloperIdentity</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetOpenIdTokenForDeveloperIdentityResponse {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>An OpenID token.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// <p>Input to the GetOpenIdToken action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOpenIdTokenInput {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens. When using graph.facebook.com and www.amazon.com, supply the access_token returned from the provider's authflow. For accounts.google.com, an Amazon Cognito Identity Provider, or any other OpenId Connect provider, always include the <code>id_token</code>.</p>
    #[serde(rename = "Logins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Returned in response to a successful GetOpenIdToken request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetOpenIdTokenResponse {
    /// <p>A unique identifier in the format REGION:GUID. Note that the IdentityId returned may not match the one passed on input.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>An OpenID token, valid for 15 minutes.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// <p>A description of the identity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IdentityDescription {
    /// <p>Date on which the identity was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>Date on which the identity was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens.</p>
    #[serde(rename = "Logins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logins: Option<Vec<String>>,
}

/// <p>An object representing an Amazon Cognito identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityPool {
    /// <p>TRUE if the identity pool supports unauthenticated logins.</p>
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    /// <p>A list representing an Amazon Cognito Identity User Pool and its client ID.</p>
    #[serde(rename = "CognitoIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,
    /// <p>The "domain" by which Cognito will refer to your users.</p>
    #[serde(rename = "DeveloperProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<String>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>A string that you provide.</p>
    #[serde(rename = "IdentityPoolName")]
    pub identity_pool_name: String,
    /// <p>A list of OpendID Connect provider ARNs.</p>
    #[serde(rename = "OpenIdConnectProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_ar_ns: Option<Vec<String>>,
    /// <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity pool.</p>
    #[serde(rename = "SamlProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_ar_ns: Option<Vec<String>>,
    /// <p>Optional key:value pairs mapping provider names to provider app IDs.</p>
    #[serde(rename = "SupportedLoginProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A description of the identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IdentityPoolShortDescription {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>A string that you provide.</p>
    #[serde(rename = "IdentityPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_name: Option<String>,
}

/// <p>Input to the ListIdentities action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIdentitiesInput {
    /// <p>An optional boolean parameter that allows you to hide disabled identities. If omitted, the ListIdentities API will include disabled identities in the response.</p>
    #[serde(rename = "HideDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_disabled: Option<bool>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>The maximum number of identities to return.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The response to a ListIdentities request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIdentitiesResponse {
    /// <p>An object containing a set of identities and associated mappings.</p>
    #[serde(rename = "Identities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<IdentityDescription>>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Input to the ListIdentityPools action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIdentityPoolsInput {
    /// <p>The maximum number of identities to return.</p>
    #[serde(rename = "MaxResults")]
    pub max_results: i64,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The result of a successful ListIdentityPools action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIdentityPoolsResponse {
    /// <p>The identity pools returned by the ListIdentityPools action.</p>
    #[serde(rename = "IdentityPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pools: Option<Vec<IdentityPoolShortDescription>>,
    /// <p>A pagination token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Input to the <code>LookupDeveloperIdentityInput</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LookupDeveloperIdentityInput {
    /// <p>A unique ID used by your backend authentication process to identify a user. Typically, a developer identity provider would issue many developer user identifiers, in keeping with the number of users.</p>
    #[serde(rename = "DeveloperUserIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_user_identifier: Option<String>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>The maximum number of identities to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token. The first call you make will have <code>NextToken</code> set to null. After that the service will return <code>NextToken</code> values as needed. For example, let's say you make a request with <code>MaxResults</code> set to 10, and there are 20 matches in the database. The service will return a pagination token as a part of the response. This token can be used to call the API again and get results starting from the 11th match.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returned in response to a successful <code>LookupDeveloperIdentity</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LookupDeveloperIdentityResponse {
    /// <p>This is the list of developer user identifiers associated with an identity ID. Cognito supports the association of multiple developer user identifiers with an identity ID.</p>
    #[serde(rename = "DeveloperUserIdentifierList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_user_identifier_list: Option<Vec<String>>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>A pagination token. The first call you make will have <code>NextToken</code> set to null. After that the service will return <code>NextToken</code> values as needed. For example, let's say you make a request with <code>MaxResults</code> set to 10, and there are 20 matches in the database. The service will return a pagination token as a part of the response. This token can be used to call the API again and get results starting from the 11th match.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A rule that maps a claim name, a claim value, and a match type to a role ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingRule {
    /// <p>The claim name that must be present in the token, for example, "isAdmin" or "paid".</p>
    #[serde(rename = "Claim")]
    pub claim: String,
    /// <p>The match condition that specifies how closely the claim value in the IdP token must match <code>Value</code>.</p>
    #[serde(rename = "MatchType")]
    pub match_type: String,
    /// <p>The role ARN.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>A brief string that the claim must match, for example, "paid" or "yes".</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Input to the <code>MergeDeveloperIdentities</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MergeDeveloperIdentitiesInput {
    /// <p>User identifier for the destination user. The value should be a <code>DeveloperUserIdentifier</code>.</p>
    #[serde(rename = "DestinationUserIdentifier")]
    pub destination_user_identifier: String,
    /// <p>The "domain" by which Cognito will refer to your users. This is a (pseudo) domain name that you provide while creating an identity pool. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (.), underscore (_), and dash (-).</p>
    #[serde(rename = "DeveloperProviderName")]
    pub developer_provider_name: String,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>User identifier for the source user. The value should be a <code>DeveloperUserIdentifier</code>.</p>
    #[serde(rename = "SourceUserIdentifier")]
    pub source_user_identifier: String,
}

/// <p>Returned in response to a successful <code>MergeDeveloperIdentities</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MergeDeveloperIdentitiesResponse {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

/// <p>A role mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleMapping {
    /// <p>If you specify Token or Rules as the <code>Type</code>, <code>AmbiguousRoleResolution</code> is required.</p> <p>Specifies the action to be taken if either no rules match the claim value for the <code>Rules</code> type, or there is no <code>cognito:preferred_role</code> claim and there are multiple <code>cognito:roles</code> matches for the <code>Token</code> type.</p>
    #[serde(rename = "AmbiguousRoleResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambiguous_role_resolution: Option<String>,
    /// <p>The rules to be used for mapping users to roles.</p> <p>If you specify Rules as the role mapping type, <code>RulesConfiguration</code> is required.</p>
    #[serde(rename = "RulesConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_configuration: Option<RulesConfigurationType>,
    /// <p>The role mapping type. Token will use <code>cognito:roles</code> and <code>cognito:preferred_role</code> claims from the Cognito identity provider token to map groups to roles. Rules will attempt to match claims from the token to map to a role.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>A container for rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesConfigurationType {
    /// <p>An array of rules. You can specify up to 25 rules per identity provider.</p> <p>Rules are evaluated in order. The first one to match specifies the role.</p>
    #[serde(rename = "Rules")]
    pub rules: Vec<MappingRule>,
}

/// <p>Input to the <code>SetIdentityPoolRoles</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetIdentityPoolRolesInput {
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>How users for a specific identity provider are to mapped to roles. This is a string to <a>RoleMapping</a> object map. The string identifies the identity provider, for example, "graph.facebook.com" or "cognito-idp-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id".</p> <p>Up to 25 rules can be specified per identity provider.</p>
    #[serde(rename = "RoleMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<::std::collections::HashMap<String, RoleMapping>>,
    /// <p>The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.</p>
    #[serde(rename = "Roles")]
    pub roles: ::std::collections::HashMap<String, String>,
}

/// <p>Input to the <code>UnlinkDeveloperIdentity</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnlinkDeveloperIdentityInput {
    /// <p>The "domain" by which Cognito will refer to your users.</p>
    #[serde(rename = "DeveloperProviderName")]
    pub developer_provider_name: String,
    /// <p>A unique ID used by your backend authentication process to identify a user.</p>
    #[serde(rename = "DeveloperUserIdentifier")]
    pub developer_user_identifier: String,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>An identity pool ID in the format REGION:GUID.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Input to the UnlinkIdentity action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnlinkIdentityInput {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens.</p>
    #[serde(rename = "Logins")]
    pub logins: ::std::collections::HashMap<String, String>,
    /// <p>Provider names to unlink from this identity.</p>
    #[serde(rename = "LoginsToRemove")]
    pub logins_to_remove: Vec<String>,
}

/// <p>An array of UnprocessedIdentityId objects, each of which contains an ErrorCode and IdentityId.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedIdentityId {
    /// <p>The error code indicating the type of error that occurred.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A unique identifier in the format REGION:GUID.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

/// Errors returned by CreateIdentityPool
#[derive(Debug, PartialEq)]
pub enum CreateIdentityPoolError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when the total number of user pools has exceeded a preset limit.</p>
    LimitExceeded(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl CreateIdentityPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIdentityPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(CreateIdentityPoolError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateIdentityPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIdentityPoolError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(CreateIdentityPoolError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateIdentityPoolError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateIdentityPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateIdentityPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIdentityPoolError {
    fn description(&self) -> &str {
        match *self {
            CreateIdentityPoolError::InternalError(ref cause) => cause,
            CreateIdentityPoolError::InvalidParameter(ref cause) => cause,
            CreateIdentityPoolError::LimitExceeded(ref cause) => cause,
            CreateIdentityPoolError::NotAuthorized(ref cause) => cause,
            CreateIdentityPoolError::ResourceConflict(ref cause) => cause,
            CreateIdentityPoolError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIdentities
#[derive(Debug, PartialEq)]
pub enum DeleteIdentitiesError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl DeleteIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIdentitiesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteIdentitiesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteIdentitiesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIdentitiesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteIdentitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIdentitiesError {
    fn description(&self) -> &str {
        match *self {
            DeleteIdentitiesError::InternalError(ref cause) => cause,
            DeleteIdentitiesError::InvalidParameter(ref cause) => cause,
            DeleteIdentitiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIdentityPool
#[derive(Debug, PartialEq)]
pub enum DeleteIdentityPoolError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl DeleteIdentityPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIdentityPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteIdentityPoolError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteIdentityPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteIdentityPoolError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIdentityPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIdentityPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteIdentityPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIdentityPoolError {
    fn description(&self) -> &str {
        match *self {
            DeleteIdentityPoolError::InternalError(ref cause) => cause,
            DeleteIdentityPoolError::InvalidParameter(ref cause) => cause,
            DeleteIdentityPoolError::NotAuthorized(ref cause) => cause,
            DeleteIdentityPoolError::ResourceNotFound(ref cause) => cause,
            DeleteIdentityPoolError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentity
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl DescribeIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityError::InternalError(ref cause) => cause,
            DescribeIdentityError::InvalidParameter(ref cause) => cause,
            DescribeIdentityError::NotAuthorized(ref cause) => cause,
            DescribeIdentityError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentityPool
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityPoolError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl DescribeIdentityPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIdentityPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityPoolError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityPoolError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeIdentityPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityPoolError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityPoolError::InternalError(ref cause) => cause,
            DescribeIdentityPoolError::InvalidParameter(ref cause) => cause,
            DescribeIdentityPoolError::NotAuthorized(ref cause) => cause,
            DescribeIdentityPoolError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityPoolError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCredentialsForIdentity
#[derive(Debug, PartialEq)]
pub enum GetCredentialsForIdentityError {
    /// <p>An exception thrown when a dependent service such as Facebook or Twitter is not responding</p>
    ExternalService(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown if the identity pool has no role associated for the given auth type (auth/unauth) or if the AssumeRole fails.</p>
    InvalidIdentityPoolConfiguration(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl GetCredentialsForIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCredentialsForIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExternalServiceException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::ExternalService(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidIdentityPoolConfigurationException" => {
                    return RusotoError::Service(
                        GetCredentialsForIdentityError::InvalidIdentityPoolConfiguration(
                            String::from(error_message),
                        ),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCredentialsForIdentityError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCredentialsForIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCredentialsForIdentityError {
    fn description(&self) -> &str {
        match *self {
            GetCredentialsForIdentityError::ExternalService(ref cause) => cause,
            GetCredentialsForIdentityError::InternalError(ref cause) => cause,
            GetCredentialsForIdentityError::InvalidIdentityPoolConfiguration(ref cause) => cause,
            GetCredentialsForIdentityError::InvalidParameter(ref cause) => cause,
            GetCredentialsForIdentityError::NotAuthorized(ref cause) => cause,
            GetCredentialsForIdentityError::ResourceConflict(ref cause) => cause,
            GetCredentialsForIdentityError::ResourceNotFound(ref cause) => cause,
            GetCredentialsForIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetId
#[derive(Debug, PartialEq)]
pub enum GetIdError {
    /// <p>An exception thrown when a dependent service such as Facebook or Twitter is not responding</p>
    ExternalService(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when the total number of user pools has exceeded a preset limit.</p>
    LimitExceeded(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl GetIdError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIdError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExternalServiceException" => {
                    return RusotoError::Service(GetIdError::ExternalService(String::from(
                        error_message,
                    )))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetIdError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetIdError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetIdError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetIdError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(GetIdError::ResourceConflict(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetIdError::ResourceNotFound(String::from(
                        error_message,
                    )))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIdError::TooManyRequests(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdError {
    fn description(&self) -> &str {
        match *self {
            GetIdError::ExternalService(ref cause) => cause,
            GetIdError::InternalError(ref cause) => cause,
            GetIdError::InvalidParameter(ref cause) => cause,
            GetIdError::LimitExceeded(ref cause) => cause,
            GetIdError::NotAuthorized(ref cause) => cause,
            GetIdError::ResourceConflict(ref cause) => cause,
            GetIdError::ResourceNotFound(ref cause) => cause,
            GetIdError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityPoolRoles
#[derive(Debug, PartialEq)]
pub enum GetIdentityPoolRolesError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl GetIdentityPoolRolesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIdentityPoolRolesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIdentityPoolRolesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIdentityPoolRolesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityPoolRolesError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityPoolRolesError::InternalError(ref cause) => cause,
            GetIdentityPoolRolesError::InvalidParameter(ref cause) => cause,
            GetIdentityPoolRolesError::NotAuthorized(ref cause) => cause,
            GetIdentityPoolRolesError::ResourceConflict(ref cause) => cause,
            GetIdentityPoolRolesError::ResourceNotFound(ref cause) => cause,
            GetIdentityPoolRolesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOpenIdToken
#[derive(Debug, PartialEq)]
pub enum GetOpenIdTokenError {
    /// <p>An exception thrown when a dependent service such as Facebook or Twitter is not responding</p>
    ExternalService(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl GetOpenIdTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOpenIdTokenError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExternalServiceException" => {
                    return RusotoError::Service(GetOpenIdTokenError::ExternalService(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetOpenIdTokenError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetOpenIdTokenError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetOpenIdTokenError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(GetOpenIdTokenError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetOpenIdTokenError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetOpenIdTokenError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOpenIdTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOpenIdTokenError {
    fn description(&self) -> &str {
        match *self {
            GetOpenIdTokenError::ExternalService(ref cause) => cause,
            GetOpenIdTokenError::InternalError(ref cause) => cause,
            GetOpenIdTokenError::InvalidParameter(ref cause) => cause,
            GetOpenIdTokenError::NotAuthorized(ref cause) => cause,
            GetOpenIdTokenError::ResourceConflict(ref cause) => cause,
            GetOpenIdTokenError::ResourceNotFound(ref cause) => cause,
            GetOpenIdTokenError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOpenIdTokenForDeveloperIdentity
#[derive(Debug, PartialEq)]
pub enum GetOpenIdTokenForDeveloperIdentityError {
    /// <p>The provided developer user identifier is already registered with Cognito under a different identity ID.</p>
    DeveloperUserAlreadyRegistered(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl GetOpenIdTokenForDeveloperIdentityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetOpenIdTokenForDeveloperIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DeveloperUserAlreadyRegisteredException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::DeveloperUserAlreadyRegistered(
                            String::from(error_message),
                        ),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::InternalError(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::InvalidParameter(String::from(
                            error_message,
                        )),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::NotAuthorized(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::ResourceConflict(String::from(
                            error_message,
                        )),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetOpenIdTokenForDeveloperIdentityError::TooManyRequests(String::from(
                            error_message,
                        )),
                    )
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOpenIdTokenForDeveloperIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOpenIdTokenForDeveloperIdentityError {
    fn description(&self) -> &str {
        match *self {
            GetOpenIdTokenForDeveloperIdentityError::DeveloperUserAlreadyRegistered(ref cause) => {
                cause
            }
            GetOpenIdTokenForDeveloperIdentityError::InternalError(ref cause) => cause,
            GetOpenIdTokenForDeveloperIdentityError::InvalidParameter(ref cause) => cause,
            GetOpenIdTokenForDeveloperIdentityError::NotAuthorized(ref cause) => cause,
            GetOpenIdTokenForDeveloperIdentityError::ResourceConflict(ref cause) => cause,
            GetOpenIdTokenForDeveloperIdentityError::ResourceNotFound(ref cause) => cause,
            GetOpenIdTokenForDeveloperIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentities
#[derive(Debug, PartialEq)]
pub enum ListIdentitiesError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl ListIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentitiesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListIdentitiesError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIdentitiesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListIdentitiesError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListIdentitiesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListIdentitiesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListIdentitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentitiesError {
    fn description(&self) -> &str {
        match *self {
            ListIdentitiesError::InternalError(ref cause) => cause,
            ListIdentitiesError::InvalidParameter(ref cause) => cause,
            ListIdentitiesError::NotAuthorized(ref cause) => cause,
            ListIdentitiesError::ResourceNotFound(ref cause) => cause,
            ListIdentitiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentityPools
#[derive(Debug, PartialEq)]
pub enum ListIdentityPoolsError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl ListIdentityPoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentityPoolsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(ListIdentityPoolsError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIdentityPoolsError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListIdentityPoolsError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListIdentityPoolsError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListIdentityPoolsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentityPoolsError {
    fn description(&self) -> &str {
        match *self {
            ListIdentityPoolsError::InternalError(ref cause) => cause,
            ListIdentityPoolsError::InvalidParameter(ref cause) => cause,
            ListIdentityPoolsError::NotAuthorized(ref cause) => cause,
            ListIdentityPoolsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by LookupDeveloperIdentity
#[derive(Debug, PartialEq)]
pub enum LookupDeveloperIdentityError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl LookupDeveloperIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LookupDeveloperIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(LookupDeveloperIdentityError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LookupDeveloperIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LookupDeveloperIdentityError {
    fn description(&self) -> &str {
        match *self {
            LookupDeveloperIdentityError::InternalError(ref cause) => cause,
            LookupDeveloperIdentityError::InvalidParameter(ref cause) => cause,
            LookupDeveloperIdentityError::NotAuthorized(ref cause) => cause,
            LookupDeveloperIdentityError::ResourceConflict(ref cause) => cause,
            LookupDeveloperIdentityError::ResourceNotFound(ref cause) => cause,
            LookupDeveloperIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by MergeDeveloperIdentities
#[derive(Debug, PartialEq)]
pub enum MergeDeveloperIdentitiesError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl MergeDeveloperIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MergeDeveloperIdentitiesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(MergeDeveloperIdentitiesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for MergeDeveloperIdentitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MergeDeveloperIdentitiesError {
    fn description(&self) -> &str {
        match *self {
            MergeDeveloperIdentitiesError::InternalError(ref cause) => cause,
            MergeDeveloperIdentitiesError::InvalidParameter(ref cause) => cause,
            MergeDeveloperIdentitiesError::NotAuthorized(ref cause) => cause,
            MergeDeveloperIdentitiesError::ResourceConflict(ref cause) => cause,
            MergeDeveloperIdentitiesError::ResourceNotFound(ref cause) => cause,
            MergeDeveloperIdentitiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityPoolRoles
#[derive(Debug, PartialEq)]
pub enum SetIdentityPoolRolesError {
    /// <p>Thrown if there are parallel requests to modify a resource.</p>
    ConcurrentModification(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl SetIdentityPoolRolesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetIdentityPoolRolesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetIdentityPoolRolesError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetIdentityPoolRolesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityPoolRolesError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityPoolRolesError::ConcurrentModification(ref cause) => cause,
            SetIdentityPoolRolesError::InternalError(ref cause) => cause,
            SetIdentityPoolRolesError::InvalidParameter(ref cause) => cause,
            SetIdentityPoolRolesError::NotAuthorized(ref cause) => cause,
            SetIdentityPoolRolesError::ResourceConflict(ref cause) => cause,
            SetIdentityPoolRolesError::ResourceNotFound(ref cause) => cause,
            SetIdentityPoolRolesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UnlinkDeveloperIdentity
#[derive(Debug, PartialEq)]
pub enum UnlinkDeveloperIdentityError {
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl UnlinkDeveloperIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnlinkDeveloperIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UnlinkDeveloperIdentityError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UnlinkDeveloperIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnlinkDeveloperIdentityError {
    fn description(&self) -> &str {
        match *self {
            UnlinkDeveloperIdentityError::InternalError(ref cause) => cause,
            UnlinkDeveloperIdentityError::InvalidParameter(ref cause) => cause,
            UnlinkDeveloperIdentityError::NotAuthorized(ref cause) => cause,
            UnlinkDeveloperIdentityError::ResourceConflict(ref cause) => cause,
            UnlinkDeveloperIdentityError::ResourceNotFound(ref cause) => cause,
            UnlinkDeveloperIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UnlinkIdentity
#[derive(Debug, PartialEq)]
pub enum UnlinkIdentityError {
    /// <p>An exception thrown when a dependent service such as Facebook or Twitter is not responding</p>
    ExternalService(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl UnlinkIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnlinkIdentityError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExternalServiceException" => {
                    return RusotoError::Service(UnlinkIdentityError::ExternalService(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UnlinkIdentityError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UnlinkIdentityError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UnlinkIdentityError::NotAuthorized(String::from(
                        error_message,
                    )))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UnlinkIdentityError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnlinkIdentityError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UnlinkIdentityError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UnlinkIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnlinkIdentityError {
    fn description(&self) -> &str {
        match *self {
            UnlinkIdentityError::ExternalService(ref cause) => cause,
            UnlinkIdentityError::InternalError(ref cause) => cause,
            UnlinkIdentityError::InvalidParameter(ref cause) => cause,
            UnlinkIdentityError::NotAuthorized(ref cause) => cause,
            UnlinkIdentityError::ResourceConflict(ref cause) => cause,
            UnlinkIdentityError::ResourceNotFound(ref cause) => cause,
            UnlinkIdentityError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIdentityPool
#[derive(Debug, PartialEq)]
pub enum UpdateIdentityPoolError {
    /// <p>Thrown if there are parallel requests to modify a resource.</p>
    ConcurrentModification(String),
    /// <p>Thrown when the service encounters an error during processing the request.</p>
    InternalError(String),
    /// <p>Thrown for missing or bad input parameter(s).</p>
    InvalidParameter(String),
    /// <p>Thrown when the total number of user pools has exceeded a preset limit.</p>
    LimitExceeded(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown when a user tries to use a login which is already linked to another account.</p>
    ResourceConflict(String),
    /// <p>Thrown when the requested resource (for example, a dataset or record) does not exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown when a request is throttled.</p>
    TooManyRequests(String),
}

impl UpdateIdentityPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIdentityPoolError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::NotAuthorized(
                        String::from(error_message),
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::ResourceConflict(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIdentityPoolError::TooManyRequests(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateIdentityPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIdentityPoolError {
    fn description(&self) -> &str {
        match *self {
            UpdateIdentityPoolError::ConcurrentModification(ref cause) => cause,
            UpdateIdentityPoolError::InternalError(ref cause) => cause,
            UpdateIdentityPoolError::InvalidParameter(ref cause) => cause,
            UpdateIdentityPoolError::LimitExceeded(ref cause) => cause,
            UpdateIdentityPoolError::NotAuthorized(ref cause) => cause,
            UpdateIdentityPoolError::ResourceConflict(ref cause) => cause,
            UpdateIdentityPoolError::ResourceNotFound(ref cause) => cause,
            UpdateIdentityPoolError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Cognito Identity API. Amazon Cognito Identity clients implement this trait.
pub trait CognitoIdentity {
    /// <p>Creates a new identity pool. The identity pool is a store of user identity information that is specific to your AWS account. The limit on identity pools is 60 per account. The keys for <code>SupportedLoginProviders</code> are as follows:</p> <ul> <li> <p>Facebook: <code>graph.facebook.com</code> </p> </li> <li> <p>Google: <code>accounts.google.com</code> </p> </li> <li> <p>Amazon: <code>www.amazon.com</code> </p> </li> <li> <p>Twitter: <code>api.twitter.com</code> </p> </li> <li> <p>Digits: <code>www.digits.com</code> </p> </li> </ul> <p>You must use AWS Developer credentials to call this API.</p>
    fn create_identity_pool(
        &self,
        input: CreateIdentityPoolInput,
    ) -> RusotoFuture<IdentityPool, CreateIdentityPoolError>;

    /// <p>Deletes identities from an identity pool. You can specify a list of 1-60 identities that you want to delete.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn delete_identities(
        &self,
        input: DeleteIdentitiesInput,
    ) -> RusotoFuture<DeleteIdentitiesResponse, DeleteIdentitiesError>;

    /// <p>Deletes a user pool. Once a pool is deleted, users will not be able to authenticate with the pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn delete_identity_pool(
        &self,
        input: DeleteIdentityPoolInput,
    ) -> RusotoFuture<(), DeleteIdentityPoolError>;

    /// <p>Returns metadata related to the given identity, including when the identity was created and any associated linked logins.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn describe_identity(
        &self,
        input: DescribeIdentityInput,
    ) -> RusotoFuture<IdentityDescription, DescribeIdentityError>;

    /// <p>Gets details about a particular identity pool, including the pool name, ID description, creation date, and current number of users.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn describe_identity_pool(
        &self,
        input: DescribeIdentityPoolInput,
    ) -> RusotoFuture<IdentityPool, DescribeIdentityPoolError>;

    /// <p>Returns credentials for the provided identity ID. Any provided logins will be validated against supported login providers. If the token is for cognito-identity.amazonaws.com, it will be passed through to AWS Security Token Service with the appropriate role for the token.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_credentials_for_identity(
        &self,
        input: GetCredentialsForIdentityInput,
    ) -> RusotoFuture<GetCredentialsForIdentityResponse, GetCredentialsForIdentityError>;

    /// <p>Generates (or retrieves) a Cognito ID. Supplying multiple logins will create an implicit linked account.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_id(&self, input: GetIdInput) -> RusotoFuture<GetIdResponse, GetIdError>;

    /// <p>Gets the roles for an identity pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn get_identity_pool_roles(
        &self,
        input: GetIdentityPoolRolesInput,
    ) -> RusotoFuture<GetIdentityPoolRolesResponse, GetIdentityPoolRolesError>;

    /// <p>Gets an OpenID token, using a known Cognito ID. This known Cognito ID is returned by <a>GetId</a>. You can optionally add additional logins for the identity. Supplying multiple logins creates an implicit link.</p> <p>The OpenId token is valid for 15 minutes.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_open_id_token(
        &self,
        input: GetOpenIdTokenInput,
    ) -> RusotoFuture<GetOpenIdTokenResponse, GetOpenIdTokenError>;

    /// <p>Registers (or retrieves) a Cognito <code>IdentityId</code> and an OpenID Connect token for a user authenticated by your backend authentication process. Supplying multiple logins will create an implicit linked account. You can only specify one developer provider as part of the <code>Logins</code> map, which is linked to the identity pool. The developer provider is the "domain" by which Cognito will refer to your users.</p> <p>You can use <code>GetOpenIdTokenForDeveloperIdentity</code> to create a new identity and to link new logins (that is, user credentials issued by a public provider or developer provider) to an existing identity. When you want to create a new identity, the <code>IdentityId</code> should be null. When you want to associate a new login with an existing authenticated/unauthenticated identity, you can do so by providing the existing <code>IdentityId</code>. This API will create the identity in the specified <code>IdentityPoolId</code>.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn get_open_id_token_for_developer_identity(
        &self,
        input: GetOpenIdTokenForDeveloperIdentityInput,
    ) -> RusotoFuture<
        GetOpenIdTokenForDeveloperIdentityResponse,
        GetOpenIdTokenForDeveloperIdentityError,
    >;

    /// <p>Lists the identities in a pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn list_identities(
        &self,
        input: ListIdentitiesInput,
    ) -> RusotoFuture<ListIdentitiesResponse, ListIdentitiesError>;

    /// <p>Lists all of the Cognito identity pools registered for your account.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn list_identity_pools(
        &self,
        input: ListIdentityPoolsInput,
    ) -> RusotoFuture<ListIdentityPoolsResponse, ListIdentityPoolsError>;

    /// <p>Retrieves the <code>IdentityID</code> associated with a <code>DeveloperUserIdentifier</code> or the list of <code>DeveloperUserIdentifier</code>s associated with an <code>IdentityId</code> for an existing identity. Either <code>IdentityID</code> or <code>DeveloperUserIdentifier</code> must not be null. If you supply only one of these values, the other value will be searched in the database and returned as a part of the response. If you supply both, <code>DeveloperUserIdentifier</code> will be matched against <code>IdentityID</code>. If the values are verified against the database, the response returns both values and is the same as the request. Otherwise a <code>ResourceConflictException</code> is thrown.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn lookup_developer_identity(
        &self,
        input: LookupDeveloperIdentityInput,
    ) -> RusotoFuture<LookupDeveloperIdentityResponse, LookupDeveloperIdentityError>;

    /// <p>Merges two users having different <code>IdentityId</code>s, existing in the same identity pool, and identified by the same developer provider. You can use this action to request that discrete users be merged and identified as a single user in the Cognito environment. Cognito associates the given source user (<code>SourceUserIdentifier</code>) with the <code>IdentityId</code> of the <code>DestinationUserIdentifier</code>. Only developer-authenticated users can be merged. If the users to be merged are associated with the same public provider, but as two different users, an exception will be thrown.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn merge_developer_identities(
        &self,
        input: MergeDeveloperIdentitiesInput,
    ) -> RusotoFuture<MergeDeveloperIdentitiesResponse, MergeDeveloperIdentitiesError>;

    /// <p>Sets the roles for an identity pool. These roles are used when making calls to <a>GetCredentialsForIdentity</a> action.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn set_identity_pool_roles(
        &self,
        input: SetIdentityPoolRolesInput,
    ) -> RusotoFuture<(), SetIdentityPoolRolesError>;

    /// <p>Unlinks a <code>DeveloperUserIdentifier</code> from an existing identity. Unlinked developer users will be considered new identities next time they are seen. If, for a given Cognito identity, you remove all federated identities as well as the developer user identifier, the Cognito identity becomes inaccessible.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn unlink_developer_identity(
        &self,
        input: UnlinkDeveloperIdentityInput,
    ) -> RusotoFuture<(), UnlinkDeveloperIdentityError>;

    /// <p>Unlinks a federated identity from an existing account. Unlinked logins will be considered new identities next time they are seen. Removing the last linked login will make this identity inaccessible.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn unlink_identity(&self, input: UnlinkIdentityInput) -> RusotoFuture<(), UnlinkIdentityError>;

    /// <p>Updates a user pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn update_identity_pool(
        &self,
        input: IdentityPool,
    ) -> RusotoFuture<IdentityPool, UpdateIdentityPoolError>;
}
/// A client for the Amazon Cognito Identity API.
#[derive(Clone)]
pub struct CognitoIdentityClient {
    client: Client,
    region: region::Region,
}

impl CognitoIdentityClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CognitoIdentityClient {
        CognitoIdentityClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CognitoIdentityClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CognitoIdentityClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CognitoIdentity for CognitoIdentityClient {
    /// <p>Creates a new identity pool. The identity pool is a store of user identity information that is specific to your AWS account. The limit on identity pools is 60 per account. The keys for <code>SupportedLoginProviders</code> are as follows:</p> <ul> <li> <p>Facebook: <code>graph.facebook.com</code> </p> </li> <li> <p>Google: <code>accounts.google.com</code> </p> </li> <li> <p>Amazon: <code>www.amazon.com</code> </p> </li> <li> <p>Twitter: <code>api.twitter.com</code> </p> </li> <li> <p>Digits: <code>www.digits.com</code> </p> </li> </ul> <p>You must use AWS Developer credentials to call this API.</p>
    fn create_identity_pool(
        &self,
        input: CreateIdentityPoolInput,
    ) -> RusotoFuture<IdentityPool, CreateIdentityPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.CreateIdentityPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IdentityPool>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateIdentityPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes identities from an identity pool. You can specify a list of 1-60 identities that you want to delete.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn delete_identities(
        &self,
        input: DeleteIdentitiesInput,
    ) -> RusotoFuture<DeleteIdentitiesResponse, DeleteIdentitiesError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.DeleteIdentities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteIdentitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIdentitiesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user pool. Once a pool is deleted, users will not be able to authenticate with the pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn delete_identity_pool(
        &self,
        input: DeleteIdentityPoolInput,
    ) -> RusotoFuture<(), DeleteIdentityPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.DeleteIdentityPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIdentityPoolError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns metadata related to the given identity, including when the identity was created and any associated linked logins.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn describe_identity(
        &self,
        input: DescribeIdentityInput,
    ) -> RusotoFuture<IdentityDescription, DescribeIdentityError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.DescribeIdentity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IdentityDescription>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeIdentityError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details about a particular identity pool, including the pool name, ID description, creation date, and current number of users.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn describe_identity_pool(
        &self,
        input: DescribeIdentityPoolInput,
    ) -> RusotoFuture<IdentityPool, DescribeIdentityPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.DescribeIdentityPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IdentityPool>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeIdentityPoolError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns credentials for the provided identity ID. Any provided logins will be validated against supported login providers. If the token is for cognito-identity.amazonaws.com, it will be passed through to AWS Security Token Service with the appropriate role for the token.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_credentials_for_identity(
        &self,
        input: GetCredentialsForIdentityInput,
    ) -> RusotoFuture<GetCredentialsForIdentityResponse, GetCredentialsForIdentityError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.GetCredentialsForIdentity",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCredentialsForIdentityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCredentialsForIdentityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Generates (or retrieves) a Cognito ID. Supplying multiple logins will create an implicit linked account.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_id(&self, input: GetIdInput) -> RusotoFuture<GetIdResponse, GetIdError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.GetId");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetIdResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIdError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the roles for an identity pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn get_identity_pool_roles(
        &self,
        input: GetIdentityPoolRolesInput,
    ) -> RusotoFuture<GetIdentityPoolRolesResponse, GetIdentityPoolRolesError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.GetIdentityPoolRoles",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetIdentityPoolRolesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetIdentityPoolRolesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets an OpenID token, using a known Cognito ID. This known Cognito ID is returned by <a>GetId</a>. You can optionally add additional logins for the identity. Supplying multiple logins creates an implicit link.</p> <p>The OpenId token is valid for 15 minutes.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn get_open_id_token(
        &self,
        input: GetOpenIdTokenInput,
    ) -> RusotoFuture<GetOpenIdTokenResponse, GetOpenIdTokenError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.GetOpenIdToken");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOpenIdTokenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOpenIdTokenError::from_response(response))),
                )
            }
        })
    }

    /// <p>Registers (or retrieves) a Cognito <code>IdentityId</code> and an OpenID Connect token for a user authenticated by your backend authentication process. Supplying multiple logins will create an implicit linked account. You can only specify one developer provider as part of the <code>Logins</code> map, which is linked to the identity pool. The developer provider is the "domain" by which Cognito will refer to your users.</p> <p>You can use <code>GetOpenIdTokenForDeveloperIdentity</code> to create a new identity and to link new logins (that is, user credentials issued by a public provider or developer provider) to an existing identity. When you want to create a new identity, the <code>IdentityId</code> should be null. When you want to associate a new login with an existing authenticated/unauthenticated identity, you can do so by providing the existing <code>IdentityId</code>. This API will create the identity in the specified <code>IdentityPoolId</code>.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn get_open_id_token_for_developer_identity(
        &self,
        input: GetOpenIdTokenForDeveloperIdentityInput,
    ) -> RusotoFuture<
        GetOpenIdTokenForDeveloperIdentityResponse,
        GetOpenIdTokenForDeveloperIdentityError,
    > {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.GetOpenIdTokenForDeveloperIdentity",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOpenIdTokenForDeveloperIdentityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetOpenIdTokenForDeveloperIdentityError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the identities in a pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn list_identities(
        &self,
        input: ListIdentitiesInput,
    ) -> RusotoFuture<ListIdentitiesResponse, ListIdentitiesError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.ListIdentities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListIdentitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIdentitiesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all of the Cognito identity pools registered for your account.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn list_identity_pools(
        &self,
        input: ListIdentityPoolsInput,
    ) -> RusotoFuture<ListIdentityPoolsResponse, ListIdentityPoolsError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.ListIdentityPools",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListIdentityPoolsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListIdentityPoolsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the <code>IdentityID</code> associated with a <code>DeveloperUserIdentifier</code> or the list of <code>DeveloperUserIdentifier</code>s associated with an <code>IdentityId</code> for an existing identity. Either <code>IdentityID</code> or <code>DeveloperUserIdentifier</code> must not be null. If you supply only one of these values, the other value will be searched in the database and returned as a part of the response. If you supply both, <code>DeveloperUserIdentifier</code> will be matched against <code>IdentityID</code>. If the values are verified against the database, the response returns both values and is the same as the request. Otherwise a <code>ResourceConflictException</code> is thrown.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn lookup_developer_identity(
        &self,
        input: LookupDeveloperIdentityInput,
    ) -> RusotoFuture<LookupDeveloperIdentityResponse, LookupDeveloperIdentityError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.LookupDeveloperIdentity",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<LookupDeveloperIdentityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(LookupDeveloperIdentityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Merges two users having different <code>IdentityId</code>s, existing in the same identity pool, and identified by the same developer provider. You can use this action to request that discrete users be merged and identified as a single user in the Cognito environment. Cognito associates the given source user (<code>SourceUserIdentifier</code>) with the <code>IdentityId</code> of the <code>DestinationUserIdentifier</code>. Only developer-authenticated users can be merged. If the users to be merged are associated with the same public provider, but as two different users, an exception will be thrown.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn merge_developer_identities(
        &self,
        input: MergeDeveloperIdentitiesInput,
    ) -> RusotoFuture<MergeDeveloperIdentitiesResponse, MergeDeveloperIdentitiesError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.MergeDeveloperIdentities",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<MergeDeveloperIdentitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(MergeDeveloperIdentitiesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the roles for an identity pool. These roles are used when making calls to <a>GetCredentialsForIdentity</a> action.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn set_identity_pool_roles(
        &self,
        input: SetIdentityPoolRolesInput,
    ) -> RusotoFuture<(), SetIdentityPoolRolesError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.SetIdentityPoolRoles",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SetIdentityPoolRolesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Unlinks a <code>DeveloperUserIdentifier</code> from an existing identity. Unlinked developer users will be considered new identities next time they are seen. If, for a given Cognito identity, you remove all federated identities as well as the developer user identifier, the Cognito identity becomes inaccessible.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn unlink_developer_identity(
        &self,
        input: UnlinkDeveloperIdentityInput,
    ) -> RusotoFuture<(), UnlinkDeveloperIdentityError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.UnlinkDeveloperIdentity",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UnlinkDeveloperIdentityError::from_response(response))
                }))
            }
        })
    }

    /// <p>Unlinks a federated identity from an existing account. Unlinked logins will be considered new identities next time they are seen. Removing the last linked login will make this identity inaccessible.</p> <p>This is a public API. You do not need any credentials to call this API.</p>
    fn unlink_identity(&self, input: UnlinkIdentityInput) -> RusotoFuture<(), UnlinkIdentityError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSCognitoIdentityService.UnlinkIdentity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UnlinkIdentityError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a user pool.</p> <p>You must use AWS Developer credentials to call this API.</p>
    fn update_identity_pool(
        &self,
        input: IdentityPool,
    ) -> RusotoFuture<IdentityPool, UpdateIdentityPoolError> {
        let mut request = SignedRequest::new("POST", "cognito-identity", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSCognitoIdentityService.UpdateIdentityPool",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IdentityPool>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateIdentityPoolError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
