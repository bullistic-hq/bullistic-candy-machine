/* eslint-disable sort-keys-fix/sort-keys-fix */
import {
  jsonStringify,
  Maybe,
} from "@formfunction-hq/formfunction-program-shared";
import BN from "bn.js";
import parsePrintCandyMachineStateArgs from "scripts/utils/env/parsePrintCandyMachineStateArgs";
import getCandyMachineSdkForScript from "scripts/utils/getCandyMachineSdkForScript";

function convertTimestamp(timeString: Maybe<BN>) {
  if (timeString == null) {
    return null;
  }

  return new Date(timeString.toNumber() * 1000);
}

function convertBn(number: Maybe<BN>) {
  return number?.toNumber();
}

async function printCandyMachineState() {
  const { candyMachine, environment } = parsePrintCandyMachineStateArgs();
  console.log(
    `\nFetching CandyMachineState for candy machine address: ${candyMachine}, environment = ${environment}`
  );

  const { candyMachineSdk } = await getCandyMachineSdkForScript(environment);
  const state = await candyMachineSdk.fetchCandyMachine(candyMachine);
  const collectionPda = await candyMachineSdk.fetchCandyMachineCollectionPda(
    candyMachine
  );

  const { data } = state;
  const candyMachineAccountState = jsonStringify({
    ...state,
    itemsRedeemed: convertBn(state.itemsRedeemed),
    data: {
      ...data,
      itemsAvailable: convertBn(data.itemsAvailable),
      maxSupply: convertBn(data.maxSupply),
      price: convertBn(data.price),
      premintPrice: convertBn(data.premintPrice),
      allowlistPrice: convertBn(data.allowlistPrice),
      allowlistSaleStartTime: convertTimestamp(data.allowlistSaleStartTime),
      publicSaleEndTime: convertTimestamp(data.publicSaleEndTime),
      publicSaleStartTime: convertTimestamp(data.publicSaleStartTime),
      merkleAllowlistRootList: `${data.merkleAllowlistRootList}`,
    },
  });

  console.log("\nCandy Machine account:\n");
  console.log(candyMachineAccountState);
  console.log(`\nCollection PDA mint: ${collectionPda.mint.toString()}\n`);
}

printCandyMachineState();
