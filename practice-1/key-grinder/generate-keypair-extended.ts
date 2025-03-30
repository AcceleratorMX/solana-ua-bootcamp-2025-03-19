import { Keypair } from "@solana/web3.js";
import fs from "fs";

const prefix = /^alex/i;

let keypair: Keypair;
let attempts = 0;

const start = Date.now();

do {
    keypair = Keypair.generate();
    attempts++;
} while (!prefix.test(keypair.publicKey.toBase58()));

const stop = Date.now();
const elapsedTime = formatTime(stop - start);

const myKeypair = {
    publicKey: keypair.publicKey.toBase58(),
    secretKey: Array.from(keypair.secretKey),
    attempts,
    elapsedTime
};

fs.writeFileSync("my-keypair.json", JSON.stringify(myKeypair, null, 2), "utf-8");

console.log(`The public key is: ${keypair.publicKey.toBase58()}`);
console.log(`The private key is: ${keypair.secretKey}`);
console.log(`Attempts: ${attempts}`);
console.log(`Elapsed time: ${elapsedTime}`);

function formatTime(milliseconds: number): string {
    return new Intl.DateTimeFormat("uk-UA", {
        hour: "numeric",
        minute: "numeric",
        second: "numeric",
        hour12: false,
        timeZone: "UTC"
    }).format(milliseconds);
};