// see https://github.com/tree-sitter/node-tree-sitter

let Parser = require('tree-sitter');
let HelloLang = require('tree-sitter-hello');

let parser = new Parser();
parser.setLanguage(HelloLang);

let code = 'hi mom';
let tree = parser.parse(code);

console.log(tree.rootNode.toString());

