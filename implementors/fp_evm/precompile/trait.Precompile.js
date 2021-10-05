(function() {var implementors = {};
implementors["crowdloan_rewards_precompiles"] = [{"text":"impl&lt;Runtime&gt; Precompile for <a class=\"struct\" href=\"crowdloan_rewards_precompiles/struct.CrowdloanRewardsWrapper.html\" title=\"struct crowdloan_rewards_precompiles::CrowdloanRewardsWrapper\">CrowdloanRewardsWrapper</a>&lt;Runtime&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"crowdloan_rewards_precompiles/type.BalanceOf.html\" title=\"type crowdloan_rewards_precompiles::BalanceOf\">BalanceOf</a>&lt;Runtime&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;U256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Call&lt;Runtime&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["crowdloan_rewards_precompiles::CrowdloanRewardsWrapper"]}];
implementors["pallet_democracy_precompiles"] = [{"text":"impl&lt;Runtime&gt; Precompile for <a class=\"struct\" href=\"pallet_democracy_precompiles/struct.DemocracyWrapper.html\" title=\"struct pallet_democracy_precompiles::DemocracyWrapper\">DemocracyWrapper</a>&lt;Runtime&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Runtime as Config&gt;::Currency as Currency&lt;&lt;Runtime as Config&gt;::AccountId&gt;&gt;::Balance: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;U256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"precompile_utils/data/trait.EvmData.html\" title=\"trait precompile_utils::data::EvmData\">EvmData</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;DemocracyCall&lt;Runtime&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;H256&gt;,&nbsp;</span>","synthetic":false,"types":["pallet_democracy_precompiles::DemocracyWrapper"]}];
implementors["pallet_evm_precompile_assets_erc20"] = [{"text":"impl&lt;Runtime, Instance&gt; Precompile for <a class=\"struct\" href=\"pallet_evm_precompile_assets_erc20/struct.Erc20AssetsPrecompile.html\" title=\"struct pallet_evm_precompile_assets_erc20::Erc20AssetsPrecompile\">Erc20AssetsPrecompile</a>&lt;Runtime, Instance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Instance: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config&lt;Instance&gt; + Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Call&lt;Runtime, Instance&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"pallet_evm_precompile_assets_erc20/type.BalanceOf.html\" title=\"type pallet_evm_precompile_assets_erc20::BalanceOf\">BalanceOf</a>&lt;Runtime, Instance&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;U256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;U256&gt; + <a class=\"trait\" href=\"precompile_utils/data/trait.EvmData.html\" title=\"trait precompile_utils::data::EvmData\">EvmData</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: <a class=\"trait\" href=\"pallet_evm_precompile_assets_erc20/trait.AccountIdAssetIdConversion.html\" title=\"trait pallet_evm_precompile_assets_erc20::AccountIdAssetIdConversion\">AccountIdAssetIdConversion</a>&lt;Runtime::AccountId, <a class=\"type\" href=\"pallet_evm_precompile_assets_erc20/type.AssetIdOf.html\" title=\"type pallet_evm_precompile_assets_erc20::AssetIdOf\">AssetIdOf</a>&lt;Runtime, Instance&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Runtime as Config&gt;::Call as Dispatchable&gt;::Origin: OriginTrait,&nbsp;</span>","synthetic":false,"types":["pallet_evm_precompile_assets_erc20::Erc20AssetsPrecompile"]}];
implementors["pallet_evm_precompile_balances_erc20"] = [{"text":"impl&lt;Runtime, Metadata, Instance&gt; Precompile for <a class=\"struct\" href=\"pallet_evm_precompile_balances_erc20/struct.Erc20BalancesPrecompile.html\" title=\"struct pallet_evm_precompile_balances_erc20::Erc20BalancesPrecompile\">Erc20BalancesPrecompile</a>&lt;Runtime, Metadata, Instance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Metadata: <a class=\"trait\" href=\"pallet_evm_precompile_balances_erc20/trait.Erc20Metadata.html\" title=\"trait pallet_evm_precompile_balances_erc20::Erc20Metadata\">Erc20Metadata</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Instance: <a class=\"trait\" href=\"pallet_evm_precompile_balances_erc20/trait.InstanceToPrefix.html\" title=\"trait pallet_evm_precompile_balances_erc20::InstanceToPrefix\">InstanceToPrefix</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config&lt;Instance&gt; + Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Call&lt;Runtime, Instance&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"pallet_evm_precompile_balances_erc20/type.BalanceOf.html\" title=\"type pallet_evm_precompile_balances_erc20::BalanceOf\">BalanceOf</a>&lt;Runtime, Instance&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;U256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;U256&gt;,&nbsp;</span>","synthetic":false,"types":["pallet_evm_precompile_balances_erc20::Erc20BalancesPrecompile"]}];
implementors["parachain_staking_precompiles"] = [{"text":"impl&lt;Runtime&gt; Precompile for <a class=\"struct\" href=\"parachain_staking_precompiles/struct.ParachainStakingWrapper.html\" title=\"struct parachain_staking_precompiles::ParachainStakingWrapper\">ParachainStakingWrapper</a>&lt;Runtime&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: <a class=\"trait\" href=\"parachain_staking/pallet/trait.Config.html\" title=\"trait parachain_staking::pallet::Config\">Config</a> + Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Runtime as <a class=\"trait\" href=\"parachain_staking/pallet/trait.Config.html\" title=\"trait parachain_staking::pallet::Config\">Config</a>&gt;::<a class=\"type\" href=\"parachain_staking/pallet/trait.Config.html#associatedtype.Currency\" title=\"type parachain_staking::pallet::Config::Currency\">Currency</a> as Currency&lt;&lt;Runtime as Config&gt;::AccountId&gt;&gt;::Balance: <a class=\"trait\" href=\"precompile_utils/data/trait.EvmData.html\" title=\"trait precompile_utils::data::EvmData\">EvmData</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"parachain_staking/pallet/enum.Call.html\" title=\"enum parachain_staking::pallet::Call\">Call</a>&lt;Runtime&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["parachain_staking_precompiles::ParachainStakingWrapper"]}];
implementors["xtokens_precompiles"] = [{"text":"impl&lt;Runtime&gt; Precompile for <a class=\"struct\" href=\"xtokens_precompiles/struct.XtokensWrapper.html\" title=\"struct xtokens_precompiles::XtokensWrapper\">XtokensWrapper</a>&lt;Runtime&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: Dispatchable&lt;PostInfo = PostDispatchInfo&gt; + GetDispatchInfo,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime::Call: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Call&lt;Runtime&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Runtime::Call as Dispatchable&gt;::Origin: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Runtime::AccountId&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"xtokens_precompiles/type.XBalanceOf.html\" title=\"type xtokens_precompiles::XBalanceOf\">XBalanceOf</a>&lt;Runtime&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;U256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;U256&gt; + <a class=\"trait\" href=\"precompile_utils/data/trait.EvmData.html\" title=\"trait precompile_utils::data::EvmData\">EvmData</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: <a class=\"trait\" href=\"xtokens_precompiles/trait.AccountIdToCurrencyId.html\" title=\"trait xtokens_precompiles::AccountIdToCurrencyId\">AccountIdToCurrencyId</a>&lt;Runtime::AccountId, <a class=\"type\" href=\"xtokens_precompiles/type.CurrencyIdOf.html\" title=\"type xtokens_precompiles::CurrencyIdOf\">CurrencyIdOf</a>&lt;Runtime&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["xtokens_precompiles::XtokensWrapper"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()