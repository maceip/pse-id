**Registration Proposal** based on the sinu-lev docs

---

# **Registration Protocol**

## **Actors**
1. **User** (\( U \)): Wants to register their verified social media account name (\( A \)).
2. **Server** (\( S \)): Operates the OPRF and maintains a Merkle tree of registered users.

---

## **Protocol Steps**

### **1. Social Media Verification**
- The user (\( U \)) proves ownership of their social media account name (\( A \)) by performing a verification task, such as:
  - Posting a unique server-provided token (\( T \)) publicly on their account.

---

### **2. OPRF Interaction**
1. **Blinding**:
   - \( U \) generates a random blinding factor \( r \) and computes the blinded input:
     \[
     B = r \cdot A
     \]
     where \( A \) is represented as a Ristretto group element.

2. \( U \) sends \( B \) to the server (\( S \)).

3. **Server OPRF Computation**:
   - \( S \) applies its secret key \( k \) to compute the blinded OPRF result:
     \[
     F_k(B) = k \cdot B
     \]
   - \( S \) sends \( F_k(B) \) back to \( U \).

4. **Unblinding**:
   - \( U \) computes the unblinded OPRF output:
     \[
     F_k(A) = \frac{F_k(B)}{r} = k \cdot A
     \]

---

### **3. Merkle Tree Inclusion**
1. **Hashing**:
   - \( U \) hashes the OPRF output to create a leaf node for the Merkle tree:
     \[
     H(A) = H(F_k(A))
     \]
     where \( H \) is a cryptographic hash function (e.g., SHA-256).

2. **Sending to Server**:
   - \( U \) sends \( H(A) \) to \( S \).

3. **Merkle Tree Construction**:
   - \( S \) includes \( H(A) \) as a leaf in its Merkle tree and computes the tree root.

4. **Proof of Inclusion**:
   - \( S \) provides \( U \) with a Merkle proof showing the inclusion of \( H(A) \) in the tree.

---

## **Formal Representation**

### **Blinding**
\[
B = r \cdot A
\]

### **Server OPRF Computation**
\[
F_k(B) = k \cdot B
\]

### **Unblinding**
\[
F_k(A) = \frac{F_k(B)}{r} = k \cdot A
\]

### **Hashing**
\[
H(A) = H(F_k(A))
\]

---

## **Security and Privacy Guarantees**
- **Privacy**:
  - The server (\( S \)) cannot learn the raw account name (\( A \)) due to the blinding factor.
- **Verifiability**:
  - Users (\( U \)) can verify their inclusion in the Merkle tree using the proof provided by the server.
- **Scalability**:
  - The protocol supports batch processing for efficient registration of multiple users.

---

Let me know if you’d like further refinements or additional sections!
