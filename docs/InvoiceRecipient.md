# InvoiceRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The ID of the customer. This is the customer profile ID that  you provide when creating a draft invoice. | [optional]
**given_name** | Option<**String**> | The recipient's given (that is, first) name. | [optional][readonly]
**family_name** | Option<**String**> | The recipient's family (that is, last) name. | [optional][readonly]
**email_address** | Option<**String**> | The recipient's email address. | [optional][readonly]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**phone_number** | Option<**String**> | The recipient's phone number. | [optional][readonly]
**company_name** | Option<**String**> | The name of the recipient's company. | [optional][readonly]
**tax_ids** | Option<[**models::InvoiceRecipientTaxIds**](InvoiceRecipientTaxIds.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


