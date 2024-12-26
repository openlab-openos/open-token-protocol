# Contract Overview: BTG Locking Period
The BTG Locking Period contract is designed to lock a specified amount of tokens for a defined period, preventing unauthorized access until the designated unlock time has passed. This contract ensures that tokens can only be withdrawn by the original owner after the expiration of the locking period.

## Key Features
* Lock Tokens: Users can lock a specific amount of tokens and set an end time for when these tokens can be unlocked.

* Mint Verification: The contract verifies that the mint account provided is either from open_token or open_token_2022, ensuring compatibility with the correct token standards.
* Ownership Check: Ensures that only the original owner can unlock the tokens, adding a layer of security.*
* Time Validation: Checks that the unlock request is not made before the end time, enforcing the locking period.