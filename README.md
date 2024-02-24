# qute-lsp

This is a lsp server for the qurakus qute templating engine. https://quarkus.io/guides/qute-reference

## Not finished
There are still some thins that need to be improved. If you have any issue please create an issue.

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

## How to install the vscode plugin
``` command
just vscode-build
```
Now you can install this addon into your vscode "plugins/vscode/qute-lsp-vscode-*.vsix"
Also, you must add the logana executable located here "target/release/qute-lsp" to your path.

## How to use inside of neovim
``` command
just build
```
This lsp server has not been added to lspconfig yet. So there needs to be some setup.
``` lua
local configs = require("lspconfig.configs")

if not configs.qute_lsp then
  configs.qute_lsp = {
    default_config = {
      cmd = { "qute-lsp" },
      filetypes = { "html" },
      root_dir = function(fname)
        return require("lspconfig").util.find_git_ancestor(fname)
      end,
      autostart = true,
      settings = {},
    },
  }
end
require("lspconfig").qute_lsp.setup({ ...... })
```
