import { Connection } from "@solana/web3.js";

export default async function assertBotTaxedTransaction(
  connection: Connection,
  txid: string,
  additionalErrorMessage?: string
) {
  const tx = await connection.getParsedTransaction(txid, "confirmed");

  expect(
    tx?.meta?.logMessages?.some(
      (log) =>
        log.includes("BotTaxCollected") &&
        log.includes("error_code_number: 8052")
    )
  ).toEqual(true);

  if (additionalErrorMessage != null) {
    expect(
      tx?.meta?.logMessages?.some((log) => log.includes(additionalErrorMessage))
    ).toEqual(true);
  }

  console.log(`Asserted ${txid} was bot taxed.`);
}
