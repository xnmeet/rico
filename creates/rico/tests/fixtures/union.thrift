// Union type for a value that can be either an integer or a string
union IntOrString {
    1: i32 int_value
    2: string string_value
}

// Union type for a result that can be a user object, an error message, or a boolean indicating success
union UserResult {
    1: User user_data
    2: string error_message
    3: bool success
}

// Union type for a configuration option that can be an integer, a string, or a list of strings
union ConfigOption {
    1: i32 int_setting
    2: string string_setting
    3: list<string> string_list_setting
}

// Union type for a data container that can hold different types of data related to an item
union ItemData {
    1: i32 item_id
    2: string item_name
    3: map<string, i32> item_attributes
    4: list<ItemDetail> item_details
}