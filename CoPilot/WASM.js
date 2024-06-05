fetch('path/to/your/module.wasm')
    .then(response => response.arrayBuffer())
    .then(buffer => WebAssembly.instantiate(buffer))
    .then(instance => {/**Access exported functions or memory here*/ })
    .catch(error => console.error('Error loading WebAssembly:', error));
const wasmModule = instance.module;
const result = wasmModule.exports.myFunction(42); // Call your exported function
console.log('Result:', result);