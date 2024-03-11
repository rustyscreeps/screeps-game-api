object_id_into_uint8array = function(id, arr) {
    // passed array is exactly 16 bytes -- 12 for id, 3 blank, 1 for string pad length
    // initialize all of them so that we can assume they're initialized on the rust side
    const padding = id.length;
    let skip_bytes = 12 - Math.ceil(padding / 2);

    for (let byte = 0; byte < skip_bytes; byte++) {
        arr[byte] = 0;
    }

    const offset = padding % 2;
    // if there's an odd number of characters, grab one for the next byte
    if (offset === 1) {
        arr[skip_bytes] = parseInt(id.substr(0, 1), 16)
        skip_bytes++;
    }

    for (let byte = 0; byte < 12 - skip_bytes; byte++) {
        arr[byte + skip_bytes] = parseInt(id.substr(byte * 2 + offset, 2), 16)
    }
    
    arr[12] = 0;
    arr[13] = 0;
    arr[14] = 0;
    arr[15] = padding;
}
