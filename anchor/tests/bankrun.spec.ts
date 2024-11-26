import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import IDL from "../target/idl/vesting.json"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { BN, Program } from "@coral-xyz/anchor"
import { Vesting }  from "../target/types/vesting"
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { Buffer } from "buffer";
import { createMint, mintTo } from "spl-token-bankrun";

describe("Vesting Smart Contract Tests", () => {

const companyName = 'THE HATE CLUB';

let beneficiary: Keypair;
let context: ProgramTestContext;
let provider: BankrunProvider;
let program: Program<Vesting>;
let banksClient: BanksClient;
let employer: Keypair;
let mint: PublicKey;
let beneficiaryProvider: BankrunProvider;
let program2: Program<Vesting>;
let vestingAccountKey: PublicKey;
let coldTokenAccount: PublicKey;
let employeeAccount: PublicKey;


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

    banksClient = context.banksClient;

    employer = provider.wallet.payer;

    // @ts-ignore
    mint = await createMint(banksClient, employer, employer.publicKey, null, 2);

    beneficiaryProvider = new BankrunProvider(context);
    beneficiaryProvider.wallet = new NodeWallet(beneficiary);

    program2 = new Program<Vesting>(IDL as Vesting, provider);

    [vestingAccountKey] = PublicKey.findProgramAddressSync(
        [Buffer.from(companyName)],
        program.programId
    );

    [coldTokenAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("vesting_cold"), Buffer.from(companyName)],
        program.programId
    );

    [employeeAccount] = PublicKey.findProgramAddressSync([
        Buffer.from("employee_vesting"),
        beneficiary.publicKey.toBuffer(),
        vestingAccountKey.toBuffer()
        ],
        program.programId
    );
  });

  it("Should Create A Vesting Account", async() => {
    const tx = await program.methods.createVestingAccount(companyName).accounts({
        signer: employer.publicKey,
        mint,
        tokenProgram: TOKEN_PROGRAM_ID
    }).rpc({ commitment: 'confirmed' });

    const vestingAccountData = await program.account.vestingAccount.fetch(vestingAccountKey, 'confirmed');

    console.log("Create Vesting Account: ", tx);
    console.log("Vesting Account Data: ", vestingAccountData, null, 2);
  });

  it("Should Fund The Cold Token Account", async() => {
    const amount = 10_000 * 10 ** 9;
    // @ts-ignore
    const mintTx = await mintTo(banksClient, employer, mint, coldTokenAccount, employer, amount);

    console.log('Mint Cold Token Account: ', mintTx);
  });

  it("Should Create Employee Vesting Account", async() => {
    const tx2 = await program.methods.createEmployeeAccount(
        new BN(0),
        new BN(100),
        new BN(100),
        new BN(0)
    ).accounts({
        beneficiary: beneficiary.publicKey,
        vestingAccount: vestingAccountKey
    }).rpc({ commitment: 'confirmed', skipPreflight: true });

    console.log("Create Employee Account Transaction: ", tx2);
    console.log("Employee Account: ", employeeAccount.toBase58());
  })
})