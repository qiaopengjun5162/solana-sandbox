use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};
use mpl_token_metadata::types::DataV2;

declare_id!("6X8QM5p8NdBzBGboFXgpFevHY2VG9h3kJNdNbZca33ps");

#[program]
pub mod solana_nft_mintcraft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


// 这个派生宏可以实现对给定结构体数据的反序列化，自动生成账户等操作。
// 有了这个派生宏，在获取账户时不再需要手动迭代账户以及反序列化操作，并且实现了账户满足程序安全运行所需要的安全检查。
#[derive(Accounts)]
pub struct MintNFT<'info> {
    // signer 是 AccountInfo 类型，表示它是一个账户对象，具备对该账户的签名能力。这通常是发起交易的用户账户
    // #[account(mut, signer)]：这个属性宏包含两个修饰符：
    // mut：表示该账户在交易过程中可能会发生状态变化，例如余额、签名等。
    // signer：确保该账户拥有签名能力，是交易发起者，并能支付相关费用（如租金和账户初始化）。
    /// CHECK: The signer field is safe because it is verified elsewhere in the program.
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(init, payer = signer,mint::decimals = 0,mint::authority = signer.key(),mint::freeze_authority = signer.key(),)]
    pub mint: Account<'info, Mint>,
    // associated_token_account 是一个关键账户，用于管理 mint 代币（例如 NFT）的持有和转移。在大多数情况下，每个用户都需要一个与 mint 相关联的代币账户来存储其代币。
    #[account(init_if_needed, payer = signer, associated_token::mint = mint, associated_token::authority = signer)]
    pub associated_token_account: Account<'info, TokenAccount>,
    // metadata_account 是 AccountInfo 类型，表示一个与 NFT 元数据相关联的账户。这个账户包含了关于 NFT 的所有描述性信息，如名称、符号、URI 以及创作者等。
    // address = MetadataAccount::find_pda(&mint.key()).0：这个表达式通过 Metaplex Token Metadata Program 中的 find_pda 函数找到与当前 mint 账户关联的元数据地址。每个 NFT 都有对应的元数据账户，存储了该 NFT 的关键信息。
    // metadata_account 是用于管理 NFT 的元数据。在 NFT 铸造流程中，除了创建代币本身，通常还需要生成并存储与该代币关联的元数据信息（如名称、图像链接等）。metadata_account 负责记录这些信息。
    /// CHECK - address
    #[account(mut, address = MetadataAccount::find_pda(&mint.key()).0)]
    pub metadata_account: AccountInfo<'info>,
    // master_edition_account 主要用于管理 NFT 的“主版”，特别是当 NFT 有多个副本或版本时。它通常在发行有限版本的 NFT（如限量发行的艺术品）时使用。master_edition_account 存储该 NFT 是否为“主版”及其相关的控制信息。
    /// CHECK - address
    #[account(mut, address = MasterEdition::find_pda(&mint.key()).0)]
    pub master_edition_account: AccountInfo<'info>,

    // token_program 字段是 Program 类型，表示 Solana 上的程序，泛型使用了 Token 来指定，是一个与 SPL 代币协议直接关联的程序，它提供了用于管理代币账户、转移、铸造和销毁代币的标准化 API。。
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // token_metadata_program 负责 NFT 元数据的创建、更新和管理。当我们铸造一个 NFT 时，需要为其指定元数据，这些元数据用于描述该 NFT 的基本属性，如名称、符号和描述的 URL。Metaplex 的元数据程序允许我们为 NFT 铸造过程添加丰富的描述信息。
    pub token_metadata_program: Program<'info, Metadata>,
    // system_program 使用 Program 类型，指 Solana 的系统程序（System Program），它是 Solana 区块链的基础组成部分，提供了一系列底层功能，如账户的创建、管理和资金的转移。
    pub system_program: Program<'info, System>,
    // 在 Solana 中，所有账户都必须支付租金以维持账户的存在。如果账户的余额不足以支付租金，则该账户会被自动清除。
    // rent 变量允许开发者在创建或管理账户时检查和处理租金的相关规则。在 NFT 铸造过程中，当我们创建新账户（如 mint、token 或 metadata 账户）时，需要确保这些账户有足够的余额以免除租金。通过访问 rent 系统变量，我们可以检查账户的租金状态并确保其长期存活。
    pub rent: Sysvar<'info, Rent>,
}