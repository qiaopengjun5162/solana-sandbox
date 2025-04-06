import { renderJavaScriptVisitor, renderRustVisitor } from '@codama/renderers';
import { createFromRoot, ProgramUpdates, updateProgramsVisitor } from 'codama';
import codamaIDL from "./codamaIDL.json";

const codama = createFromRoot(codamaIDL);

// console.log(codama.getRoot());


codama.accept(renderJavaScriptVisitor('generated/ts', {}));
// codama.accept(renderRustVisitor('clients/rust/src/generated', { ... }));
