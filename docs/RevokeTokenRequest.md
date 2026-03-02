# RevokeTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | The Square-issued ID for your application, which is available on the **OAuth** page in the [Developer Dashboard](https://developer.squareup.com/apps). | [optional]
**access_token** | Option<**String**> | The access token of the merchant whose token you want to revoke. Do not provide a value for `merchant_id` if you provide this parameter. | [optional]
**merchant_id** | Option<**String**> | The ID of the merchant whose token you want to revoke. Do not provide a value for `access_token` if you provide this parameter. | [optional]
**revoke_only_access_token** | Option<**bool**> | If `true`, terminate the given single access token, but do not terminate the entire authorization. Default: `false` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


