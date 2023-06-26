import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ResizeExample } from "../target/types/resize_example";
import { assert } from "chai";

describe("resize-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ResizeExample as Program<ResizeExample>;

  var [account] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(100).accounts({
      resizeAccount: account,
    }).rpc();
    const size = await program.account.accountThing.getAccountInfo(account);
    assert.equal(size.data.length, 100);
  });

  it("Grows", async () => {
    const tx = await program.methods.resize(10249).accounts({
      resizeAccount: account
    }).rpc();
    
    const size = await program.account.accountThing.getAccountInfo(account);
    assert.equal(size.data.length, 10249);
  });
  
  it("Shrinks", async () => {
    const tx = await program.methods.resize(18).accounts({
      resizeAccount: account
    }).rpc();
    
    const size = (await program.account.accountThing.getAccountInfo(account)).data.length;
    const acc = await program.account.accountThing.fetch(account);
    
    assert.equal(size, 18);
    assert.equal(acc.myData.length, 6)
  });
});
