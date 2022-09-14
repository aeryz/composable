import {
  Box,
  useTheme,
  Button,
} from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useState } from "react";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useStakedPositions } from "@/store/stakingRewards/stakingRewards.slice";

export const PoolUnstakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  
  const theme = useTheme();
  const poolDetails = useLiquidityPoolDetails(poolId);
  const { pool } = poolDetails;
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const positions = useStakedPositions(pool?.lpToken ?? "-");
  console.log(positions);

  const handleUnStake = () => {
    // TODO: handle stake here
  }

  return (
    <Box {...boxProps}>
      <Box>
        <BigNumberInput
          maxValue={poolDetails.lpBalance}
          setValid={setValid}
          noBorder
          value={amount}
          setValue={setAmount}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => setAmount(poolDetails.lpBalance),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount to Unstake",
            TypographyProps: {color: "text.secondary"},
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${poolDetails.lpBalance} ${poolDetails.baseAsset?.symbol}/${poolDetails.quoteAsset?.symbol}`,
              BalanceTypographyProps: {color: "text.secondary"},
            },
          }}
          EndAdornmentAssetProps={{
            assets: 
            poolDetails.baseAsset && poolDetails.quoteAsset ? 
            [
              {icon: poolDetails.baseAsset.icon, label: poolDetails.baseAsset.symbol},
              {icon: poolDetails.quoteAsset.icon, label: poolDetails.quoteAsset.symbol},
            ] : [],
            separator: "/",
          }}
        />
      </Box>
      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleUnStake}
          disabled={!valid}
        >
          {`Unstake ${poolDetails.baseAsset?.symbol}/${poolDetails.quoteAsset?.symbol}`}
        </Button>
      </Box>
    </Box>
  );
};

