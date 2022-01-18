import os from "os";
const fs = require('fs');
import { Keypair } from "@solana/web3.js";

export const loadKeypairFromFile = (filePath: string): Keypair => {
    return Keypair.fromSecretKey(
        new Uint8Array(
            JSON.parse(
                fs.readFileSync(filePath.replace(/^~/, os.homedir())).toString()
            )
        )
    );
};

export const hexToBytes = (hex: string): any[] => {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}

export const bytesToHex = (bytes: number[]): string => {
    for (var hex = [], i = 0; i < bytes.length; i++) {
        var current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
        hex.push((current >>> 4).toString(16));
        hex.push((current & 0xF).toString(16));
    }
    return hex.join("");
}