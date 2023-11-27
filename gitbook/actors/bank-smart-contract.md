# 🏦 Bank Smart Contract

A Bank Smart Contract (BSC) is deployed by the [FSC](factory-smart-contract.md) and can never be interacted with directly. Only the FSC has allowance to call functions of a BSC, which is enforced by code. There exist no [access keys](../terminology.md#access-keys) for a BSC.

When a new partnership between two Banks is established it will be checked whether a Bank already has a deployed BSC. If not a new instance of a BSC will be deployed to a [sub-address](../terminology.md#addresses) of the FSC, where the prefix is calculated by hashing the Bank's name using the default Rust hashing algorithm.

Every Bank has its own BSC which holds all trade data, thus all data is separated. The counterparty for every trade can be recognized by the respective `counterparty` field
