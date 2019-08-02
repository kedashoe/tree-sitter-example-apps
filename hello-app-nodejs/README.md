# hello-app-nodejs

This is an example of demonstrating how to use a tree-sitter parser in nodejs.

1. Build the parser in /tree-sitter-hello by running `/tree-sitter-hello/bin/build-nodejs`
2. From here (/hello-app-nodejs):
  1. `npm install` (you will need `python` in your path)
  2. `npm link tree-sitter-hello` so you can `require` the parser
  3. You should now be able to run `node index.js`

# Additional Resources
- https://github.com/tree-sitter/node-tree-sitter
- https://gist.github.com/Aerijo/df27228d70c633e088b0591b8857eeef

