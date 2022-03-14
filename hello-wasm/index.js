// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/hello_wasm.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWasm = await init("./pkg/hello_wasm_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = helloWasm.call_me_from_javascript(24, 24);

  // Set the result onto the body
  document.body.textContent = `Hello Wasm! addResult: ${addResult}`;
  console.log(helloWasm.ADD_CONSTANT); // Should output 'undefined'
  console.log(helloWasm.add_integer_with_constant); // Should output 'undefined'
};
runWasm();
