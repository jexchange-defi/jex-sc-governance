# JEXchangeDefi Governance (smart contract)

JEXchangeDefi governance smart contract.

This document describes the JEXchangeDefi Improvement Proposal (JIP) process and its smart contract.

## Stakeholders

- proposer: author of the proposal
- voters: voters of the proposal
- JEXchangeDefi core team

## Scope of improvements

In order to protect the protocol in the early stages of decentralized governance, the scope of improvements is currently limited to:

- fees level (in liquidity pools, orderbook, DEX aggregator)
- fees management

## How to create a proposal

### Prerequisites

An author MUST have at least a governance power of `1,000,000` to write a proposal.

### Write the proposal

The proposal consists of a raw text document.

It MUST contain:

- a meaningful and concise subject/title
- a description of the proposed improvement(s), and for each improvement:
  - pros and cons
  - estimated cost(s)
- possible vote(s) (can be Y/N or a set a values depending on the proposal)

### Review the proposal

Proposal must be reviewed on our Discord server.

During the review process, the core team verifies if the proposal is in the [Scope of improvements](#scope-of-improvements) and contains all the [required information](#write-the-proposal).

A dedicated channel is created on our Discord server where anyone can discuss about the proposal, leading to potential modifications before submission.

Once validated, a unique identifier `JIP-xxx` (eg `JIP-1`) is associated to the proposal. This identifier MUST be added to the proposal itself (text document).

### Submit the proposal

The proposer MUST create an on-chain transaction with the following information:

- receiver: sender's address
- value: `0` (EGLD)
- data: hex-encoded proposal text document

The proposer MUST post the transaction hash on our Discord server in the channel dedicated to the proposal.

### Proposal mint

JEX core team creates the proposal in the governance smart contract.

- receiver:
  - mainnet: `erd1qqqqqqqqqqqqqpgqgkdysw3qh385aghaaky3zz0malncp8206avs5cqwel`
- endpoint: `createProposal`
- value: `0` EGLD
- arguments:
  - proposal id
  - label
  - vote start timestamp (seconds)
  - vote end timestamp (seconds)
  - number of vote choices (eg `2` for a `Yes/No` vote)
  - proposal tx hash (see [here](#submit-the-proposal))
- gas limit: `10000000` (10 million)

A NFT is automatically minted during this process. It will remain in the smart contract.

### Voting

Ongoing proposals are displayed in JEXchangeDefi application at https://app.jexchange.io or using xPortal application hub.

During a proposal voting period, voters can vote using the `vote` endpoint in the governance smart contract.

- endpoint: `vote`
- value: `0` EGLD
- arguments:
  - vote choice

Voter's governance power at the exact time of the vote will be added to the balance of the vote (choice).

Note that if a voter increases its governance power after submitting a vote, the new power is not taken into account.

### Implementation

If a proposal is accepted, JEXchangeDefi core team MUST take actions as soon as possible to implement it.

## Get started

Create a NFT collection with `canTransferNFTCreateRole` flag set to `true`.

Compile and deploy the smart contract.

Transfer `ESDTRoleNFTCreate` role to the smart contract.

```
Tx receiver: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
Gas limit: 51000000
Data:

setSpecialRole
@
JIP-df3784 in hex
@
SC address in hex
@
ESDTRoleNFTCreate in hex = 45534454526f6c654e4654437265617465

example:
setSpecialRole@4a49502d646633373834@00000000000000000500459a483a20bc4f4ea2fded891109fbefe7809d4fd759@45534454526f6c654e4654437265617465
```

Call `setAdmin` endpoint to configure admin account.
