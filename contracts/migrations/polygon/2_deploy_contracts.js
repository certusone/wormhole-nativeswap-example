const fsp = require("fs/promises");

const CrossChainSwapV2 = artifacts.require("CrossChainSwapV2");
const SwapHelper = artifacts.require("SwapHelper");

const scriptsAddressPath = "../react/src/addresses";

module.exports = async function (deployer, network) {
  const routerAddress = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff"; // quickwap
  const feeTokenAddress = "0xcf7BEE494B42cB5A902dF000158037Ad334eB4a7"; // wormUSD
  const tokenBridgeAddress = "0x377D55a7928c046E18eEbb61977e714d2a76472a";
  const wrappedMaticAddress = "0x9c3c9283d3e44854697cd22d3faa240cfb032889";

  await deployer.deploy(SwapHelper);
  await deployer.link(SwapHelper, CrossChainSwapV2);
  await deployer.deploy(CrossChainSwapV2, routerAddress, feeTokenAddress, tokenBridgeAddress, wrappedMaticAddress);

  // save the contract address somewhere
  await fsp.mkdir(scriptsAddressPath, { recursive: true });

  await fsp.writeFile(
    `${scriptsAddressPath}/${network}.ts`,
    `export const SWAP_CONTRACT_ADDRESS = '${CrossChainSwapV2.address}';`
  );

  //deployer.link(ConvertLib, MetaCoin);
  //deployer.deploy(MetaCoin);
};
