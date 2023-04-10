import {
  Environment,
  requestAirdrops,
} from "@formfunction-hq/formfunction-program-shared";
import { Wallet as AnchorWallet } from "@project-serum/anchor";
import { Connection } from "@solana/web3.js";
import { DEVNET_AIRDROP_SIZE_SOL } from "scripts/constants";
import getWalletKeypairForScript from "scripts/utils/getWalletKeypairForScript";
import FormfnCandyMachineSdk from "sdk/FormfnCandyMachineSdk";
import getRpcFromEnvironmentForTest from "tests/utils/getRpcFromEnvironmentForTest";

export default async function getCandyMachineSdkForScript(
  environment: Environment = Environment.Development
) {
  const connection = new Connection(
    getRpcFromEnvironmentForTest(environment),
    "confirmed"
  );
  const walletKeypair = getWalletKeypairForScript(environment);
  const wallet = new AnchorWallet(walletKeypair);

  if (environment !== Environment.Production) {
    if (environment === Environment.Local) {
      await requestAirdrops({
        amountInSol: 1000,
        connection,
        environment,
        wallets: [walletKeypair],
      });
    } else {
      await requestAirdrops({
        amountInSol: DEVNET_AIRDROP_SIZE_SOL,
        connection,
        environment,
        wallets: [walletKeypair],
      });
    }
  }

  const candyMachineSdk = new FormfnCandyMachineSdk({
    connection,
    environment,
    wallet,
  });
  return { candyMachineSdk, connection, environment, wallet: walletKeypair };
}
