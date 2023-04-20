#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod lease {
    
    #[ink(storage)]
    pub struct Lease {
        landlord: AccountId,
        tenant: AccountId,
        rent_amount: Balance,
        deposit_amount: Balance,
        lease_duration: u64, //in months 
        lease_start: Timestamp,
        lease_end: Timestamp, 
        lease_violated: bool,
    }

    // Emitted when rent is paid.
    #[ink(event)]
    pub struct RentPaid {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Lease {
        #[ink(constructor)]
        pub fn new(
            landlord: AccountId,
            tenant: AccountId,
            rent_amount: Balance,
            deposit_amount: Balance,
            lease_duration: Timestamp
        ) -> Self {
            let lease_start = Self::env().block_timestamp();
            let lease_duration_secs = lease_duration * 60 * 60 * 24 * 30;
            let lease_end = lease_start + lease_duration_secs; 

            Self{
                landlord,
                tenant,
                rent_amount,
                deposit_amount,
                lease_duration,
                lease_start,
                lease_end,
                lease_violated: false, // initialize to false
            }
        }

        #[ink(message, payable)]
        pub fn pay_rent(&mut self) -> bool {
            // Checks caller is tenant 
            assert_eq!(self.env().caller(), self.tenant);
            // Checks amount of tokens transferred is equal to rent 
            assert_eq!(self.env().transferred_value(), self.rent_amount);
            // Checks current timestamp is after ending day 
            if self.env().block_timestamp() > self.lease_end {
                self.lease_violated = true; // if rent is not paid in time
            } else {
                self.env().emit_event(RentPaid {
                from: self.tenant,
                amount: self.rent_amount,
                });
            }

            true
        }

        #[ink(message)]
        pub fn release_deposit(&mut self) -> bool {
            // Checks caller is landlord 
            assert_eq!(self.env().caller(), self.landlord);
            // Checks current timestamp is greater than ending day 
            assert!(self.env().block_timestamp() > self.lease_end);

            if !self.lease_violated {
                self.env().transfer(self.tenant, self.deposit_amount).unwrap();
            } else {
            // Transfers the deposit tokens from the contract to the landlord's account
                self.env().transfer(self.landlord, self.deposit_amount).unwrap();
            }
            
            true
        }

    }
}
