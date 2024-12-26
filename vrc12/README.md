# Contract Overview: BTG Locking Period
The BTG Locking Period contract provides a secure and reliable way to lock BTG for a specified open_token and a specified period, 
ensuring they can only be accessed by the rightful owner after the designated time has elapsed. 
This contract leverages Openverse's capabilities and follows best practices to ensure robustness and security.
## Key Features
* Lock BTG: Users can lock a specific amount of BTG and set an end time for when these BTG can be unlocked.
* Mint Verification: The contract verifies that the mint account provided is either from open_token or open_token_2022, ensuring compatibility with the correct token standards.
* Ownership Check: Ensures that only the original owner can unlock the BTG, adding a layer of security.*
* Time Validation: Checks that the unlock request is not made before the end time, enforcing the locking period.