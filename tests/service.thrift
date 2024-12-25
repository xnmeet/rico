service AppService {
    // 注意下一行的开头是制表符，由于制表符在 IDE 中是有自己的空格转换逻辑的，可能表现会有差异，导致结果存在位置偏差
    // 所以应该禁止制表符使用
	GetAppInfoResponse GetAppInfo(1: GetAppInfoRequest request )(api.post = "/api/test")
    void GetAppInfo(1: apps.GetAppInfoRequest request=[1,2] )
    record.UpdateRecordByActionResponse UpdateRecordByFlow(1: record.UpdateRecordByFlowRequest request )
}

service ForTest extends AppService {
	GetAppInfoResponse GetAppInfo(1: GetAppInfoRequest request )(api.post = "/api/test")
}(api.gw="/api/test")