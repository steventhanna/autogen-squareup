# EventData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The name of the affected object’s type. | [optional]
**id** | Option<**String**> | The ID of the affected object. | [optional]
**deleted** | Option<**bool**> | This is true if the affected object has been deleted; otherwise, it's absent. | [optional]
**object** | Option<[**serde_json::Value**](.md)> | An object containing fields and values relevant to the event. It is absent if the affected object has been deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


