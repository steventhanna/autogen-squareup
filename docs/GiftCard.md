# GiftCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Square-assigned ID of the gift card. | [optional][readonly]
**r#type** | [**models::GiftCardType**](GiftCardType.md) |  | 
**gan_source** | Option<[**models::GiftCardGanSource**](GiftCardGANSource.md)> |  | [optional]
**state** | Option<[**models::GiftCardStatus**](GiftCardStatus.md)> |  | [optional]
**balance_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**gan** | Option<**String**> | The gift card account number (GAN). Buyers can use the GAN to make purchases or check  the gift card balance. | [optional]
**created_at** | Option<**String**> | The timestamp when the gift card was created, in RFC 3339 format.  In the case of a digital gift card, it is the time when you create a card  (using the Square Point of Sale application, Seller Dashboard, or Gift Cards API).   In the case of a plastic gift card, it is the time when Square associates the card with the  seller at the time of activation. | [optional][readonly]
**customer_ids** | Option<**Vec<String>**> | The IDs of the [customer profiles](entity:Customer) to whom this gift card is linked. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


