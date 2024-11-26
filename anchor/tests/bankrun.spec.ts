import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
import { ProgramTestContext, startAnchor } from "solana-bankrun";
import IDL from "../target/idl/vesting.json"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { Program } from "@coral-xyz/anchor"
import { Vesting }  from "../target/types/vesting"

describe("Vesting Smart Contract Tests", () => {

let beneficiary: Keypair;
let context: ProgramTestContext;
let provider: BankrunProvider;
let program: Program<Vesting>;

  beforeAll(async() => {
    beneficiary = new anchor.web3.Keypair();

    context = await startAnchor('', [
        { name: "vesting" , programId: new PublicKey(IDL.address) }
    ],
    [
        {
            address: beneficiary.publicKey,
            info: {
                lamports: 1_000_000_000,
                data: Buffer.alloc(0),
                owner: SYSTEM_PROGRAM_ID,
                executable: false
            }
        }
    ]
    );

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    program = new Program<Vesting>(IDL as Vesting, provider);
  })  
})