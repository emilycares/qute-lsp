import * as vscode from 'vscode';
import * as assert from 'assert';
import { getDocUri, activate } from './helper';

suite('Should get diagnostics', () => {
	console.log("sdfsdsf");
	
	const docUri = getDocUri('diagnostics.txt');

	test('Diagnoses uppercase texts', async () => {
		await testDefinition(docUri, 
			{} as unknown as vscode.Definition
		);
	});
});

function toRange(sLine: number, sChar: number, eLine: number, eChar: number) {
	const start = new vscode.Position(sLine, sChar);
	const end = new vscode.Position(eLine, eChar);
	return new vscode.Range(start, end);
}

async function testDefinition(docUri: vscode.Uri, expectedDefinition: vscode.Definition) {

	await activate(docUri);

	const position = new vscode.Position(0, 0);
	const actualDefinition = await vscode.commands.executeCommand("editor.action.goToDefinition", docUri, position);

	assert.equal(actualDefinition, expectedDefinition);
}
