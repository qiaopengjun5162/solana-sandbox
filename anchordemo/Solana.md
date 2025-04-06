# Solana



## 

生成一个以 `"AnkP"` 开头（不区分大小写）的 Solana 虚荣地址（Vanity Address）

```bash
➜ solana-keygen grind --starts-with AnkP:1 --ignore-case   

anchordemo on  master [?] via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 1m 9.2s 
➜ solana-keygen grind --starts-with AnkP:1 --ignore-case                   
Searching with 12 threads for:
        1 pubkey that starts with 'ankp' and ends with ''
Wrote keypair to AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc.json

```

### **关键区别**

|       特性       |      虚荣地址（Vanity Address）       |           程序 ID（Program ID）            |
| :--------------: | :-----------------------------------: | :----------------------------------------: |
|     **用途**     |          个人钱包、品牌营销           |              智能合约部署地址              |
|   **生成方式**   | 人为暴力生成（`solana-keygen grind`） |         随机生成（或固定系统程序）         |
| **是否可自定义** |           可自定义部分字符            | 通常不可自定义（除非是 Vanity Program ID） |
|     **示例**     |        `AnkP9...`（人为生成）         |      `Tokenkeg...`（随机或系统固定）       |

- **普通 Solana 地址** = 随机生成（钱包或程序 ID）。
- **虚荣地址** = 人为生成特定模式的地址（可用于钱包或程序 ID）。
- **程序 ID** 默认是随机的，但可以通过 `solana-keygen grind` 生成 **Vanity Program ID**。





### 构建项目

```bash
anchordemo on  master [?] via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base 
➜ anchor build
   Compiling anchordemo v0.1.0 (/Users/qiaopengjun/Code/solana-code/2025/SolanaSandbox/anchordemo/programs/anchordemo)
    Finished `release` profile [optimized] target(s) in 1.69s
   Compiling anchordemo v0.1.0 (/Users/qiaopengjun/Code/solana-code/2025/SolanaSandbox/anchordemo/programs/anchordemo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.55s
     Running unittests src/lib.rs (/Users/qiaopengjun/Code/solana-code/2025/SolanaSandbox/anchordemo/target/debug/deps/anchordemo-3e6164bcfacb8d99)

```





```bash
anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base 
➜ npx tsx createCodamaIDL.ts
Need to install the following packages:
tsx@4.19.3
Ok to proceed? (y) y

{
  accept: [Function: accept],
  clone: [Function: clone],
  getJson: [Function: getJson],
  getRoot: [Function: getRoot],
  update: [Function: update]
}

anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 13.0s 
➜ npx tsx createCodamaIDL.ts
{
  accept: [Function: accept],
  clone: [Function: clone],
  getJson: [Function: getJson],
  getRoot: [Function: getRoot],
  update: [Function: update]
}
{"kind":"rootNode","standard":"codama","version":"1.2.11","program":{"kind":"programNode","name":"anchordemo","publicKey":"AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc","version":"0.1.0","origin":"anchor","docs":[],"accounts":[{"kind":"accountNode","name":"demoDataAccount","docs":[],"data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"2e53e38584aeacb6","encoding":"base16"}},{"kind":"structFieldTypeNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"structFieldTypeNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}},{"kind":"structFieldTypeNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}}]},"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"instructions":[{"kind":"instructionNode","name":"initialize","docs":[],"optionalAccountStrategy":"programId","accounts":[{"kind":"instructionAccountNode","name":"dataAccount","isWritable":true,"isSigner":true,"isOptional":false,"docs":[]},{"kind":"instructionAccountNode","name":"authority","isWritable":true,"isSigner":true,"isOptional":false,"docs":[],"defaultValue":{"kind":"identityValueNode"}},{"kind":"instructionAccountNode","name":"systemProgram","isWritable":false,"isSigner":false,"isOptional":false,"docs":[],"defaultValue":{"kind":"publicKeyValueNode","publicKey":"11111111111111111111111111111111","identifier":"systemProgram"}}],"arguments":[{"kind":"instructionArgumentNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"afaf6d1f0d989bed","encoding":"base16"}},{"kind":"instructionArgumentNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"instructionArgumentNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}},{"kind":"instructionArgumentNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}}],"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"definedTypes":[],"pdas":[],"errors":[]},"additionalPrograms":[]}

```





### 更新

```ts
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

```





