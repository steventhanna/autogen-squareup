# \OAuthApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**obtain_token**](OAuthApi.md#obtain_token) | **POST** /oauth2/token | ObtainToken
[**retrieve_token_status**](OAuthApi.md#retrieve_token_status) | **POST** /oauth2/token/status | RetrieveTokenStatus
[**revoke_token**](OAuthApi.md#revoke_token) | **POST** /oauth2/revoke | RevokeToken



## obtain_token

> models::ObtainTokenResponse obtain_token(obtain_token_request)
ObtainToken

Returns an OAuth access token and refresh token using the `authorization_code` or `refresh_token` grant type.  When `grant_type` is `authorization_code`: - With the [code flow](https://developer.squareup.com/docs/oauth-api/overview#code-flow), provide `code`, `client_id`, and `client_secret`. - With the [PKCE flow](https://developer.squareup.com/docs/oauth-api/overview#pkce-flow), provide `code`, `client_id`, and `code_verifier`.   When `grant_type` is `refresh_token`: - With the code flow, provide `refresh_token`, `client_id`, and `client_secret`. The response returns the same refresh token provided in the request. - With the PKCE flow, provide `refresh_token` and `client_id`. The response returns a new refresh token.  You can use the `scopes` parameter to limit the set of permissions authorized by the access token. You can use the `short_lived` parameter to create an access token that expires in 24 hours.  __Important:__ OAuth tokens should be encrypted and stored on a secure server. Application clients should never interact directly with OAuth tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**obtain_token_request** | [**ObtainTokenRequest**](ObtainTokenRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::ObtainTokenResponse**](ObtainTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_token_status

> models::RetrieveTokenStatusResponse retrieve_token_status()
RetrieveTokenStatus

Returns information about an [OAuth access token](https://developer.squareup.com/docs/build-basics/access-tokens#get-an-oauth-access-token) or an application’s [personal access token](https://developer.squareup.com/docs/build-basics/access-tokens#get-a-personal-access-token).  Add the access token to the Authorization header of the request.  __Important:__ The `Authorization` header you provide to this endpoint must have the following format:  ``` Authorization: Bearer ACCESS_TOKEN ```  where `ACCESS_TOKEN` is a [valid production authorization credential](https://developer.squareup.com/docs/build-basics/access-tokens).  If the access token is expired or not a valid access token, the endpoint returns an `UNAUTHORIZED` error.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RetrieveTokenStatusResponse**](RetrieveTokenStatusResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_token

> models::RevokeTokenResponse revoke_token(revoke_token_request)
RevokeToken

Revokes an access token generated with the OAuth flow.  If an account has more than one OAuth access token for your application, this endpoint revokes all of them, regardless of which token you specify.   __Important:__ The `Authorization` header for this endpoint must have the following format:  ``` Authorization: Client APPLICATION_SECRET ```  Replace `APPLICATION_SECRET` with the application secret on the **OAuth** page for your application in the Developer Dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_token_request** | [**RevokeTokenRequest**](RevokeTokenRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::RevokeTokenResponse**](RevokeTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

