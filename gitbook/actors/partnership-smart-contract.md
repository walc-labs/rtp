# 🤝 Partnership Smart Contract

A Partnership Smart Contract (PSC) is deployed by the [FSC](factory-smart-contract.md) and can never be interacted with directly. Only the FSC has allowance to call functions of a PSC, which is enforced by code. There exist no [access keys](../terminology.md#access-keys) for a PSC.

When a new partnership between two Banks is established a new instance of a PSC will be deployed to a [sub-address](../terminology.md#addresses) of the FSC, where the prefix is calculated by hashing a tuple of Bank A and Bank B's name using the default Rust hashing algorithm.

Every partnership has its own PSC, thus all data is separated.
