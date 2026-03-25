package main

import "github.com/klauspost/compress/zstd"

//export compress_data
func compress_data(input []byte) []byte {
    var encoder, _ = zstd.NewWriter(nil)
    return encoder.EncodeAll(input, make([]byte, 0, len(input)))
}
