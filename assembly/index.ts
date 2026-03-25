// assembly/index.ts
export fn calculate_simple_checksum(data: Uint8Array): u32 {
  let hash: u32 = 0;
  for (let i = 0; i < data.length; i++) {
    hash = (hash << 5) - hash + data[i];
  }
  return hash
}
