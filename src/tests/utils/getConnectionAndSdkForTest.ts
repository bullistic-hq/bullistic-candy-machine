import {
  Environment,
  requestAirdrops,
} from "@bullistic-hq/bullistic-program-shared";
import { Wallet as AnchorWallet } from "@project-serum/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import BullisticCandyMachineSdk from "sdk/BullisticCandyMachineSdk";
import getRpcFromEnvironmentForTest from "tests/utils/getRpcFromEnvironmentForTest";

export default async function getConnectionAndSdkForTest(
  environment: Environment
): Promise<{
  candyMachineSdk: BullisticCandyMachineSdk;
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

  const candyMachineSdk = new BullisticCandyMachineSdk({
    connection,
    environment,
    wallet,
  });
  return { candyMachineSdk, connection, environment, wallet: walletKeypair };
}
