import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";

describe("blockchain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.blockchain as Program<Blockchain>;
  const user = anchor.getProvider().wallet;

  it("Is initialized!", async () => {
    const [todoPDA, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("list"), user.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        todo: todoPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const todoAccount = program.account.todo.fetch(todoPDA);

    console.log("Your account is: ", todoAccount);

    // const todosPDA = await anchor.web3.PublicKey
    console.log("Your transaction signature", tx);
  });
});
