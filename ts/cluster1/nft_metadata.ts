import wallet from "../wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import dotenv from 'dotenv';
dotenv.config();


// Create a devnet connection
const umi = createUmi(`https://devnet.helius-rpc.com/?api-key=${process.env.Devnet_Key}`);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

    const image =
      "https://devnet.irys.xyz/GF8ctY3mt5aXKv9yJbYpfmzv9eFCRhLQzAqusDMbEgPE";
    const metadata = {
      name: "Ral Andrew Turbin3 Rug",
      symbol: "RAT3R",
      description: "Crazy Rug",
      image: image,
      attributes: [
        { trait_type: "Something Nice", value: "Cool Pink" },
        { trait_type: "Something Not Nice", value: "Cool Green" },
        { trait_type: "Something Just Nice", value: "Cool Blue" },
        { trait_type: "Something Not Just Nice", value: "Cool Purple" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "image-uri",
          },
        ],
      },
      creators: [
        {
          address: keypair.publicKey,
          share: 100,
        },
      ],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
