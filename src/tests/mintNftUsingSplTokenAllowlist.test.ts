import {
  Environment,
  requestAirdrops,
} from "@formfunction-hq/formfunction-program-shared";
import { Keypair } from "@solana/web3.js";
import assertBotTaxedTransaction from "tests/utils/assertBotTaxedTransaction";
import getConnectionAndSdkForTest from "tests/utils/getConnectionAndSdkForTest";
import mintNftForTest from "tests/utils/mintNftForTest";
import parseCandyMachinePubkeyForTest from "tests/utils/parseCandyMachinePubkeyForTest";
import parseSplAllowlistForTest from "tests/utils/parseSplAllowlistForTest";
import invariant from "tiny-invariant";

describe("FormfnCandyMachineSdk minting using SPL token allowlist.", () => {
  test("Minting an NFT from a candy machine with an SPL token allowlist works.", async () => {
    const allowlist = parseSplAllowlistForTest();
    const { keypairs: allowlistKeypairs } = allowlist;
    invariant(
      allowlistKeypairs != null,
      "allowlist keypairs are required for test."
    );
    const candyMachine = parseCandyMachinePubkeyForTest();

    const { candyMachineSdk, connection, environment, wallet } =
      await getConnectionAndSdkForTest(Environment.Local);

    await requestAirdrops({
      connection,
      environment,
      wallets: [wallet, ...allowlistKeypairs],
    });

    const fetchCandyMachineState = async () => {
      return candyMachineSdk.program.account.candyMachine.fetch(
        candyMachine,
        "confirmed"
      );
    };

    const candyMachineState = await candyMachineSdk.fetchCandyMachine(
      candyMachine
    );
    let currentItemsRedeemed = candyMachineState.itemsRedeemed.toNumber();
    const initialItemsRedeemed = currentItemsRedeemed;

    const maxNftCountToMint = 25;

    // First check that a buyer without the SPL token gets bot taxed and can't mint.
    const nonAllowlistedKeypair = Keypair.generate();
    await requestAirdrops({
      connection,
      environment,
      wallets: [nonAllowlistedKeypair],
    });
    const { txid } = await mintNftForTest({
      allowlistType: "SPL",
      buyer: nonAllowlistedKeypair,
      buyerWithAllowlistProofData: null,
      candyMachine,
      candyMachineSdk,
      candyMachineState,
      connection,
    });
    await assertBotTaxedTransaction(
      connection,
      txid,
      "No SPL allowlist token present"
    );
    const candyMachineStateUnchanged = await fetchCandyMachineState();
    expect(candyMachineStateUnchanged.itemsRedeemed.toNumber()).toBe(0);

    // Then check all the allowlisted buyers can mint.
    for (const buyerKeypair of allowlistKeypairs) {
      await requestAirdrops({
        connection,
        environment,
        wallets: [buyerKeypair],
      });

      await mintNftForTest({
        allowlistType: "SPL",
        buyer: buyerKeypair,
        buyerWithAllowlistProofData: null,
        candyMachine,
        candyMachineSdk,
        candyMachineState,
        connection,
      });

      const candyMachineStateAfter = await fetchCandyMachineState();

      const itemsAvailable =
        candyMachineStateAfter.data.itemsAvailable.toNumber();
      const itemsRedeemed = candyMachineStateAfter.itemsRedeemed.toNumber();

      if (itemsAvailable - itemsRedeemed === 0) {
        console.log("All candy machine NFTs have been minted, ending test.");
        return;
      }

      const totalRedeemedInTest = itemsRedeemed - initialItemsRedeemed;
      if (totalRedeemedInTest === maxNftCountToMint) {
        console.log(
          `Reached max NFT count to mint in test of ${maxNftCountToMint}, ending test.`
        );
        return;
      }

      currentItemsRedeemed += 1;

      expect(itemsRedeemed).toBe(currentItemsRedeemed);
    }
  });
});
