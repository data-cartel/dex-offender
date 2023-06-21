use async_trait::async_trait;
use ctf::ethernaut::lvl18_magic_number::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    /**
     * @title Ethernaut Level 18: Magic Number
     *
     * To solve this level, you only need to provide the
     * Ethernaut with a `Solver`, a contract that
     * responds to `whatIsTheMeaningOfLife()` with the
     * right number.
     *
     * Easy right?
     * Well... there's a catch.
     * The solver's code needs to be really tiny. Really
     * reaaaaaallly tiny. Like freakin' really
     * really itty-bitty tiny: 10 opcodes at most.
     *
     * Hint: Perhaps its time to leave the comfort of
     * the Solidity compiler momentarily, and build this
     * one by hand O_o.
     *
     * That's right: Raw EVM bytecode.
     * Good luck!
     */
    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        println!("\n\nDeploying the contract...");
        let tx = offender
            .send_transaction(
                TransactionRequest::new().from(offender.address()).data(vec![
                    ////////////////////////////
                    //////   DEPLOYMENT   //////
                    ////////////////////////////
                    0x60, 0x80, // PUSH1 0x80
                    0x60, 0x40, // PUSH1 0x40
                    0x52, // MSTORE
                    0x34, // CALLVALUE
                    0x80, // DUP1
                    0x15, // ISZERO
                    0x60, 0x0f, // PUSH1 0xf
                    0x57, // JUMPI
                    0x60, 0x00, // PUSH1 0x0
                    0x80, // DUP1
                    0xfd, // REVERT
                    0x5b, // JUMPDEST
                    0x50, // POP
                    0x60, 0xa,  // PUSH1 0xa
                    0x80, // DUP1
                    0x60, 0x1d, // PUSH1 0x1d
                    0x60, 0x00, // PUSH1 0x0
                    0x39, // CODECOPY
                    0x60, 0x00, // PUSH1 0x0
                    0xf3, // RETURN
                    0xfe, // INVALID
                    ////////////////////////////
                    //////     RUNTIME    //////
                    ////////////////////////////
                    0x60, 0x2a, // PUSH1 0x2a
                    0x60, 0x0,  // PUSH1 0x0
                    0x52, // MSTORE
                    0x60, 0x20, // PUSH1 0x1f
                    0x60, 0x00, // PUSH1 0x0
                    0xf3, // RETURN
                          ////////////////////////////
                          //////    THE END    ///////
                          ////////////////////////////
                ]),
                None,
            )
            .await?
            .await?;

        let address = tx.unwrap().contract_address.unwrap();

        println!(
            "Contract address: {}",
            ethers::utils::to_checksum(&address, None)
        );

        println!("Setting the solver address...\n\n");
        let target = MagicNum::new(target.address, offender.to_owned());
        target.set_solver(address).send().await?.await?;

        Ok(())
    }
}
