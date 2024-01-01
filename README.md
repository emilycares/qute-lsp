# qute-lsp

This is a lsp server for the qurakus qute templating engine. https://quarkus.io/guides/qute-reference

## Features
 - Go to definition for include statements. When you are on a line that includes an ["include"](https://quarkus.io/guides/qute-reference#include_helper) then you can go to that definition
 - Quickfix on html element
   - "Add fragment frame" This will add a fragment definition around the currnt html element.
   - "Extract as file" This will extract the html element into another file. The html element must have an id.
   - "Extract as fragment" This will extract the html element into another file and to a fragment. The html element must have an id.
