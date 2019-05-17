// importScripts('wasm_thread.js')

console.log('一个新的Worker')
self.addEventListener('message', function(e) {
  console.log('worker message', e)
  // wasm_bindgen.fuck();
  postMessage(['[fuck from worker]',2,3,{a:'fuck'}])
})