```bash
anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 2.2s 
➜ npx tsx createCodamaIDL.ts
{
  accept: [Function: accept],
  clone: [Function: clone],
  getJson: [Function: getJson],
  getRoot: [Function: getRoot],
  update: [Function: update]
}
{"kind":"rootNode","standard":"codama","version":"1.2.11","program":{"kind":"programNode","name":"anchordemo","publicKey":"AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc","version":"0.1.0","origin":"anchor","docs":[],"accounts":[{"kind":"accountNode","name":"demoDataAccount","docs":[],"data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"2e53e38584aeacb6","encoding":"base16"}},{"kind":"structFieldTypeNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"structFieldTypeNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}},{"kind":"structFieldTypeNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}}]},"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"instructions":[{"kind":"instructionNode","name":"initialize","docs":[],"optionalAccountStrategy":"programId","accounts":[{"kind":"instructionAccountNode","name":"dataAccount","isWritable":true,"isSigner":true,"isOptional":false,"docs":[]},{"kind":"instructionAccountNode","name":"authority","isWritable":true,"isSigner":true,"isOptional":false,"docs":[],"defaultValue":{"kind":"identityValueNode"}},{"kind":"instructionAccountNode","name":"systemProgram","isWritable":false,"isSigner":false,"isOptional":false,"docs":[],"defaultValue":{"kind":"publicKeyValueNode","publicKey":"11111111111111111111111111111111","identifier":"systemProgram"}}],"arguments":[{"kind":"instructionArgumentNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"afaf6d1f0d989bed","encoding":"base16"}},{"kind":"instructionArgumentNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"instructionArgumentNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}},{"kind":"instructionArgumentNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}}],"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"definedTypes":[],"pdas":[],"errors":[]},"additionalPrograms":[]}
updated

{"kind":"rootNode","standard":"codama","version":"1.2.11","program":{"kind":"programNode","name":"updatedName","publicKey":"AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc","version":"0.1.0","origin":"anchor","docs":[],"accounts":[{"kind":"accountNode","name":"demoDataAccount","docs":[],"data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"2e53e38584aeacb6","encoding":"base16"}},{"kind":"structFieldTypeNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"structFieldTypeNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}},{"kind":"structFieldTypeNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}}]},"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"instructions":[{"kind":"instructionNode","name":"initialize","docs":[],"optionalAccountStrategy":"programId","accounts":[{"kind":"instructionAccountNode","name":"dataAccount","isWritable":true,"isSigner":true,"isOptional":false,"docs":[]},{"kind":"instructionAccountNode","name":"authority","isWritable":true,"isSigner":true,"isOptional":false,"docs":[],"defaultValue":{"kind":"identityValueNode"}},{"kind":"instructionAccountNode","name":"systemProgram","isWritable":false,"isSigner":false,"isOptional":false,"docs":[],"defaultValue":{"kind":"publicKeyValueNode","publicKey":"11111111111111111111111111111111","identifier":"systemProgram"}}],"arguments":[{"kind":"instructionArgumentNode","name":"discriminator","defaultValueStrategy":"omitted","docs":[],"type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},"defaultValue":{"kind":"bytesValueNode","data":"afaf6d1f0d989bed","encoding":"base16"}},{"kind":"instructionArgumentNode","name":"number","docs":[],"type":{"kind":"numberTypeNode","format":"u64","endian":"le"}},{"kind":"instructionArgumentNode","name":"text","docs":[],"type":{"kind":"sizePrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"}}},{"kind":"instructionArgumentNode","name":"optionalKey","docs":[],"type":{"kind":"optionTypeNode","fixed":false,"item":{"kind":"publicKeyTypeNode"},"prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"}}}],"discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}],"definedTypes":[],"pdas":[],"errors":[]},"additionalPrograms":[]}

```





```bash
import { renderJavaScriptVisitor, renderRustVisitor } from '@codama/renderers';
import { createFromRoot, ProgramUpdates, updateProgramsVisitor } from 'codama';
import codamaIDL from "./codamaIDL.json";

const codama = createFromRoot(codamaIDL);

console.log(codama.getRoot());


// codama.accept(renderJavaScriptVisitor('clients/js/src/generated', { ... }));
// codama.accept(renderRustVisitor('clients/rust/src/generated', { ... }));

```





```bash
anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 2.8s 
➜ pnpm i @codama/renderers-js      
Already up to date
Progress: resolved 61, reused 61, downloaded 0, added 0, done

dependencies:
+ @codama/renderers-js 1.2.10

Done in 6s using pnpm v10.4.1

anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 6.3s 
➜ pnpm i@codama/renderers                       
 ERR_PNPM_RECURSIVE_EXEC_FIRST_FAIL  Command "i@codama/renderers" not found

anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base 
➜ pnpm i @codama/renderers   
Already up to date
Progress: resolved 61, reused 61, downloaded 0, added 0, done

dependencies:
+ @codama/renderers 1.0.19

Done in 2.9s using pnpm v10.4.1

anchordemo/app on  master [?] is 📦 1.0.0 via ⬢ v22.1.0 via 🦀 1.86.0 via 🅒 base took 3.1s 
➜ npx tsx generateCode.ts   
{
  kind: 'rootNode',
  standard: 'codama',
  version: '1.2.11',
  program: {
    kind: 'programNode',
    name: 'anchordemo',
    publicKey: 'AnkpTFgp1wzTCZHU7kxQTsit4zQZuqpY4cDzgS5bQnCc',
    version: '0.1.0',
    origin: 'anchor',
    docs: [],
    accounts: [ [Object] ],
    instructions: [ [Object] ],
    definedTypes: [],
    pdas: [],
    errors: []
  },
  additionalPrograms: []
}
```





## 总结







## 参考

- https://github.com/codama-idl/codama
- https://github.com/solana-foundation/gill