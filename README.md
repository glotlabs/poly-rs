# poly


## Overview
Poly is a proof of concept web framework for building interactive web applications with Rust.
It's loosely inspired by The Elm Architecture.


## How it works
It compiles to WebAssembly and exposes functions like `init`, `update`, `view`, `subscriptions`, etc. to JavaScript.
[poly-js](https://github.com/glotlabs/poly-js) is a runtime layer that hooks into to these functions and updates the DOM, sets up event listeners, etc.

#### init
Returns the initial model as json.

#### view
The view function returns the page html as a string to JS.

#### update
Is called when an event listener, interval etc is triggered from js. The model is passed as passed as an argument and the new model is returned.
It also returns a list of effects as json, i.e. focus element.

#### subscriptions
Subscriptions are event listeners, intervals, etc. These are returned as json as a declarative list.


## Server-side rendering?
Yes, since it compiles to wasm the same functions mentioned above can also be called from the server.
Should work with any wasm backend, and works with [Cloudflare Pages](https://pages.cloudflare.com/) as an example.


## Docs?
Sorry


## Built with poly
- [glot-app](https://github.com/glotcode/glot-app)
- [html-to-maud](https://github.com/glotlabs/html-to-maud)
