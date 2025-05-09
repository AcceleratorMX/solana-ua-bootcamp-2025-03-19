import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import {
  airdropIfRequired,
  getCustomErrorMessage,
} from "@solana-developers/helpers";
import { expect, describe, test } from "@jest/globals";
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes our favorites to the blockchain", async () => {
    // Add your test here.
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favorites instruction handler
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
          // Note that both `favorites` and `system_program` are added
          // automatically.
        })
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] =
      web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
      );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());
  }, 30000);

  it("Updates existing favorites on the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Initial favorite values
    const initialNumber = new anchor.BN(42);
    const initialColor = "blue";

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] =
      web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
      );

    // Set the initial values
    try {
      const tx = await program.methods
        .setFavorites(initialNumber, initialColor)
        .accounts({
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
      console.log(`Initial set tx signature: ${tx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify initial values
    let dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(initialColor);
    expect(dataFromPda.number.toNumber()).toEqual(initialNumber.toNumber());

    // Now update only the number
    const updatedNumber = new anchor.BN(99);
    try {
      const updateTx = await program.methods
        .updateFavorites(updatedNumber, null)
        .accounts({
          user: user.publicKey,
          originalOwner: user.publicKey,
        })
        .signers([user])
        .rpc();
      console.log(`Update number tx signature: ${updateTx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify that only the number was updated
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    // Color should remain unchanged
    expect(dataFromPda.color).toEqual(initialColor);
    // Number should be updated
    expect(dataFromPda.number.toNumber()).toEqual(updatedNumber.toNumber());

    // Update only the color
    const updatedColor = "green";
    try {
      const updateTx = await program.methods
        .updateFavorites(null, updatedColor)
        .accounts({
          user: user.publicKey,
          originalOwner: user.publicKey,
        })
        .signers([user])
        .rpc();
      console.log(`Update color tx signature: ${updateTx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify that only the color was updated
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    // Color should be updated
    expect(dataFromPda.color).toEqual(updatedColor);
    // Number should remain unchanged
    expect(dataFromPda.number.toNumber()).toEqual(updatedNumber.toNumber());

    // Finally, update both values
    const finalNumber = new anchor.BN(7);
    const finalColor = "purple";
    try {
      const updateTx = await program.methods
        .updateFavorites(finalNumber, finalColor)
        .accounts({
          user: user.publicKey,
          originalOwner: user.publicKey,
        })
        .signers([user])
        .rpc();
      console.log(`Update both values tx signature: ${updateTx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify that both values were updated
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(finalColor);
    expect(dataFromPda.number.toNumber()).toEqual(finalNumber.toNumber());
  }, 30000);

  it("Sets a delegate and allows them to update favorites", async () => {
    // Generate keypairs for owner and delegate
    const owner = web3.Keypair.generate();
    const delegate = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`Owner public key: ${owner.publicKey}`);
    console.log(`Delegate public key: ${delegate.publicKey}`);

    // Airdrop SOL to both owner
    await airdropIfRequired(
      anchor.getProvider().connection,
      owner.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Airdrop SOL to the delegate
    await airdropIfRequired(
      anchor.getProvider().connection,
      delegate.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Initial favorite values
    const initialNumber = new anchor.BN(42);
    const initialColor = "blue";

    // Calculate the PDA account address for the favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), owner.publicKey.toBuffer()],
      program.programId
    );

    // Set initial favorites with a delegate
    try {
      const tx = await program.methods
        .setFavoritesWithDelegate(initialNumber, initialColor, delegate.publicKey)
        .accounts({
          user: owner.publicKey,
        })
        .signers([owner])
        .rpc();
      console.log(`Set favorites with delegate tx: ${tx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify initial favorites and delegate
    let dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(initialColor);
    expect(dataFromPda.number.toNumber()).toEqual(initialNumber.toNumber());
    expect(dataFromPda.delegate.toString()).toEqual(delegate.publicKey.toString());

    // Update favorites using the delegate
    const updatedNumber = new anchor.BN(99);
    const updatedColor = "green";
    try {
      const updateTx = await program.methods
        .updateFavorites(updatedNumber, updatedColor)
        .accounts({
          user: delegate.publicKey,
          originalOwner: owner.publicKey,
        })
        .signers([delegate])
        .rpc();
      console.log(`Delegate update tx: ${updateTx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify that favorites were updated by the delegate
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(updatedColor);
    expect(dataFromPda.number.toNumber()).toEqual(updatedNumber.toNumber());
    // Remove the delegate
    try {
      const removeTx = await program.methods
        .setAuthority(null)
        .accounts({
          user: owner.publicKey,
        })
        .signers([owner])
        .rpc();
      console.log(`Remove delegate tx: ${removeTx}`);
    } catch (error) {
      const rawError = error as Error;
      throw new Error(
        getCustomErrorMessage(systemProgramErrors, rawError.message)
      );
    }

    // Verify that the delegate was removed
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.delegate).toBeNull();
  }, 30000);
});
