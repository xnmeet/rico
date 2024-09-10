struct Filter {
    1: optional bool CreatedBySelf (go.tag = "json:\"createdBySelf\"",api.json = "createdBySelf")
    2: optional list<i64> StatusList (go.tag = "json:\"statusList\"",api.json = "statusList")
    3: optional list<i64> DistributionStatusList (go.tag = "json:\"distributionStatusList\"",api.json = "distributionStatusList")
}