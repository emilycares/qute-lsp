build:
  cargo build --release
vscode-setup:
  cd ./plugins/vscode && npm ci
vscode-build: build 
  cd ./plugins/vscode && npm run build
e2e:
  cd ./plugins/vscode && npm run compile && npm run test
