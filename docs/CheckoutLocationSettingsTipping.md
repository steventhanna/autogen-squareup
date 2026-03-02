# CheckoutLocationSettingsTipping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**percentages** | Option<**Vec<i32>**> | Set three custom percentage amounts that buyers can select at checkout. If Smart Tip is enabled, this only applies to transactions totaling $10 or more. | [optional]
**smart_tipping_enabled** | Option<**bool**> | Enables Smart Tip Amounts. If Smart Tip Amounts is enabled, tipping works as follows: If a transaction is less than $10, the available tipping options include No Tip, $1, $2, or $3. If a transaction is $10 or more, the available tipping options include No Tip, 15%, 20%, or 25%.  You can set custom percentage amounts with the `percentages` field. | [optional]
**default_percent** | Option<**i32**> | Set the pre-selected percentage amounts that appear at checkout. If Smart Tip is enabled, this only applies to transactions totaling $10 or more. | [optional]
**smart_tips** | Option<[**Vec<models::Money>**](Money.md)> | Show the Smart Tip Amounts for this location. | [optional]
**default_smart_tip** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


