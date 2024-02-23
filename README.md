# qute-lsp

This is a lsp server for the qurakus qute templating engine. https://quarkus.io/guides/qute-reference

## Features
 - Go to definition for include statements. When you are on a line that includes an ["include"](https://quarkus.io/guides/qute-reference#include_helper) then you can go to that definition
 - Quickfix on html element
   - "Add fragment frame" This will add a fragment definition around the current html element.
   - "Extract as file" This will extract the html element into another file. The html element must have an id.
   - "Extract as fragment" This will extract the html element into another file and to a fragment. The html element must have an id.
 - Completion for common qute features like if, for and fragments ...
 - Completion for quarkus routes inside htmx attributes that require a path

 ## Build requirements
 - rust compiler
 - just
 - nodejs (vscode addon only)

 ## How to install the vscode addon
 ``` bash
 just vscode-build
 ```
 Now you can install this addon into your vscode "plugins/vscode/qute-lsp-vscode-*.vsix"
