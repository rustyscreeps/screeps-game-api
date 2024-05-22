module.exports.bodypart_to_part_num = function(part_str_to_num_map, bodypart) {
    return part_str_to_num_map.get(bodypart.type)
}

module.exports.part_nums_to_str_array = function(part_num_to_str_map, body_num_array) {
    // this is a Uint8Array and its map can't produce strings as-is,
    // spread it first so the map can result in an array with constant strings
    return [...body_num_array].map((v) => part_num_to_str_map.get(v));
}
