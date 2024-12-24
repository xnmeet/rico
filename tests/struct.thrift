struct MockFilterMinimal {
    1: bool CreatedBySelf = false
    2: required list<i64> StatusList = [1, 2]
    3: optional list<i64> DistributionStatusList = []  # 空列表
}

struct MockFilterWithOptional {
    1: bool CreatedBySelf = true
    2: required list<i64> StatusList = [3, 4, 5]
    3: optional list<i64> DistributionStatusList = [10, 20]  # 包含可选字段
}

struct MockFilterEmpty {
    1: bool CreatedBySelf = false
    2: required list<i64> StatusList = []  # 空的必填字段
    3: optional list<i64> DistributionStatusList = []  # 空的可选字段
}

struct MockFilterAllFields {
    1: bool CreatedBySelf = true
    2: required list<i64> StatusList = [100, 200, 300]
    3: optional list<i64> DistributionStatusList = [400, 500]  # 全部字段都有值
}

struct MockFilterEdgeCases {
    1: bool CreatedBySelf = true
    2: required list<i64> StatusList = [-1, 9223372036854775807]  # 边界值
    3: optional list<i64> DistributionStatusList = [0]  # 包含零
}

struct MockFilterLargeData {
    1: bool CreatedBySelf = false
    2: required list<i64> StatusList = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  # 大量数据
    3: optional list<i64> DistributionStatusList = [100, 200, 300, 400, 500]  # 大量可选数据
}

struct MockFilterString {
    1: bool CreatedBySelf = true  # 是否由自己创建
    2: required string Name = "DefaultName"  # 默认名称
    3: optional string Description = null  # 可选描述，默认为 null
}

struct MockFilterFloat {
    1: bool CreatedBySelf = false
    2: required list<float> Ratings = [4.5, 3.8, 5.0]  # 评分列表
    3: optional float AverageRating = 4.1  # 平均评分，默认值
}

struct MockFilterMap {
    1: bool CreatedBySelf = true
    2: required map<string, i64> StatusMap = {"active": 1, "inactive": 0}  # 状态映射
    3: optional map<string, string> Metadata = {}  # 可选元数据，默认为空
}

struct MockFilterComplex {
    1: bool CreatedBySelf = false
    2: required list<MockFilterString> SubFilters = []  # 子过滤器列表
    3: optional MockFilterFloat AggregateData = null  # 聚合数据，默认为 null
}

struct MockFilterDate {
    1: bool CreatedBySelf = true
    2: required i64 CreatedAt = 1672531199000  # 创建时间，默认值为某个时间戳
    3: optional i64 UpdatedAt = null  # 更新时间，默认为 null
}

struct MockFilterMultiLineComment {
    1: bool CreatedBySelf = false
    2: required list<i64> StatusList = [1, 2, 3]  # 状态列表
    3: optional list<string> Notes = [
        "这是一个可选的注释",  # 单行注释
        "可以包含多行注释，\n用于详细描述"  # 多行注释
    ]
    4: optional map<string, string> Metadata = {}
}