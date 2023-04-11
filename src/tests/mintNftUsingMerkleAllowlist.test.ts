import {
  Environment,
  requestAirdrops,
} from "@bullistic-hq/bullistic-program-shared";
import BuyerWithAllowlistProofData from "sdk/types/BuyerWithAllowlistProofData";
import getConnectionAndSdkForTest from "tests/utils/getConnectionAndSdkForTest";
import keypairObjectToKeypair from "tests/utils/keypairObjectToKeypair";
import mintNftForTest from "tests/utils/mintNftForTest";
import parseMerkleAllowlistForTest from "tests/utils/parseMerkleAllowlistForTest";

describe("BullisticCandyMachineSdk minting using merkle allowlist.", () => {
  test("Minting an NFT from a candy machine with a merkle allowlist proof works.", async () => {
    const allowlist = parseMerkleAllowlistForTest();
    const { merkleAllowlistData } = allowlist;
    const candyMachineKeypair = keypairObjectToKeypair(
      allowlist.candyMachineKeypair
    );
    const candyMachine = candyMachineKeypair.publicKey;

    const { candyMachineSdk, connection, environment, wallet } =
      await getConnectionAndSdkForTest(Environment.Local);

    await requestAirdrops({ connection, environment, wallets: [wallet] });

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

    // Avoid running forever if the allowlist and itemsAvailable are huge.
    const maxNftCountToMint = 25;

    let buyerIndex = 0;
    for (let i = 0; i < merkleAllowlistData.length; i++) {
      const buyerForTest = merkleAllowlistData[i].buyers[buyerIndex];
      const buyerKeypair = keypairObjectToKeypair(buyerForTest.keypairObject);
      const buyerWithAllowlistProofData: BuyerWithAllowlistProofData = {
        amount: buyerForTest.amount,
        rootIndexForProof: buyerForTest.merkleTreeIndex,
        serializedProof: buyerForTest.serializedProof,
      };

      await requestAirdrops({
        connection,
        environment,
        wallets: [buyerKeypair],
      });
      await mintNftForTest({
        allowlistType: "merkle",
        buyer: buyerKeypair,
        buyerWithAllowlistProofData,
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

      if (i === merkleAllowlistData.length - 1) {
        buyerIndex++;
        i = 0;
      }
    }
  });
});
