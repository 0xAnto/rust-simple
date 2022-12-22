const { providers, utils } = require("ethers");

// create a provider instance
const provider = new providers.JsonRpcProvider(
  `https://polygon-mainnet.g.alchemy.com/v2/${your_key}`
);

// the transaction hash
const transactionHash =
  "0x871f9cdfc61f01609bc2a8b588295fa7c119cdc0b28518af48d32a2984da383d";

// get the transaction receipt
provider.getTransactionReceipt(transactionHash).then((receipt) => {
  console.log("Data", parseInt(receipt.logs[8].data));
});
