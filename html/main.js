const GRID_SIZE = 64 * 64;

class Conway {
  constructor(size) {
    this.grid = new Uint8Array(GRID_SIZE);
  }
}

class Wasm {
  constructor() {
    var memory = new WebAssembly.Memory({initial:256, maximum:256});
    var importObject = {
      imports: { imported_func: arg => console.log(arg) },
      env: {
        memory: memory
      }	
    };
    WebAssembly.instantiateStreaming(
      fetch(
        "http://localhost/conway/conway.wasm",
        {
          headers: new Headers({ "Content-Type": "application/wasm" })
        },
      ).then(obj => {
        return obj;
      }), importObject);

  }
}

$(document).ready(function() {
  let wasm = new Wasm();
  let conway = new Conway();
});
