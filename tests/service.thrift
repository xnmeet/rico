service AppService {
    // 注意下一行的开头是制表符，由于制表符在 IDE 中是有自己的空格转换逻辑的，可能表现会有差异，导致结果存在位置偏差
    // 所以应该禁止制表符使用
	GetInfoResponse GetInfo(1: GetInfoRequest request)(api.post = "/api/test")
    void UpdateInfo(1: UpdateInfoRequest request)
    oneway void GetAppInfo(1: apps.GetAppInfoRequest request=[1,2] )
    record.UpdateRecordByActionResponse UpdateByFlow(1: record.UpdateByFlowRequest request )
}

service ForTest extends AppService {
	GetAppInfoResponse GetAppInfo(1: GetAppInfoRequest request )(api.post = "/api/test")
}(api.gw="/api/test")