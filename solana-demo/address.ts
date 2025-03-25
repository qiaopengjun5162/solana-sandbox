import {
    generateKeyPairSigner, generateKeyPair, getBase58Encoder, address,
    getAddressFromPublicKey, createKeyPairSignerFromBytes, isAddress, createKeyPairFromPrivateKeyBytes
} from "@solana/web3.js";
import bs58 from 'bs58';
import * as bip39 from "bip39";
import { derivePath, getPublicKey } from 'ed25519-hd-key'


export function createSolAddress(seedHex: string, addressIndex: number): string {
    const path = `m/44'/501'/0'/${addressIndex}'`;

    const { key } = derivePath(path, seedHex);
    //

    const publicKey = getPublicKey(key, false).toString("hex")
    const buffer = Buffer.from(getPublicKey(key, false).toString("hex"), "hex");
    const address = bs58.encode(buffer)
    const hdWallet = {
        privateKey: key.toString('hex') + publicKey,
        publicKey,
        address
    }
    return JSON.stringify(hdWallet)
}

export function verifySolAddress(address: string): boolean {
    return /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(address);
}


export function pubKeyToAddress(publicKey: string): string {
    if (publicKey.length !== 64) {
        throw new Error("public key length Invalid");
    }
    const buffer = Buffer.from(publicKey, "hex");
    return bs58.encode(buffer);
}



// https://solana.com/zh/developers/cookbook/wallets/check-publickey
export const isSolanaAddress = (address: string): boolean => {
    return isAddress(address);
}

// https://solana.com/developers/cookbook/wallets/verify-keypair
// @ts-ignore
export async function privateKeyToAddress(secretKey: string): Promise<string> {
    // const bufferPriv = Buffer.from(privateKey, "hex");
    const sourceKeypair = await createKeyPairSignerFromBytes(
        getBase58Encoder().encode(secretKey)
    );
    // const keypair = Keypair.fromSecretKey(bufferPriv);
    return sourceKeypair.address;
    // return keypair.publicKey.toBase58();
}
