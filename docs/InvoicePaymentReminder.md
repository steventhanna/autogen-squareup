# InvoicePaymentReminder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A Square-assigned ID that uniquely identifies the reminder within the `InvoicePaymentRequest`. | [optional][readonly]
**relative_scheduled_days** | Option<**i32**> | The number of days before (a negative number) or after (a positive number) the payment request `due_date` when the reminder is sent. For example, -3 indicates that the reminder should be sent 3 days before the payment request `due_date`. | [optional]
**message** | Option<**String**> | The reminder message. | [optional]
**status** | Option<[**models::InvoicePaymentReminderStatus**](InvoicePaymentReminderStatus.md)> |  | [optional]
**sent_at** | Option<**String**> | If sent, the timestamp when the reminder was sent, in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


