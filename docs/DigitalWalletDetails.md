# DigitalWalletDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The status of the `WALLET` payment. The status can be `AUTHORIZED`, `CAPTURED`, `VOIDED`, or `FAILED`. | [optional]
**brand** | Option<**String**> | The brand used for the `WALLET` payment. The brand can be `CASH_APP`, `PAYPAY`, `ALIPAY`, `RAKUTEN_PAY`, `AU_PAY`, `D_BARAI`, `MERPAY`, `WECHAT_PAY`, `LIGHTNING` or `UNKNOWN`. | [optional]
**cash_app_details** | Option<[**models::CashAppDetails**](CashAppDetails.md)> |  | [optional]
**lightning_details** | Option<[**models::LightningDetails**](LightningDetails.md)> |  | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the payment. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


