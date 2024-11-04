export const formatEthBalance = (balanceInHex) => {
  if (!balanceInHex) return '0 ETH'

  // Convert hex to decimal (BigInt to handle large numbers)
  const balanceInWei = BigInt(balanceInHex)

  // Convert Wei to ETH (1 ETH = 10^18 Wei)
  const ethValue = Number(balanceInWei) / 1e18

  // Format with up to 6 decimal places, removing trailing zeros
  const formatted = ethValue.toFixed(6).replace(/\.?0+$/, '')
  return `${formatted} ETH`
}
