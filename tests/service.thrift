service AppService {
	GetAppInfoResponse GetAppInfo(1: GetAppInfoRequest request )(api.post = "/api/test")
    void GetAppInfo(1: apps.GetAppInfoRequest request=[1,2] )
    record.UpdateRecordByActionResponse UpdateRecordByFlow(1: record.UpdateRecordByFlowRequest request )
}

service ForTest extends AppService {
	GetAppInfoResponse GetAppInfo(1: GetAppInfoRequest request )(api.post = "/api/test")
}(api.gw="/api/test")