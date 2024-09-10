enum Env {
    Development = 1 // 开发环境
    Online = 2      // 线上环境
}

enum ProxyType {
    TLB = 1          // TLB配置
    ProxyService = 2 // 代理服务配置
}

enum RootPathStatus {
    UnFinish = 1,      // 未完成配置
    Accessible = 2,    // 可访问
    UnAccessible = 3,  // 不可访问
}

enum RootPathPublishStatus {
    // 未发布
    // 未发布
    /* 另一种注释 */
    /* 另一种注释 */
    Draft = 1,       
    Published = 2,     // 已发布
}