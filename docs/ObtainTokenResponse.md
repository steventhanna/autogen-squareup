# ObtainTokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<**String**> | An OAuth access token used to authorize Square API requests on behalf of the seller. Include this token as a bearer token in the `Authorization` header of your API requests.  OAuth access tokens expire in 30 days (except `short_lived` access tokens). You should call `ObtainToken` and provide the returned `refresh_token` to get a new access token well before the current one expires. For more information, see [OAuth API: Walkthrough](https://developer.squareup.com/docs/oauth-api/walkthrough). | [optional]
**token_type** | Option<**String**> | The type of access token. This value is always `bearer`. | [optional]
**expires_at** | Option<**String**> | The timestamp of when the `access_token` expires, in [ISO 8601](http://www.iso.org/iso/home/standards/iso8601.htm) format. | [optional]
**merchant_id** | Option<**String**> | The ID of the authorizing [merchant](entity:Merchant) (seller), which represents a business. | [optional]
**subscription_id** | Option<**String**> | __LEGACY__ The ID of merchant's subscription. The ID is only present if the merchant signed up for a subscription plan during authorization. | [optional]
**plan_id** | Option<**String**> | __LEGACY__ The ID of the subscription plan the merchant signed up for. The ID is only present if the merchant signed up for a subscription plan during authorization. | [optional]
**id_token** | Option<**String**> | The OpenID token that belongs to this person. This token is only present if the `OPENID` scope is included in the authorization request.  Deprecated at version 2021-09-15. Square doesn't support OpenID or other single sign-on (SSO) protocols on top of OAuth. | [optional]
**refresh_token** | Option<**String**> | A refresh token that can be used in an `ObtainToken` request to generate a new access token.  With the code flow: - For the `authorization_code` grant type, the refresh token is multi-use and never expires. - For the `refresh_token` grant type, the response returns the same refresh token.  With the PKCE flow: - For the `authorization_code` grant type, the refresh token is single-use and expires in 90 days. - For the `refresh_token` grant type, the refresh token is a new single-use refresh token that expires in 90 days.  For more information, see [Refresh, Revoke, and Limit the Scope of OAuth Tokens](https://developer.squareup.com/docs/oauth-api/refresh-revoke-limit-scope). | [optional]
**short_lived** | Option<**bool**> | Indicates whether the access_token is short lived. If `true`, the access token expires in 24 hours. If `false`, the access token expires in 30 days. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]
**refresh_token_expires_at** | Option<**String**> | The timestamp of when the `refresh_token` expires, in [ISO 8601](http://www.iso.org/iso/home/standards/iso8601.htm) format.  This field is only returned for the PKCE flow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


