const fs = require('fs');

function idlTypeToTsType(idlType) {
    if (typeof idlType === 'string') {
        switch (idlType) {
            case 'string':
                return 'string';
            case 'i64':
                return 'bigint';
            // Add other basic type conversions here
            default:
                return 'any';
        }
    } else if (idlType && idlType.kind === 'array') {
        return `${idlTypeToTsType(idlType.type)}[]`;
    } else if (idlType && idlType.kind === 'option') {
        return `${idlTypeToTsType(idlType.type)} | null`;
    } else if (idlType && idlType.defined) {
        return idlType.defined;
    }
    return 'any';
}

function convertIDLtoTs(idlJson) {
    if (!idlJson.instructions || !Array.isArray(idlJson.instructions)) {
        console.error('Invalid or missing instructions array in IDL JSON');
        return '';
    }

    let tsCode = `export interface ${idlJson.name} {\n`;
    tsCode += `  version: '${idlJson.version}';\n`;
    tsCode += `  name: '${idlJson.name}';\n`;
    tsCode += `  instructions: [\n`;

    idlJson.instructions.forEach(instr => {
        tsCode += `    {\n`;
        tsCode += `      name: '${instr.name}';\n`;
        tsCode += `      docs: [${(instr.docs || []).map(doc => `'${doc}'`).join(', ')}];\n`;
        tsCode += `      accounts: [\n`;

        (instr.accounts || []).forEach(acc => {
            tsCode += `        {\n`;
            tsCode += `          name: '${acc.name}';\n`;
            tsCode += `          isMut: ${acc.isMut};\n`;
            tsCode += `          isSigner: ${acc.isSigner};\n`;
            if (acc.docs) {
                tsCode += `          docs: [${acc.docs.map(doc => `'${doc}'`).join(', ')}];\n`;
            }
            if (acc.isOptional) {
                tsCode += `          isOptional: ${acc.isOptional};\n`;
            }
            tsCode += `        },\n`;
        });

        tsCode += `      ];\n`;
        tsCode += `      args: [\n`;

        (instr.args || []).forEach(arg => {
            tsCode += `        {\n`;
            tsCode += `          name: '${arg.name}';\n`;
            tsCode += `          type: '${idlTypeToTsType(arg.type)}';\n`;
            tsCode += `        },\n`;
        });

        tsCode += `      ];\n`;
        tsCode += `    },\n`;
    });

    tsCode += `  ];\n`;
    tsCode += `}\n`;

    return tsCode;
}

const idlPath = 'openbook_v2.json'; // Replace with your IDL file path
try {
    const idlJson = JSON.parse(fs.readFileSync(idlPath, 'utf8'));
    const tsCode = convertIDLtoTs(idlJson);
    fs.writeFileSync('output.ts', tsCode);
    console.log('TypeScript interface generated successfully.');
} catch (error) {
    console.error('Error reading or processing IDL file:', error);
}
