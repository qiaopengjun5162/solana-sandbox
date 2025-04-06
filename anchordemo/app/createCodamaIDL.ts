import { createFromRoot, ProgramUpdates, updateProgramsVisitor } from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import anchorIdl from '../target/idl/anchordemo.json';
import { updateAccountsVisitor, updateInstructionsVisitor } from 'codama';

import * as fs from 'fs';

const codama = createFromRoot(rootNodeFromAnchor(anchorIdl));

// fs.writeFileSync('codamaIDL.json', codama.getJson());

console.log(codama);
console.log(codama.getJson());

// codama.update(updateAccountsVisitor({ ... }));
// codama.update(updateInstructionsVisitor({ ... }));

const map: Record<string, ProgramUpdates> = {
    "anchordemo": { name: "updatedName" }
}

codama.update(updateProgramsVisitor(map));
console.log("updated\n");
console.log(codama.getJson());
