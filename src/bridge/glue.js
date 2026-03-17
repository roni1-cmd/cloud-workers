import rust_wasm from "../build/main.wasm";
import go_wasm from "../build/validator.wasm";
import c_wasm from "../build/crypto.wasm";

export async function linkModules(env) {
  // Initialize Go environment
  const go = new Go(); 
  const goInstance = await WebAssembly.instantiate(go_wasm, go.importObject);
  go.run(goInstance.instance);

  // Initialize C and Rust
  const rustInstance = await WebAssembly.instantiate(rust_wasm, {
    env: {
      memory: goInstance.instance.exports.memory, // Shared memory space
      obfuscate: c_wasm.exports.obfuscate_meeting_id
    }
  });

  return rustInstance.instance.exports;
}
