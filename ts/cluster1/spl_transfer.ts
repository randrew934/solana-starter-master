import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "../wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("GMVa74KENMjVPUh9ikrFRqH9QydXCS8a2PpSSp6mSvjD");

// Recipient address
const to = new PublicKey("D6s47BoD2dkWKifBW7jHEvcHnNw7TtM1hRJX48AVPGXD");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it

    const fromATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Your ata is: ${fromATA.address.toBase58()}`);

    // Get the token account of the toWallet address, and if it does not exist, create it

    const toATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    console.log(`Your ata is: ${toATA.address.toBase58()}`);

    // Transfer the new token to the "toTokenAccount" we just created

    const transferTX = await transfer(
      connection,
      keypair,
      fromATA.address,
      toATA.address,
      keypair.publicKey,
      10000
    );
    console.log(`Your transfer txid: ${transferTX}`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
