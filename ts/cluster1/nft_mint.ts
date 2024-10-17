import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";
import dotenv from 'dotenv';
dotenv.config();


// Create a devnet connection
const umi = createUmi(`https://devnet.helius-rpc.com/?api-key=${process.env.Devnet_Key}`);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

const name = "Ral Andrew Turbin3 Rug";
const symbol = "RAT3R";
const uri = "https://devnet.irys.xyz/75Xk97knvxvBsxfMqruW1E8jPr3qJjpPSAkTKcTytvW9";
const sellerFeeBasisPoints = percentAmount(0, 2);

(async () => {
    let tx = createNft(
        umi,
        {
            mint, name, symbol, uri, sellerFeeBasisPoints
        }
    )
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();