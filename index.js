// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/mcmc_example.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("./pkg/mcmc_example_bg.wasm");

  // Call the Add function export from wasm, save the result
  helloWorld.run();

//   // Set the result onto the body
//   document.body.textContent = `Hello World! addResult: ${addResult}`;
};
runWasm();