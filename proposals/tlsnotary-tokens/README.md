# **TLSNotary Tokens**

<p align="center"><img width="384" alt="onetap" src="https://github.com/user-attachments/assets/537f8d12-27e8-4b74-be35-3fda8012e7c7"></p>

## **Overview**
TLSNotary Tokens enable users to self-issue anonymous, verifiable identity tokens by leveraging their existing accounts on legacy platforms (e.g., Google, Facebook). These tokens can be used for authentication or registration on new services without compromising privacy. The protocol integrates features like garbled identities, on-chain Merkle roots, and compatibility with modern browser APIs to streamline user onboarding.
<p align="center">
<sup>Some might call it a vampire attack, and if so you can call me </sup>
    <img src="https://github.com/user-attachments/assets/384a9a0d-48b6-4015-a9b7-3f5c047467a9" alt="count von count" width="70">
</p>

---

## **Protocol Steps**

### **Step 1: Generate Garbled Identity and On-Chain Roots**
Users authenticate with a legacy platform to create a garbled version of their identity and a proof of interaction.

1. **OPRF Execution**:
   - The user and a semi-trusted intermediary participate in an **Oblivious Pseudorandom Function (OPRF)** protocol.
   - The user's account name (e.g., email or username) is converted into a **garbled identifier** that cannot be reversed or linked to the original.

2. **Batch Registration**:
   - Garbled identifiers from multiple users are collected into a **Merkle tree**.
   - The **Merkle root** of the tree is published **on-chain** as a tamper-proof, public reference point.

3. **Proof Delivery**:
   - Each user receives:
     - A **Merkle branch** proving their inclusion in the tree.
     - A **TLSNotary proof** tying their garbled identity to a verified session on the legacy platform.

```
+---------+        +-----------------+        +-----------+        +----------------+
|   User  | --OPRF--> Garbler/       | ---> Batch Users ---> Merkle Tree (Root On-Chain)
| (Legacy |        | Intermediary    |        +-----------+        +----------------+
| Account)|        | (Semi-Trusted)  |
+---------+        +-----------------+
     |                      |
     +-----> Garbled ID     +-----> Proof of Inclusion
```
---

### **Step 2: Token Issuance**
Users interact with an issuer to obtain an anonymous token.

1. Users present their **Merkle branch** and **TLSNotary proof** to the token issuer.
2. The issuer generates an anonymous, unlinkable token that:
   - Encodes the user’s garbled identity and proof.
   - Embeds private metadata (e.g., trust signals) that remain hidden from the user and verifier.

```
+---------+          +----------------+          +-------------+
|   User  | --> Present Proof & ID -->|  Issuer  | --> Generate Token -->
|         |          | (Anonymous)    |          |
+---------+          +----------------+          +-------------+
```
---

### **Step 3: Token Verification**
Users present their token to authenticate or register with a new service.

1. The verifier checks the token’s validity and ensures the user's garbled identity is included in the on-chain Merkle root.
2. The verifier optionally extracts trust signals from the token but cannot link it back to the user’s original legacy account.

```
+---------+          +-------------+          +-----------+          +---------------+
|   User  | --> Present Token -->  | Verifier | --> Verify Token -->  | On-Chain Root |
+---------+          +-------------+          +-----------+          +---------------+
```
---




## **Key Features**


1. **Enhanced Privacy**:
   - The OPRF garbles user account names, ensuring they are not directly exposed or linkable to legacy platforms.
   - The Merkle tree structure allows individual verification without disclosing batch-level details.

2. **Decentralized Verification**:
   - Publishing Merkle roots on-chain creates a decentralized, tamper-proof reference for verifying garbled identities and tokens.

3. **Scalability**:
   - Batch registration reduces the load on intermediaries and facilitates efficient token issuance for large user bases.

4. **Flexibility**:
   - New services can verify user tokens and identities without relying on centralized intermediaries, leveraging the on-chain Merkle roots.

5. **Browser Compatibility**:
   - Modern browsers already support technologies for storing and presenting anonymous or private tokens:
     - **Safari**: Implements support for **Private Access Tokens**, allowing the seamless integration of tokens into web authentication workflows. ([Apple Developer News](https://developer.apple.com/news/?id=huqjyh7k))
     - **Chrome**: Introduces **Private State Tokens** as part of the Privacy Sandbox, providing an API for tokens that integrate privacy-preserving authentication. ([Google Developer Guide](https://developers.google.com/privacy-sandbox/protections/private-state-tokens))
   - These browser capabilities ensure users can manage and present their tokens securely and efficiently, reducing the need for additional software or extensions.

6. **Developer Velocity / DX**:
   - This protocol can significantly benefit even **privacy-naive developers** as it reduces external dependencies on services like firebase / auth0 -- and increases the speed a user can login or register. Futher benefets include instant onboarding by integrating with the **Federated Credential Management (FedCM) API** ([MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FedCM_API)).
   - By using the FedCM API, developers can streamline user registration and onboarding flows:
     - **Eliminates Round Trips**: Removes the need for a round trip to an OAuth 2.0 provider for authentication.
     - **No API Keys**: Simplifies implementation by removing the dependency on API keys or client secrets.
   - FedCM integration accelerates development while ensuring users experience a seamless and private registration process.
   - Google has experimented with "speeding up onboarding" through a service called [One Tap](https://developers.google.com/identity/gsi/web/guides/display-google-one-tap) by injecting js into pages and mediating a modal to nudge users to log-in with _pre-warmed_ sessions:
   - <p align="center"><img width="384" alt="onetap" src="https://github.com/user-attachments/assets/8c06fdb6-3491-4a55-989b-55243e6533f3"></p>
   -  this requires developers to register for one tap, get an api key, and load google scripts on their page
   -  an improvement would be to wire up TLSNT to a *browser mediated modal* which means: a) its fast and b) developers dont have to design it or get it to work in any way:
   - <p align="center"><img width="314" alt="browser-mediated" src="https://github.com/user-attachments/assets/d98ef395-a042-4041-9f50-e343ed95c314"></p>



7. **User-Friendly Experience**:
   - Since browsers natively support token storage and management, users can engage in anonymous authentication with minimal technical overhead, leading to widespread adoption and ease of use.

This protocol’s compatibility with browser-native features and APIs like FedCM positions it as a practical, privacy-preserving solution for developers and users alike, offering both advanced privacy and streamlined implementation for modern web applications.

---

## **Conclusion**
By integrating privacy-preserving cryptography and modern web standards, TLSNotary Tokens empower users and developers to achieve secure, anonymous authentication and registration.




