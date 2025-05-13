import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor"
import { Day } from "../target/types/day";
import { assert } from "chai";

describe("day", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day as Program<Day>;

  it("Input test", async () => {
    // Add your test here.
    try {
      const tx = await program.methods.limitRange(new anchor.BN(9)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;
      const errMsg =
        "a is too small";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }

    try {
      const tx = await program.methods.limitRange(new anchor.BN(101)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;
      const errMsg =
        "a is too big";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  });
});

