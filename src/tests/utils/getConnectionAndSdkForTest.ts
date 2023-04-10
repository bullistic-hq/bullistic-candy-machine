import {
  Environment,
  requestAirdrops,
} from "@formfunction-hq/formfunction-program-shared";
import { Wallet as AnchorWallet } from "@project-serum/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import FormfnCandyMachineSdk from "sdk/FormfnCandyMachineSdk";
import getRpcFromEnvironmentForTest from "tests/utils/getRpcFromEnvironmentForTest";

export default async function getConnectionAndSdkForTest(
  environment: Environment
): Promise<{
  candyMachineSdk: FormfnCandyMachineSdk;
  connection: Connection;
  environment: Environment;
  wallet: Keypair;
}> {
  const walletKeypair = Keypair.generate();
  const wallet = new AnchorWallet(walletKeypair);
  const connection = new Connection(
    getRpcFromEnvironmentForTest(environment),
    "processed" // This is the same as what Anchor uses.
  );

  await requestAirdrops({ connection, environment, wallets: [walletKeypair] });

  const candyMachineSdk = new FormfnCandyMachineSdk({
    connection,
    environment,
    wallet,
  });
  return { candyMachineSdk, connection, environment, wallet: walletKeypair };
}
