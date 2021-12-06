## Wallet Seed Backups

By adhering to the 3-2-1 rule we can guarantee the security of private keys for the organizational and custodial wallets. To store the seed phrase we provide least 3 copies of the data, over at least two types of media, with at least 1 copy stored off site.

#### Encrypted on local disk

The first way of storing the keys will be the most readily used as these will be stored on the computer with NEAR-CLI installed. By encrypting the data with AES-256, a cryptographically secure algorithm we are able to ensure that should the system become compromised the keys are still secure.

#### Encrypted on cloud storage

The second way of storing keys will be a copy of the files stored on Nextcloud. This provides a backup which is stored in a different location, and via a different media which is easily recoverable. It is important that this is encrypted BEFORE being uploaded so that the plaintext keys are never exposed to the internet.

#### Physical copy stored in a safe place

The final copy of the data will be stored off-site on paper. The exact manner of this backup is yet to be determined, as it is incredibly flexible and decisions surrounding balancing ease of access and security need to be made however the basic structure is as follows.

* Keys split between multiple locations to provide security/redundancy
* Utilize RAID-like technology to ensure that if a single location is compromised the entire key is still secure
* Can store on paper for ease of creation, or steel for water/fire resistance

###### Example Solution

* Seed phrase engraved on steel plates
* Split between 3 locations - Anvil House, Joff's House, Finn's House,
* Each split key contains 12 words however only 8 of the words are correct. This provides a dummy key should one be compromised, and only requires 2 of the plates for the seed to be recovered.
  * Anvil House - N mod 3 == 0 && N mod 3 == 1
  * Joff's House - N mod 3 == 1 && N mod 3 == 2
  * Finn's House - N mod 3 == 0 && N mod 3 == 2

#### Air gapped system

Ideally the system in which the keys are stored on is also air gapped while not in use. This could be as simple as unplugging the ethernet cable on the computer, or storing the keys on a USB flash drive or external HDD. This greatly reduces the risk for one of the primary attack vectors in which the system could be compromised.

## Mitigating Human Risk

No matter how secure a system there will always be weak points, and the human factor is often the greatest of these and the most difficult to mitigate as often it directly hinders ease of access.

#### Education of the system

One of the primary risks for the system is human error, due to NEAR and blockchain being budding technology it is unlikely the users have extensive experience with said systems, therefore education surrounding the system becomes incredibly important. This is not only for reducing the risk of human error such as not sending test transactions before a large transaction, but by clearly defining expectations for using the system people will be better equipped to spot suspicious activity such as phishing attacks.

#### Security Minded Culture

Some of the most common security risks such as phishing and social engineering are able to be mitigated by cybersecurity awareness training but these programs cost time and resources. Often a simpler approach is to have open conversations surrounding security and a culture which promotes raising security concerns.

#### Access rights

Reducing the amount of people with access to the keys we can reduce the attack vector on the system for human based attacks.

#### Audit Logs

Utilizing audit logs we can ensure correct procedure is being followed surrounding the usage of the system, such as proper procedure surrounding encryption.

#### Value Limits on Wallets

Limiting the value of wallets can reduce the potential damage of a security breach and even deter threats such as insider attacks.

###### Example Solution

Cold Wallet - $10000 Limit

Hot Wallet - $100 Limit

Custodial Wallet - $10000 Limit

## Encryption/Decryption Process

#### Encryption

1. Open a [terminal](https://www.lifewire.com/ways-to-open-a-terminal-console-window-using-ubuntu-4075024) window, then change to the directory which contains credentials with the command:

   ```
   cd ~/.near-credentials/testnet
   ```
2. Encrypt the file with the command:

   ```
   gpg -c --no-symkey-cache isparx.testnet.json
   ```
3. When prompted, type and verify an encryption password.
4. This creates a file named isparx.testnet.json.gpg. That new file cannot be opened without decryption.
5. Delete the original unencrypted file **AFTER** ensuring that the file can be decrypted.

   ```
   rm isparx.testnet.json
   ```

#### Decryption

1. Decrypt the .gpg file with the command:

   ```
   gpg isparx.testnet.json.gpg
   ```
2. You are prompted for the passphrase you created during the encryption. Type that passphrase to decrypt the file.

### NEAR-CLI