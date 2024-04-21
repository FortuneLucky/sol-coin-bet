# Contest_Platorm

## What is Memecoin Contest?

Memecoin Contest (name may change) is a simple platform for memecoin contests on the Solana blockchain. 

A memecoin contest is defined by the following parameters:
2 meme tokens that are “competing,” incl. relevant liquidity pool/contract addresses for the tokens
An end date (e.g. March 15, 00:00:00 UTC)
A platform wallet and platform fee percentage

Suppose a contest has been defined and initialized - say the tokens are token Alicecoin and token Bobcoin.  Users may navigate to our website and interact with a smart contract (“contest contract”) that allows them to deposit either Alicecoins or Bobcoins as a participant.  On this website they can also see the amount and value of Alicecoins or Bobcoins deposited so far. Once they deposit tokens, these tokens are locked until the end date. 

At the end date, the smart contract (or a human) looks up prices of Alicecoin and Bobcoin from the relevant liquidity pools and determines the values of the Alicecoins and Bobcoins in the contest contract.  

To be clear,  Value(Alicecoins) = Price_Alicecoin * Quantity_Alicecoin_in_Contest_Contract

If Value(Alicecoins) > Value(Bobcoins), Alicecoin side wins. 
If Value(Bobcoins) > Value (Alicecoins), Bobcoin side wins. 

Then, a redemption phase begins:

A 2% fee of ALL tokens in the contest contract gets sent to the platform wallet. 

If Alicecoin side wins, Alicecoin depositors may withdraw their Alicecoins from the smart contract. They also get an allocated fraction (more detail later) of all Bobcoins deposited. Bobcoin depositors lose: they get nothing..

(Conversely, if Bobcoin side wins, Bobcoin depositors may withdraw their Bobcoins from the smart contract. They also get an allocated fraction of all Alicecoins deposited, as rewards. Alicecoin depositors lose: they get nothing.)

## Contest Lifecycle
### Initialization: 

- Memecoin contest is defined with meme tokens, end date, maximum single deposit, and platform wallet as parameters. 
- Contest contract is activated 

### Contest Period:

- Website visitors/Users deposit tokens into the contest contract. Typically that means they will deposit on one side of the contract - either Alicecoin or Bobcoin. Each single deposit transaction is capped by the maximum single deposit parameter. 
Once a user deposits his tokens, he is assigned a “bet coefficient” defined as 

- bet coefficient :=  sol_value_of_deposit * (1+ opposite-side_value in contest contract) / (1+ deposit_side_value in contest contract)

- Webpage will display the live value of all Alicecoins and Bobcoins in the contest contract. It will do this by looking up the price from the relevant liquidity pool and will use a SOL:USD converter.

### Redemption:

- Redemption phase is triggered when end date is reached
- 2% of all tokens in contest contract are sent to platform wallet
- Winning side is able to withdraw deposit side tokens they deposited (exactly the amount they deposited minus the 2% fee). They also can withdraw, as rewards, a number of opposite-side-tokens proportional to their bet coefficient.
