$(document).ready(function() {

  WebAssembly.instantiateStreaming(fetch('myModule.wasm'), importObject)
  .then(obj => {
  // Call an exported function:
    obj.instance.exports.gr();
})

});
