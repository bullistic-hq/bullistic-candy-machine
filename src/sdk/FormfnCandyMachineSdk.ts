import {
  AnchorWallet,
  Environment,
  ixsToTx,
  Maybe,
} from "@formfunction-hq/formfunction-program-shared";
import { AnchorProvider, BN, Idl, Program } from "@project-serum/anchor";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import dayjs from "dayjs";
import { CandyMachineProgram, FORMFN_CANDY_MACHINE_IDL } from "sdk/idl";
import { FormfnCandyMachine } from "sdk/idl/FormfnCandyMachine";
import candyMachineMintNftIx from "sdk/instructions/candyMachineMintNftIx";
import candyMachineSetCollectionDuringMintIx from "sdk/instructions/candyMachineSetCollectionDuringMintIx";
import findCandyMachineCollectionPda from "sdk/pdas/findCandyMachineCollectionPda";
import findCandyMachineCreatorPda from "sdk/pdas/findCandyMachineCreatorPda";
import BuyerWithAllowlistProofData from "sdk/types/BuyerWithAllowlistProofData";
import CandyMachineAccount from "sdk/types/candy-machine/CandyMachineAccount";
import MintPhase from "sdk/types/MintPhase";
import getMintPhase from "sdk/utils/getMintPhase";
import getProgramIdsFromEnvironment from "sdk/utils/getProgramIdsFromEnvironment";

export default class FormfnCandyMachineSdk {
  private _connection: Connection;
  private _idl: Idl = FORMFN_CANDY_MACHINE_IDL;
  private _program: CandyMachineProgram;
  private _botSignerAuthority: PublicKey;
  private _candyMachineProgramId: PublicKey;

  constructor({
    connection,
    environment,
    wallet,
  }: {
    connection: Connection;
    environment: Environment;
    wallet: AnchorWallet;
  }) {
    this._connection = connection;

    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "recent",
    });

    const programIds = getProgramIdsFromEnvironment(environment);

    this._botSignerAuthority = programIds.botSignerAuthority;
    this._candyMachineProgramId = programIds.candyMachineProgramId;

    this._program = new Program<FormfnCandyMachine>(
      FORMFN_CANDY_MACHINE_IDL,
      this.candyMachineProgramId,
      provider
    );
  }

  get connection() {
    return this._connection;
  }

  get idl() {
    return this._idl;
  }

  get program() {
    return this._program;
  }

  get botSignerAuthority() {
    return this._botSignerAuthority;
  }

  get candyMachineProgramId() {
    return this._candyMachineProgramId;
  }

  async fetchCandyMachine(
    candyMachine: PublicKey
  ): Promise<CandyMachineAccount> {
    return this.program.account.candyMachine.fetch(
      candyMachine
    ) as unknown as CandyMachineAccount;
  }

  async fetchCandyMachineCollectionPda(candyMachine: PublicKey) {
    const [collectionPda] = findCandyMachineCollectionPda(
      candyMachine,
      this.candyMachineProgramId
    );
    return this.program.account.collectionPda.fetch(collectionPda);
  }

  async findCandyMachineCreatorPda(candyMachine: PublicKey) {
    return findCandyMachineCreatorPda(candyMachine, this.candyMachineProgramId);
  }

  async getExpectedMintPrice(candyMachine: PublicKey): Promise<BN> {
    const candyMachineState = await this.fetchCandyMachine(candyMachine);
    const {
      allowlistPrice,
      allowlistSaleStartTime,
      premintPrice,
      price,
      publicSaleEndTime,
      publicSaleStartTime,
    } = candyMachineState.data;
    const mintPhase = getMintPhase({
      allowlistSaleStartTimeUnix:
        allowlistSaleStartTime == null
          ? null
          : dayjs.unix(allowlistSaleStartTime.toNumber()),
      publicSaleEndTimeUnix: dayjs.unix(publicSaleEndTime.toNumber()),
      publicSaleStartTimeUnix: dayjs.unix(publicSaleStartTime.toNumber()),
    });
    switch (mintPhase) {
      case MintPhase.Premint:
        return premintPrice ?? price;
      case MintPhase.Allowlist:
        return allowlistPrice ?? price;
      case MintPhase.Public:
      case MintPhase.Expired:
        return price;
    }
  }

  async mintNft(
    {
      buyer,
      buyerAllowlistTokenAccount,
      candyMachine,
      mint,
    }: {
      buyer: PublicKey;
      buyerAllowlistTokenAccount: Maybe<PublicKey>;
      candyMachine: PublicKey;
      mint: PublicKey;
    },
    {
      buyerWithAllowlistProofData,
    }: {
      buyerWithAllowlistProofData: Maybe<BuyerWithAllowlistProofData>;
    }
  ): Promise<Transaction> {
    const expectedPrice = await this.getExpectedMintPrice(candyMachine);
    const mintNftIx = await candyMachineMintNftIx(
      {
        botSignerAuthority: this.botSignerAuthority,
        buyer,
        buyerAllowlistTokenAccount,
        candyMachine,
        mint,
      },
      {
        buyerWithAllowlistProofData,
        expectedPrice,
        program: this.program,
      }
    );
    const setCollectionDuringMintIx =
      await candyMachineSetCollectionDuringMintIx(
        {
          buyer,
          candyMachine,
          mint,
        },
        {
          program: this.program,
        }
      );

    return ixsToTx([mintNftIx, setCollectionDuringMintIx]);
  }
}
