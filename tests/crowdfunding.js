const anchor = require("@project-serum/anchor");
const { Connection, PublicKey, clusterApiUrl } = require("@solana/web3.js");
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfunding;

  const [campaigns] = await PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode("CAMPAIGN"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  const tx = await program.rpc.createAccount({
    accounts: {
      campaigns,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
  await program.rpc.createCampaign("Sanskar", "Solana dev", {
    accounts: {
      campaigns,
      user: provider.wallet.publicKey,
    },
  });
  await program.rpc.createCampaign("Akash", "Eth dev", {
    accounts: {
      campaigns,
      user: provider.wallet.publicKey,
    },
  });
  console.log("your tx:", tx);
  console.log("Created a new campaign w/ address:", campaigns.toString());
  let account = await program.account.campaigns.fetch(campaigns);
  console.log(account);
};
const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
