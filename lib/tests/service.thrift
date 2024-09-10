include "apps.thrift"

namespace go lark.apaas.app


service AppService {
    /*----------------- 【app_core】应用信息相关接口 ------------------*/

    // 应用详情页聚合查询接口，仅提供前端页面使用
	apps.GetAppInfoResponse GetAppInfo(1: apps.GetAppInfoRequest request )(api.post = "/api/v1/app/namespaces/:namespace/app_info",kgw.login = "true",kgw.role = "package_test_user,package_data_admin")
}
